use crate::data::DbId;
use derive_more::Constructor;
use serde::{Deserialize, Serialize};

/// The internal database id field for a [`Clip`](crate::domain::clip::Clip).
#[derive(Clone, Debug, Constructor, Deserialize, Serialize)]
pub struct ClipId(DbId);

impl ClipId {
    /// Return the underlying [`DbId`](crate::data::DbId).
    pub fn into_inner(self) -> DbId {
        self.0
    }
}

impl From<DbId> for ClipId {
    fn from(id: DbId) -> Self {
        Self(id)
    }
}

/// The Default implementation for for [`ClipId`] is an empty ID.
impl Default for ClipId {
    fn default() -> Self {
        Self(DbId::nil())
    }
}