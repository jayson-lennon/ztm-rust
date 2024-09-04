use std::{
    fs::OpenOptions,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use error_stack::{Report, Result, ResultExt};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{
    error::Suggestion,
    feature::backend::{tracker::TimeTrackerError, EndTime, StartTime, TimeRecord, Tracker},
};

use super::load_records;

/// An error that may occur while using the flat file tracker.
#[derive(Debug, thiserror::Error)]
#[error("filesystem tracker error")]
pub struct FlatFileTrackerError;

/// A time tracker that uses a flat-file database.
#[derive(Debug)]
pub struct FlatFileTracker {
    /// Where all the tracking records are stored.
    pub(in crate::feature::backend::flatfile) records: PathBuf,

    /// Path to use when tracking is active.
    lockfile: PathBuf,
}

impl FlatFileTracker {
    /// Create a new flat file tracker.
    pub fn new<R, L>(records: R, lockfile: L) -> Self
    where
        R: Into<PathBuf>,
        L: Into<PathBuf>,
    {
        let records = records.into();
        let lockfile = lockfile.into();
        Self { records, lockfile }
    }
}

/// The data stored in the lockfile.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct LockfileData {
    start_time: StartTime,
}

/// The data stored in the records file.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct RecordsData {
    records: Vec<TimeRecord>,
}

impl Tracker for FlatFileTracker {
    #[instrument]
    fn start(&mut self) -> Result<StartTime, TimeTrackerError> {
        if self.lockfile.exists() {
            Err(Report::from(TimeTrackerError)).attach_printable("already tracking")
        } else {
            let start_time = StartTime::now();

            let serialized = serialize_start_time(start_time)?;

            OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&self.lockfile)
                .change_context(TimeTrackerError)
                .attach_printable(format!("path: {:?}", &self.lockfile))
                .attach_printable("failed to open lockfile")
                .attach(Suggestion("make sure you have read permissions"))?
                .write_all(serialized.as_bytes())
                .change_context(TimeTrackerError)
                .attach_printable("failed to write lockfile data")
                .attach(Suggestion("make sure you have write permissions"))?;

            Ok(start_time)
        }
    }

    #[instrument]
    fn stop(&mut self) -> Result<EndTime, TimeTrackerError> {
        let lockfile_data = read_lockfile(&self.lockfile)?;

        let records = load_records(&self.records)?;
        let end = EndTime::now();
        let records = records.chain(std::iter::once(TimeRecord {
            start: lockfile_data.start_time,
            end,
        }));
        save_records(&self.records, records.collect())?;

        std::fs::remove_file(&self.lockfile)
            .change_context(TimeTrackerError)
            .attach_printable("failed to remove lockfile")
            .attach_printable(format!("lockfile path: {:?}", self.lockfile))
            .attach(Suggestion("make sure you have write permissions"))?;

        Ok(end)
    }

    #[instrument]
    fn running(&self) -> Result<Option<StartTime>, TimeTrackerError> {
        if self.lockfile.exists() {
            let lockfile_data = read_lockfile(&self.lockfile)?;
            Ok(Some(lockfile_data.start_time))
        } else {
            Ok(None)
        }
    }
}

fn serialize_start_time(start_time: StartTime) -> Result<String, TimeTrackerError> {
    let lockfile_data = LockfileData { start_time };
    serde_json::to_string(&lockfile_data)
        .change_context(TimeTrackerError)
        .attach_printable("failed to serialize lockfile data")
}

#[instrument(skip(records))]
fn save_records(db: &Path, records: Vec<TimeRecord>) -> Result<(), TimeTrackerError> {
    let records = serde_json::to_string(&records)
        .change_context(TimeTrackerError)
        .attach_printable("failed to serialize records")?;

    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .open(db)
        .change_context(TimeTrackerError)
        .attach_printable("failed to open db")?
        .write_all(records.as_bytes())
        .change_context(TimeTrackerError)
        .attach_printable("failed to write db")
        .attach(Suggestion("make sure you have write permissions"))
}

#[instrument]
fn read_lockfile(lockfile: &Path) -> Result<LockfileData, TimeTrackerError> {
    let mut lockfile_data = String::default();
    OpenOptions::new()
        .read(true)
        .open(lockfile)
        .change_context(TimeTrackerError)
        .attach_printable("failed to open lockfile")
        .attach(format!("path: {}", lockfile.display()))?
        .read_to_string(&mut lockfile_data)
        .change_context(TimeTrackerError)
        .attach_printable("failed to read lockfile")
        .attach(Suggestion(
            "your lockfile contains invalid data. delete it and then try again",
        ))?;

    let lockfile_data: LockfileData = serde_json::from_str(&lockfile_data)
        .change_context(TimeTrackerError)
        .attach_printable("failed to deserialize lockfile")
        .attach(Suggestion(
            "your lockfile may be empty or corrupted. delete it and then try again",
        ))?;
    Ok(lockfile_data)
}

#[cfg(test)]
pub mod tests {
    use assert_fs::{
        fixture::{ChildPath, FixtureError},
        prelude::*,
        TempDir,
    };

    use super::*;

    pub fn tracking_paths() -> Result<(TempDir, ChildPath, ChildPath), FixtureError> {
        let temp = TempDir::new()?;
        let db = temp.child("db.json");
        let lockfile = temp.child("lockfile");
        Ok((temp, db, lockfile))
    }

    pub fn new_flat_file_tracker(db: ChildPath, lockfile: &ChildPath) -> FlatFileTracker {
        FlatFileTracker::new(db.to_path_buf(), lockfile.to_path_buf())
    }

    #[test]
    fn running_is_none_when_lockfile_missing() {
        let (_tree, db, lockfile) = tracking_paths().unwrap();

        let tracker = new_flat_file_tracker(db, &lockfile);

        assert!(tracker.running().unwrap().is_none());
    }

    #[test]
    fn running_is_some_when_lockfile_found() {
        let (_tree, db, lockfile) = tracking_paths().unwrap();

        let mut tracker = new_flat_file_tracker(db, &lockfile);
        tracker.start().unwrap();

        assert!(tracker.running().unwrap().is_some());
    }

    #[test]
    fn starts_tracking() {
        let (_tree, db, lockfile) = tracking_paths().unwrap();

        let mut tracker = new_flat_file_tracker(db, &lockfile);

        assert!(tracker.start().is_ok());
    }

    #[test]
    fn returns_err_if_starting_while_already_tracking() {
        let (_tree, db, lockfile) = tracking_paths().unwrap();
        lockfile.touch().unwrap();

        let mut tracker = new_flat_file_tracker(db, &lockfile);

        assert!(tracker.start().is_err());
    }

    #[test]
    fn lockfile_created_when_tracking_starts() {
        let (_tree, db, lockfile) = tracking_paths().unwrap();

        let mut tracker = new_flat_file_tracker(db, &lockfile);
        tracker.start().unwrap();

        assert!(lockfile.path().exists());
    }

    #[test]
    fn lockfile_deleted_when_tracking_stops() {
        let (_tree, db, lockfile) = tracking_paths().unwrap();

        let mut tracker = new_flat_file_tracker(db, &lockfile);
        tracker.start().unwrap();
        tracker.stop().unwrap();

        assert!(!lockfile.path().exists());
    }

    #[test]
    fn stops_tracking() {
        let (_tree, db, lockfile) = tracking_paths().unwrap();

        let mut tracker = new_flat_file_tracker(db, &lockfile);
        tracker.start().unwrap();

        assert!(tracker.stop().is_ok());
    }
}
