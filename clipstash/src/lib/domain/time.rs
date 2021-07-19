//! Time wrapper structure.

use chrono::{DateTime, NaiveDateTime, Utc};
use derive_more::From;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// This type uses Utc time only.
#[derive(Clone, Debug, From, Deserialize, Serialize)]
pub struct Time(DateTime<Utc>);

impl Time {
    /// Get the underlying ['DateTime']
    pub fn into_inner(self) -> DateTime<Utc> {
        self.0
    }

    /// Return the number of seconds in the Time.
    pub fn timestamp(&self) -> i64 {
        self.0.timestamp()
    }

    /// Convert a [`NaiveDateTime`] into a [`Time`]
    pub fn from_naive_utc(datetime: NaiveDateTime) -> Self {
        Time(DateTime::from_utc(datetime, Utc))
    }
}

/// The format required is `YYYY-MM-DDThh:mm:ssZ`.
///
/// See the[`chrono`](chrono::format::strftime) docs for more info.
impl FromStr for Time {
    type Err = chrono::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match format!("{}T00:00:00Z", s).parse::<DateTime<Utc>>() {
            Ok(time) => Ok(time.into()),
            Err(e) => Err(e)
        }
    }
}