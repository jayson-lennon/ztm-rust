//! Time tracking implementations

mod flatfile;

use crate::common::UtcToLocal;
use chrono::{DateTime, Local, Utc};
use error_stack::Result;
pub use flatfile::FlatFileTracker;
use serde::{Deserialize, Serialize};

/// The start of a tracked time record.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct StartTime(DateTime<Utc>);

impl StartTime {
    pub fn now() -> Self {
        Self(Utc::now())
    }
}

impl AsRef<DateTime<Utc>> for StartTime {
    fn as_ref(&self) -> &DateTime<Utc> {
        &self.0
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
    ///
    /// Returns `Err` if the tracker is already running or if there was a problem starting the
    /// tracker.
    fn start(&mut self) -> Result<StartTime, TimeTrackerError>;

    /// Stops time tracking.
    ///
    /// Returns `Err` if there was a problem starting the tracker.
    fn stop(&mut self) -> Result<EndTime, TimeTrackerError>;

    /// Returns all tracking records.
    ///
    /// Returns `Err` if there was a problem accessing the records.
    fn records(&self) -> Result<Vec<TimeRecord>, TimeTrackerError>;

    /// Returns `Some` if the tracker is currently tracking time.
    ///
    /// Returns `Err` if there was a problem accessing tracking data.
    fn running(&self) -> Result<Option<StartTime>, TimeTrackerError>;
}

#[cfg(test)]
pub mod tlib {
    use error_stack::ResultExt;

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

    /// A fake time tracker used for testing.
    pub struct FakeTracker {
        records: Vec<TimeRecord>,
        current: Option<StartTime>,
    }

    impl TimeTracker for FakeTracker {
        fn start(&mut self) -> Result<StartTime, TimeTrackerError> {
            if self.current.is_none() {
                let start_time = StartTime::now();
                self.current = Some(start_time);
                Ok(start_time)
            } else {
                Err(TimeTrackerError).attach_printable("time already being tracked")
            }
        }

        fn stop(&mut self) -> Result<EndTime, TimeTrackerError> {
            let end = EndTime::now();
            if let Some(start) = self.current.take() {
                let record = TimeRecord { start, end };
                self.records.push(record);
            }
            Ok(end)
        }

        fn records(&self) -> Result<Vec<TimeRecord>, TimeTrackerError> {
            Ok(self.records.clone())
        }

        fn running(&self) -> Result<Option<StartTime>, TimeTrackerError> {
            Ok(self.current)
        }
    }
}
