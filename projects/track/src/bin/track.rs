use error_stack::{Result, ResultExt};
use track::{
    error::AppError,
    feature::cli::{self},
    init::{self},
};

fn main() -> Result<(), AppError> {
    init::error_reporting();

    init::tracing();

    cli::run().change_context(AppError)?;

    Ok(())
}
