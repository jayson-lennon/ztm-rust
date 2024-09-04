//! Time tracking implementations
mod flatfile;
mod reporter;
mod tracker;

use chrono::{DateTime, Utc};
pub use flatfile::{FlatFileReporter, FlatFileTracker};
pub use reporter::{ReportTimespan, Reporter, TWENTY_FOUR_HOURS};
use serde::{Deserialize, Serialize};
pub use tracker::Tracker;

/// The start of a tracked time record.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct StartTime(DateTime<Utc>);

impl StartTime {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub const fn timestamp_millis(&self) -> i64 {
        self.0.timestamp_millis()
    }
}

impl PartialOrd<DateTime<Utc>> for StartTime {
    fn partial_cmp(&self, other: &DateTime<Utc>) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialEq<DateTime<Utc>> for StartTime {
    fn eq(&self, other: &DateTime<Utc>) -> bool {
        &self.0 == other
    }
}

/// The end of a tracked time record.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EndTime(DateTime<Utc>);

impl EndTime {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub const fn timestamp_millis(&self) -> i64 {
        self.0.timestamp_millis()
    }
}

/// A tracked timespan.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TimeRecord {
    pub start: StartTime,
    pub end: EndTime,
}
