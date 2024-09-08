//! Statistics reporting

use std::time::Duration;

#[derive(Debug, Default)]
pub struct HMSFormatter;

pub trait DurationFormat {
    fn format(&self, duration: Duration) -> String;
}

impl DurationFormat for HMSFormatter {
    fn format(&self, duration: Duration) -> String {
        let total_seconds = duration.as_secs();

        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;

        // Format into HH:MM:SS
        format!("{hours:02}:{minutes:02}:{seconds:02}")
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn formats_seconds() {
        let duration = Duration::from_secs(5);

        let formatter = HMSFormatter::default();

        let text = formatter.format(duration);

        assert_eq!(&text, "00:00:05");
    }
}
