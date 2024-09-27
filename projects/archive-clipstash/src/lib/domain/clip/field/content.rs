use crate::domain::clip::ClipError;
use rocket::form::{self, FromFormField, ValueField};
use serde::{Deserialize, Serialize};

/// The content field for a [`Clip`](crate::domain::clip::Clip).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Content(String);

impl Content {
    /// Create a new `Content` field.
    ///
    /// If the content provided is empty, then a [`ClipError`] will be returned.
    pub fn new(content: &str) -> Result<Self, ClipError> {
        if !content.trim().is_empty() {
            Ok(Self(content.to_owned()))
        } else {
            Err(ClipError::EmptyContent)
        }
    }
    /// Return the underlying [`String`].
    pub fn into_inner(self) -> String {
        self.0
    }
    /// Return a reference to the underlying [`&str`].
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Content {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        Ok(Self::new(field.value)
            .map_err(|e| form::Error::validation(format!("{}", e)))?
        )
    }
}

#[cfg(test)]
mod test {
    use super::Content;

    #[test]
    fn disallow_empty_content() {
        assert!(Content::new("").is_err());
    }
}
