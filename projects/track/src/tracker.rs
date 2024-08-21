mod flatfile;

use chrono::{DateTime, Utc};
use error_stack::Result;
use serde::{Deserialize, Serialize};

pub use flatfile::FlatFileTracker;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct StartTime(DateTime<Utc>);

impl StartTime {
    pub fn now() -> Self {
        Self(Utc::now())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EndTime(DateTime<Utc>);

impl EndTime {
    pub fn now() -> Self {
        Self(Utc::now())
    }
}

#[derive(Debug, thiserror::Error)]
#[error("time tracker error")]
pub struct TimeTrackerError;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TimeRecord {
    start: StartTime,
    end: EndTime,
}

pub trait TimeTracker {
    fn start(&mut self) -> Result<StartTime, TimeTrackerError>;
    fn stop(&mut self) -> Result<EndTime, TimeTrackerError>;
    fn records(&self) -> Result<Vec<TimeRecord>, TimeTrackerError>;
    fn is_tracking(&self) -> Result<bool, TimeTrackerError>;
}
