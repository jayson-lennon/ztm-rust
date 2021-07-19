use crate::domain::time::Time;
use serde::{Deserialize, Serialize};
use derive_more::Constructor;

/// The date posted field for a [`Clip`](crate::domain::clip::Clip).
#[derive(Clone, Constructor, Debug, Deserialize, Serialize)]
pub struct Posted(Time);

impl Posted {
    /// Return the underlying [`Time`](crate::domain::time::Time).
    pub fn into_inner(self) -> Time {
        self.0
    }
}