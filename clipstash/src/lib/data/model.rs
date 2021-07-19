//! Database models for executing queries & returning data.

use crate::data::DbId;
use crate::{ClipError, ShortCode, Time};
use chrono::{NaiveDateTime, Utc};
use std::convert::TryFrom;

/// Clip that is stored in, and retrieved from, the database.
#[derive(Debug, sqlx::FromRow)]
pub struct Clip {
    pub(in crate::data) clip_id: String,
    pub(in crate::data) shortcode: String,
    pub(in crate::data) content: String,
    pub(in crate::data) title: Option<String>,
    pub(in crate::data) posted: NaiveDateTime,
    pub(in crate::data) expires: Option<NaiveDateTime>,
    pub(in crate::data) password: Option<String>,
    pub(in crate::data) hits: i64,
}

/// Convert from a database model into a domain Clip.
impl TryFrom<Clip> for crate::domain::Clip {
    type Error = ClipError;
    fn try_from(clip: Clip) -> Result<Self, Self::Error> {
        use crate::domain::clip::field;
        use std::str::FromStr;
        Ok(Self {
            clip_id: field::ClipId::new(DbId::from_str(clip.clip_id.as_str())?),
            shortcode: field::ShortCode::from(clip.shortcode),
            content: field::Content::new(clip.content.as_str())?,
            title: field::Title::new(clip.title),
            posted: field::Posted::new(Time::from_naive_utc(clip.posted)),
            expires: field::Expires::new(clip.expires.map(Time::from_naive_utc)),
            password: field::Password::new(clip.password.unwrap_or_default())?,
            hits: field::Hits::new(u64::try_from(clip.hits)?),
        }

        )
    }
}

/// Data required to run the [`get_clip`](crate::data::query::get_clip()) query to get a [`Clip`] from the database.
pub struct GetClip {
    pub(in crate::data) shortcode: String,
}

impl From<crate::service::ask::GetClip> for GetClip {
    fn from(req: crate::service::ask::GetClip) -> Self {
        Self {
            shortcode: req.shortcode.into_inner()
        }
    }
}

impl From<ShortCode> for GetClip {
    fn from(shortcode: ShortCode) -> Self {
        GetClip {
            shortcode: shortcode.into_inner()
        }
    }
}

impl From<String> for GetClip {
    fn from(shortcode: String) -> Self {
        GetClip { shortcode }
    }
}

/// Data required to run the [`new_clip`](crate::data::query::new_clip()) query to add a [`Clip`] to the database.
pub struct NewClip {
    pub(in crate::data) clip_id: String,
    pub(in crate::data) shortcode: String,
    pub(in crate::data) content: String,
    pub(in crate::data) title: Option<String>,
    pub(in crate::data) posted: i64,
    pub(in crate::data) expires: Option<i64>,
    pub(in crate::data) password: Option<String>,
}

impl From<crate::service::ask::NewClip> for NewClip {
    fn from(req: crate::service::ask::NewClip) -> Self {
        Self {
            clip_id: DbId::new().into(),
            content: req.content.into_inner(),
            title: req.title.into_inner(),
            expires: req.expires.into_inner().map(|time| time.timestamp()),
            password: req.password.into_inner(),
            shortcode: ShortCode::default().into(),
            posted: Utc::now().timestamp(),
        }
    }
}

/// Data required to run the [`update_clip`](crate::data::query::update_clip()) query to update a [`Clip`] in the database.
pub struct UpdateClip {
    pub(in crate::data) shortcode: String,
    pub(in crate::data) content: String,
    pub(in crate::data) title: Option<String>,
    pub(in crate::data) expires: Option<i64>,
    pub(in crate::data) password: Option<String>,
}

impl From<crate::service::ask::UpdateClip> for UpdateClip {
    fn from(req: crate::service::ask::UpdateClip) -> Self {
        Self {
            content: req.content.into_inner(),
            title: req.title.into_inner(),
            expires: req.expires.into_inner().map(|time| time.timestamp()),
            password: req.password.into_inner(),
            shortcode: ShortCode::default().into(),
        }
    }
}