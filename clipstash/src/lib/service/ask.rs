//! Data structures to make a service request.

use crate::domain::clip::field;
use crate::ShortCode;

use serde::{Deserialize, Serialize};

/// Data required to run the [`new_clip`](crate::service::action::new_clip()) action to add a new [`crate::domain::Clip`].
#[derive(Debug, Deserialize, Serialize)]
pub struct NewClip {
    pub content: field::Content,
    pub title: field::Title,
    pub expires: field::Expires,
    pub password: field::Password,
}

/// Data required to run the [`update_clip`](crate::service::action::update_clip()) action to update [`crate::domain::Clip`] data.
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateClip {
    pub content: field::Content,
    pub title: field::Title,
    pub expires: field::Expires,
    pub password: field::Password,
    pub shortcode: field::ShortCode,
}


/// Data required to run the [`get_clip`](crate::service::action::get_clip()) action to get a [`crate::domain::Clip`].
#[derive(Debug, Deserialize, Serialize)]
pub struct GetClip {
    pub shortcode: ShortCode,
    pub password: field::Password,
}

impl GetClip {
    /// Convert a [`&str`] into a [`GetClip`] action request.
    pub fn from_raw(shortcode: &str) -> Self {
        Self {
            shortcode: ShortCode::from(shortcode),
            password: field::Password::default()
        }
    }
}

impl From<ShortCode> for GetClip {
    fn from(shortcode: ShortCode) -> Self {
        Self {
            shortcode,
            password: field::Password::default(),
        }
    }
}

impl From<&str> for GetClip {
    fn from(raw: &str) -> Self {
        Self::from_raw(raw)
    }
}