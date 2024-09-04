use crate::error::Suggestion;
use clap_verbosity_flag::Verbosity;
use error_stack::{fmt::ColorMode, Report};
use owo_colors::OwoColorize;
use tracing_log::AsTrace;
use tracing_subscriber::EnvFilter;

pub fn error_reporting() {
    Report::set_color_mode(ColorMode::Color);
    Report::install_debug_hook::<Suggestion>(|value, context| {
        let msg = value.0;
        let body = format!("suggestion: {msg}");
        match context.color_mode() {
            ColorMode::Color => context.push_body(body.cyan().to_string()),
            ColorMode::Emphasis => context.push_body(body.italic().to_string()),
            ColorMode::None => context.push_body(body),
        };
    });
}

pub const ENV_FILTER_TARGETS: [&str; 1] = ["track"];

pub fn tracing(verbosity: &Verbosity) {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().pretty())
        .with(
            EnvFilter::builder()
                .with_default_directive(verbosity.log_level_filter().as_trace().into())
                .from_env_lossy(),
        )
        .with(ErrorLayer::default())
        .init();
}
