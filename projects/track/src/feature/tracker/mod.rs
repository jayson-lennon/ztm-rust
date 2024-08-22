mod flatfile;

use chrono::{DateTime, Local, Utc};
use error_stack::Result;
use serde::{Deserialize, Serialize};

pub use flatfile::FlatFileTracker;

use crate::common::UtcToLocal;

/// The start of a tracked time record.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct StartTime(DateTime<Utc>);

impl StartTime {
    pub fn now() -> Self {
        Self(Utc::now())
    }
}

impl UtcToLocal for StartTime {
    fn to_local(&self) -> DateTime<Local> {
        self.0.to_local()
    }
}

/// The end of a tracked time record.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EndTime(DateTime<Utc>);

impl EndTime {
    pub fn now() -> Self {
        Self(Utc::now())
    }
}

impl UtcToLocal for EndTime {
    fn to_local(&self) -> DateTime<Local> {
        self.0.to_local()
    }
}

/// An error that may occur with a time tracker.
#[derive(Debug, thiserror::Error)]
#[error("time tracker error")]
pub struct TimeTrackerError;

/// A record of tracked time.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TimeRecord {
    pub start: StartTime,
    pub end: EndTime,
}

/// Tracks time and loads records.
pub trait TimeTracker {
    /// Starts time tracking.
    fn start(&mut self) -> Result<StartTime, TimeTrackerError>;

    /// Stops time tracking.
    fn stop(&mut self) -> Result<EndTime, TimeTrackerError>;

    /// Returns all tracking records.
    fn records(&self) -> Result<Vec<TimeRecord>, TimeTrackerError>;

    /// Returns `true` when currently tracking.
    fn is_tracking(&self) -> Result<bool, TimeTrackerError>;
}

#[cfg(test)]
pub mod tlib {
    use super::*;

    /// Test methods for working with time records.
    pub trait TimeRecordTestExt {
        fn from_seconds(timestamp: (i64, i64)) -> TimeRecord;
    }

    impl TimeRecordTestExt for TimeRecord {
        fn from_seconds((start, end): (i64, i64)) -> TimeRecord {
            if start > end {
                panic!("start time cannot be greater than end time");
            }
            TimeRecord {
                start: StartTime(DateTime::from_timestamp_millis(start * 1000).unwrap()),
                end: EndTime(DateTime::from_timestamp_millis(end * 1000).unwrap()),
            }
        }
    }
}
