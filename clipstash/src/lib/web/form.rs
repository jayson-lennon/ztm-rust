//! Form data.

use crate::domain::clip::field;
use rocket::form::FromForm;
use serde::Serialize;

/// The form to create a new [`Clip`](crate::Clip).
#[derive(Debug, Serialize, FromForm)]
pub struct NewClip {
    pub content: field::Content,
    pub title: field::Title,
    pub expires: field::Expires,
    pub password: field::Password,
}

/// The form to submit a [`Password`](crate::domain::clip::field::Password) for a protected [`Clip`](crate::Clip).
#[derive(Debug, Serialize, FromForm)]
pub struct GetPasswordProtectedClip {
    pub password: field::Password,
}
