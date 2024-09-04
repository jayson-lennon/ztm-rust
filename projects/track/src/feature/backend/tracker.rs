use error_stack::Result;

use super::{EndTime, StartTime};

/// An error that may occur with a time tracker.
#[derive(Debug, thiserror::Error)]
#[error("time tracker error")]
pub struct TimeTrackerError;

/// Tracks time.
pub trait Tracker {
    /// Starts time tracking.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the tracker is already running or if there was a problem starting the
    /// tracker.
    fn start(&mut self) -> Result<StartTime, TimeTrackerError>;

    /// Stops time tracking.
    ///
    /// # Errors
    ///
    /// Returns `Err` if there was a problem stopping the tracker.
    fn stop(&mut self) -> Result<EndTime, TimeTrackerError>;

    /// Returns `Some` if the tracker is currently tracking time.
    ///
    /// # Errors
    ///
    /// Returns `Err` if there was a problem accessing tracking data.
    fn running(&self) -> Result<Option<StartTime>, TimeTrackerError>;
}
