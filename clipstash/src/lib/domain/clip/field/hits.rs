use derive_more::Constructor;
use serde::{Deserialize, Serialize};

/// The hits field for a [`Clip`](crate::domain::clip::Clip).
#[derive(Clone, Constructor, Debug, Deserialize, Serialize)]
pub struct Hits(u64);

impl Hits {
    /// Return the underlying [`u64`].
    pub fn into_inner(self) -> u64 {
        self.0
    }
}