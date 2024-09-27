use crate::domain::clip::ClipError;
use derive_more::From;
use rocket::{UriDisplayPath, UriDisplayQuery};
use serde::{Deserialize, Serialize};
use rocket::request::FromParam;
use std::str::FromStr;

/// The shortcode field for a [`Clip`](crate::domain::clip::Clip).
///
/// The shortcode is utilized by clients to locate the `Clip` within the service.
#[derive(Debug, Clone, Deserialize, Serialize, From, UriDisplayQuery, UriDisplayPath, Hash, Eq, PartialEq)]
pub struct ShortCode(String);

impl ShortCode {
    /// Create a new `ShortCode` field.
    pub fn new() -> Self {
        use rand::prelude::*;
        let allowed_chars = [
            'a', 'b', 'c', 'd', '1', '2', '3', '4'
        ];

        let mut rng = thread_rng();
        let mut shortcode = String::with_capacity(10);
        for _ in 0..10 {
            shortcode.push(
                *allowed_chars
                    .choose(&mut rng)
                    .expect("sampling array should have values"),
            );
        }
        Self(shortcode)
    }

    /// Return the underlying [`&str`].
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Return the underlying [`String`].
    pub fn into_inner(self) -> String {
        self.0
    }
}

/// The Default implementation is a new randomly generated shortcode.
impl Default for ShortCode {
    fn default() -> Self {
        Self::new()
    }
}

impl From<ShortCode> for String {
    fn from(shortcode: ShortCode) -> Self {
        shortcode.0
    }
}

impl From<&str> for ShortCode {
    fn from(shortcode: &str) -> Self {
        ShortCode(shortcode.to_owned())
    }
}

impl<'r> FromParam<'r> for ShortCode {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        Ok(ShortCode::from(param))
    }
}

impl FromStr for ShortCode {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.into()))
    }
}