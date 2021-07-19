use crate::domain::clip::ClipError;
use serde::{Deserialize, Serialize};
use rocket::form::{self, FromFormField, ValueField};
use std::str::FromStr;

/// The password field for a [`Clip`](crate::domain::clip::Clip).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct Password(Option<String>);

impl Password {
    /// Create a new `Password` field.
    ///
    /// There are no complexity checks currently implemented, however, if
    /// some were to be added, then a [`ClipError`] should be returned.
    pub fn new<T: Into<Option<String>>>(password: T) -> Result<Self, ClipError> {
        let password: Option<String> = password.into();
        match password {
            Some(password) => {
                if !password.trim().is_empty() {
                    Ok(Self(Some(password)))
                } else {
                    Ok(Self(None))
                }
            }
            None => Ok(Self(None))
        }
    }

    /// Return the underlying [`String`].
    pub fn into_inner(self) -> Option<String> {
        self.0
    }

    /// Returns whether a password has been set.
    pub fn has_password(&self) -> bool {
        self.0.is_some()
    }
}

/// The Default implementation is no password.
impl Default for Password {
    fn default() -> Self {
        Self(None)
    }
}

impl FromStr for Password {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Password {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        Ok(Self::new(field.value.to_owned())
            .map_err(|e| form::Error::validation(format!("{}", e)))?
        )
    }
}

#[cfg(test)]
mod test {
    use super::Password;

    #[test]
    fn empty_password_is_none() {
        assert_eq!(false, Password::new("".to_owned()).unwrap().has_password());
    }

    #[test]
    fn default_is_none() {
        assert_eq!(false, Password::default().has_password());
    }

    #[test]
    fn accepts_valid_password() {
        assert!(Password::new("123".to_owned()).unwrap().has_password());
    }
}
