use chrono::{DateTime, Local, Utc};

/// Converts a UTC time to Local time.
pub trait UtcToLocal {
    /// Convert UTC to Local time.
    fn to_local(&self) -> DateTime<Local>;
}

impl UtcToLocal for DateTime<Utc> {
    fn to_local(&self) -> DateTime<Local> {
        self.with_timezone(&Local)
    }
}
