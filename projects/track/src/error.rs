#[derive(Debug, thiserror::Error)]
#[error("an application error has occurred")]
pub struct AppError;

/// A suggestion displayed to the user in the event of an error.
pub struct Suggestion(pub &'static str);
