//! Statistics reporting

use std::time::Duration;

pub trait DurationFormat {
    fn format(&self, duration: Duration) -> String;
}

#[derive(Debug, Clone)]
pub struct HourMinSecFormatter;

impl DurationFormat for HourMinSecFormatter {
    fn format(&self, duration: Duration) -> String {
        // Calculate total seconds
        let total_seconds = duration.as_secs();

        // Calculate hours, minutes, and seconds
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;

        // Format into HH:MM:SS
        format!("{hours:02}:{minutes:02}:{seconds:02}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_seconds() {
        let duration = Duration::from_secs(5);
        let formatted = HourMinSecFormatter.format(duration);

        assert_eq!(&formatted, "00:00:05");
    }
}
