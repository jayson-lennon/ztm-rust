use crate::error::Suggestion;
use error_stack::{fmt::ColorMode, Report};
use owo_colors::OwoColorize;

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

pub fn tracing(env_filter_targets: &[&str]) {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;
    use tracing_subscriber::Layer;
    let pretty = tracing_subscriber::fmt::layer()
        .pretty()
        // .with_span_events(FmtSpan::CLOSE)
        .with_writer(std::io::stdout)
        .boxed();
    tracing_subscriber::registry()
        .with(pretty)
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                env_filter_targets
                    .iter()
                    .map(|s| format!("{s}=trace"))
                    .collect::<Vec<_>>()
                    .join(",")
                    .into()
            }),
        )
        .with(ErrorLayer::default())
        .init();
}
