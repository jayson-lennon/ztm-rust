use std::{
    fs::OpenOptions,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use super::{EndTime, StartTime, TimeRecord, TimeTracker, TimeTrackerError};
use error_stack::{Report, Result, ResultExt};
use serde::{Deserialize, Serialize};
use tracing::instrument;

/// An error that may occur while using the flat file tracker.
#[derive(Debug, thiserror::Error)]
#[error("filesystem tracker error")]
pub struct FlatFileTrackerError;

/// A time tracker that uses a flat-file database.
#[derive(Debug)]
pub struct FlatFileTracker {
    /// Where all the tracking records are stored.
    records: PathBuf,

    /// Path to use when tracking is active.
    lockfile: PathBuf,
}

impl FlatFileTracker {
    /// Create a new flat file tracker.
    pub fn new(records: PathBuf, lockfile: PathBuf) -> Result<Self, FlatFileTrackerError> {
        Ok(Self { records, lockfile })
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

impl TimeTracker for FlatFileTracker {
    fn start(&mut self) -> Result<StartTime, TimeTrackerError> {
        if !self.lockfile.exists() {
            let start_time = StartTime::now();

            let serialized = {
                let lockfile_data = LockfileData { start_time };
                serde_json::to_string(&lockfile_data)
                    .change_context(TimeTrackerError)
                    .attach_printable("failed to serialize lockfile data")?
            };
            OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&self.lockfile)
                .change_context(TimeTrackerError)
                .attach_printable("failed to open lockfile")?
                .write_all(serialized.as_bytes())
                .change_context(TimeTrackerError)
                .attach_printable("failed to write lockfile data")?;

            Ok(start_time)
        } else {
            Err(Report::from(TimeTrackerError)).attach_printable("already tracking")
        }
    }

    fn stop(&mut self) -> Result<EndTime, TimeTrackerError> {
        let mut lockfile_data = String::default();
        OpenOptions::new()
            .read(true)
            .open(&self.lockfile)
            .change_context(TimeTrackerError)
            .attach_printable("failed to open lockfile")?
            .read_to_string(&mut lockfile_data)
            .change_context(TimeTrackerError)
            .attach_printable("failed to read lockfile")?;

        let lockfile_data: LockfileData = serde_json::from_str(&lockfile_data)
            .change_context(TimeTrackerError)
            .attach_printable("failed to deserialize lockfile")?;

        let mut records = load_records(&self.records)?;
        records.push(TimeRecord {
            start: lockfile_data.start_time,
            end: EndTime::now(),
        });
        save_records(&self.records, &records)?;

        std::fs::remove_file(&self.lockfile)
            .change_context(TimeTrackerError)
            .attach_printable("failed to remove lockfile")?;

        Ok(EndTime::now())
    }

    fn records(&self) -> Result<Vec<TimeRecord>, TimeTrackerError> {
        load_records(&self.records)
    }

    fn is_tracking(&self) -> Result<bool, TimeTrackerError> {
        Ok(self.lockfile.exists())
    }
}

#[instrument(err(Debug))]
fn load_records(db: &Path) -> Result<Vec<TimeRecord>, TimeTrackerError> {
    let mut buf = String::default();
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(db)
        .change_context(TimeTrackerError)
        .attach_printable("failed to open db")?
        .read_to_string(&mut buf)
        .change_context(TimeTrackerError)
        .attach_printable("failed to read db")?;

    if !buf.is_empty() {
        serde_json::from_str(&buf)
            .change_context(TimeTrackerError)
            .attach_printable("failed to deserialize records")
    } else {
        Ok(Vec::default())
    }
}

#[instrument(err(Debug))]
fn save_records(db: &Path, records: &[TimeRecord]) -> Result<(), TimeTrackerError> {
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
}

#[cfg(test)]
mod tests {
    use assert_fs::{
        fixture::{ChildPath, FixtureError},
        prelude::*,
        TempDir,
    };

    use super::*;

    fn tracking_paths() -> Result<(TempDir, ChildPath, ChildPath), FixtureError> {
        let temp = TempDir::new()?;
        let db = temp.child("db.json");
        let lockfile = temp.child("lockfile");
        Ok((temp, db, lockfile))
    }

    #[test]
    fn is_tracking_false_when_lockfile_missing() {
        let (_tree, db, lockfile) = tracking_paths().unwrap();

        let tracker = FlatFileTracker::new(db.to_path_buf(), lockfile.to_path_buf()).unwrap();

        assert!(!tracker.is_tracking().unwrap());
    }

    #[test]
    fn is_tracking_true_when_lockfile_found() {
        let (_tree, db, lockfile) = tracking_paths().unwrap();

        let mut tracker = FlatFileTracker::new(db.to_path_buf(), lockfile.to_path_buf()).unwrap();
        tracker.start().unwrap();

        assert!(tracker.is_tracking().unwrap());
    }

    #[test]
    fn starts_tracking() {
        let (_tree, db, lockfile) = tracking_paths().unwrap();

        let mut tracker = FlatFileTracker::new(db.to_path_buf(), lockfile.to_path_buf()).unwrap();

        assert!(tracker.start().is_ok());
    }

    #[test]
    fn returns_err_if_starting_while_already_tracking() {
        let (_tree, db, lockfile) = tracking_paths().unwrap();
        lockfile.touch().unwrap();

        let mut tracker = FlatFileTracker::new(db.to_path_buf(), lockfile.to_path_buf()).unwrap();

        assert!(tracker.start().is_err());
    }

    #[test]
    fn lockfile_created_when_tracking_starts() {
        let (_tree, db, lockfile) = tracking_paths().unwrap();

        let mut tracker = FlatFileTracker::new(db.to_path_buf(), lockfile.to_path_buf()).unwrap();
        tracker.start().unwrap();

        assert!(lockfile.path().exists());
    }

    #[test]
    fn lockfile_deleted_when_tracking_stops() {
        let (_tree, db, lockfile) = tracking_paths().unwrap();

        let mut tracker = FlatFileTracker::new(db.to_path_buf(), lockfile.to_path_buf()).unwrap();
        tracker.start().unwrap();
        tracker.stop().unwrap();

        assert!(!lockfile.path().exists());
    }

    #[test]
    fn time_record_created_when_tracking_stops() {
        let (_tree, db, lockfile) = tracking_paths().unwrap();

        let mut tracker = FlatFileTracker::new(db.to_path_buf(), lockfile.to_path_buf()).unwrap();
        tracker.start().unwrap();
        tracker.stop().unwrap();

        assert!(!tracker.records().unwrap().is_empty());
    }

    #[test]
    fn stops_tracking() {
        let (_tree, db, lockfile) = tracking_paths().unwrap();

        let mut tracker = FlatFileTracker::new(db.to_path_buf(), lockfile.to_path_buf()).unwrap();
        tracker.start().unwrap();

        assert!(tracker.stop().is_ok());
    }
}