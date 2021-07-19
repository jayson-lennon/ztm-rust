//! Structures, errors, and implementation for the [`Clip`](crate::Clip) data type.
pub mod field;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// The possible errors that can occur when building a [`Clip`]
#[derive(Debug, Error)]
pub enum ClipError {
    /// Password does not meet complexity requirements.
    #[error("invalid password: {0}")]
    InvalidPassword(String),

    /// Clip title has unwanted words/data.
    #[error("invalid title: {0}")]
    InvalidTitle(String),

    /// Content was not provided.
    #[error("empty content")]
    EmptyContent,

    /// Date is invalid: invalid day of the month, too far in the past, etc.
    #[error("invalid date: {0}")]
    InvalidDate(String),

    /// Date failed to parse.
    #[error("date parse error: {0}")]
    DateParse(#[from] chrono::ParseError),

    /// [crate::data::DbId] failed to parse.
    #[error("id parse error: {0}")]
    Id(#[from] uuid::Error),

    /// Number of hits is negative or not a number.
    #[error("hits parse error: {0}")]
    Hits(#[from] std::num::TryFromIntError),
}

/// Clip stores all the data about Clips posted to the service.
/// 
/// Each field in the Clip uses a newtype that encapsulates the requirements
/// for that particular field. If one of the fields cannot be created, then
/// a Clip cannot be created. This enforcement of field creation ensures
/// that a Clip will always be valid whenever it is utilized at any point
/// in the program.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Clip {
    #[serde(skip)]
    /// The internal [`DbId`](crate::data::DbId) for the Clip.
    pub clip_id: field::ClipId,
    /// The code used to access this clip from the service.
    pub shortcode: field::ShortCode,
    /// The content of the Clip.
    pub content: field::Content,
    /// The title of the Clip.
    pub title: field::Title,
    /// The date that this Clip was posted to the service.
    pub posted: field::Posted,
    /// The date that this Clip will expire.
    pub expires: field::Expires,
    /// The password needed to view this Clip.
    pub password: field::Password,
    /// The number of hits received by this Clip.
    pub hits: field::Hits,
}
