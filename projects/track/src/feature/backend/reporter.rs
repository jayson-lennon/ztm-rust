use chrono::{DateTime, Utc};
use error_stack::Result;
use std::time::Duration;

use super::TimeRecord;

pub const TWENTY_FOUR_HOURS: Duration = Duration::from_secs(60 * 60 * 24);

/// An error that may occur with a time reporter.
#[derive(Debug, thiserror::Error)]
#[error("time reporter error")]
pub struct ReporterError;

/// The interval of time to use when generating a report.
#[derive(Debug, Clone, Copy)]
pub enum ReportTimespan {
    Since(DateTime<Utc>),
}

/// Generates data for reports.
pub trait Reporter {
    /// Returns all records.
    ///
    /// # Errors
    ///
    /// Returns `Err` if there was a problem accessing the records.
    fn records(&self) -> Result<impl Iterator<Item = TimeRecord>, ReporterError>;

    /// Returns the total duration tracked across a given timespan.
    fn total_duration(&self, timespan: ReportTimespan) -> Result<Duration, ReporterError>;
}
