// Topic: Control flow with macros
//
// Summary:
//   The existing program has multiple checks to confirm that a user can access
//   a resource. Refactor the `get_data` function to use a macro to check for
//   all of the requirements.
//
// Requirements:
// * Create a macro that returns early from a function.
//
// Notes:
// * The macro should check for all existing requirements as indicated in
//   the `get_data` function.
// * Use `cargo test --bin m1` to check your work.

use std::error::Error;
use std::fmt;

struct Resource(String);
impl From<Resource> for Vec<u8> {
    fn from(resource: Resource) -> Vec<u8> {
        resource.0.as_bytes().into()
    }
}

#[derive(Clone, Copy)]
struct UserId(usize);

#[derive(Debug)]
enum ServeError {
    AccountInactive,
    NotLoggedIn,
    Unauthorized,
}
impl Error for ServeError {}
impl fmt::Display for ServeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AccountInactive => write!(f, "Account not active"),
            Self::NotLoggedIn => write!(f, "Login required"),
            Self::Unauthorized => write!(f, "Unauthorized"),
        }
    }
}

fn account_is_active(user: UserId) -> bool {
    match user.0 {
        1 | 2 | 3 => true,
        _ => false,
    }
}

fn is_logged_in(user: UserId) -> bool {
    match user.0 {
        2 | 3 => true,
        _ => false,
    }
}

fn is_authorized(user: UserId) -> bool {
    match user.0 {
        3 => true,
        _ => false,
    }
}

fn get_data<T: Into<Vec<u8>>>(resource: T, user: UserId) -> Result<Vec<u8>, ServeError> {
    if !account_is_active(user) {
        return Err(ServeError::AccountInactive);
    }
    if !is_logged_in(user) {
        return Err(ServeError::NotLoggedIn);
    }
    if !is_authorized(user) {
        return Err(ServeError::Unauthorized);
    }
    Ok(resource.into())
}

fn main() {
    // Use `cargo test --bin m1` to check your work.
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn errors_when_user_inactive() {
        let resource = Resource("Sample".into());
        let user = UserId(4);
        let data = get_data(resource, user);
        assert!(
            matches!(data, Err(ServeError::AccountInactive)),
            "should be an AccountInactive error"
        );
    }

    #[test]
    fn errors_when_user_not_logged_in() {
        let resource = Resource("Sample".into());
        let user = UserId(1);
        let data = get_data(resource, user);
        assert!(
            matches!(data, Err(ServeError::NotLoggedIn)),
            "should be a NotLoggedIn error"
        );
    }

    #[test]
    fn errors_when_user_not_authorized() {
        let resource = Resource("Sample".into());
        let user = UserId(2);
        let data = get_data(resource, user);
        assert!(
            matches!(data, Err(ServeError::Unauthorized)),
            "should be an Unauthorized error"
        );
    }

    #[test]
    fn ok_when_all_checks_pass() {
        let resource = Resource("Sample".into());
        let user = UserId(3);
        let data = get_data(resource, user);
        assert!(data.is_ok());
    }
}
