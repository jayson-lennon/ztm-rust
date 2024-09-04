mod reporter;
mod tracker;

use std::{fs::OpenOptions, io::Read, path::Path};

use error_stack::{Result, ResultExt};
use tracing::instrument;

use super::{tracker::TimeTrackerError, TimeRecord};

pub use reporter::FlatFileReporter;
pub use tracker::FlatFileTracker;

/// Returns an iterator over all records for the flat file backend.
#[instrument]
fn load_records(db: &Path) -> Result<impl Iterator<Item = TimeRecord>, TimeTrackerError> {
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

    if buf.is_empty() {
        Ok(Vec::default().into_iter())
    } else {
        let records: Vec<TimeRecord> = serde_json::from_str(&buf)
            .change_context(TimeTrackerError)
            .attach_printable("failed to deserialize records")?;
        Ok(records.into_iter())
    }
}
