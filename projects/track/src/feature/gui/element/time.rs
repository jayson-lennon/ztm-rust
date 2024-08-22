use chrono::Utc;
use iced::widget::{row, text, Row, Text};

use crate::feature::{
    gui::{GuiApp, Message},
    report::{
        duration_format::{self, DurationFormat},
        ReportTimespan, TrackingReport,
    },
};

impl GuiApp {
    pub fn current_tracking_time(&self) -> Text {
        if let Some(start_time) = &self.start_time {
            let now = Utc::now();
            let duration = (now - start_time.as_ref()).to_std().unwrap();
            let duration = duration_format::HourMinSecFormatter.format(duration);
            Text::new(duration)
        } else {
            text![""]
        }
    }

    pub(in crate::feature::gui) fn time_tracked_today(&self) -> Row<Message> {
        let reporter = TrackingReport;

        let duration = {
            let duration = reporter.duration(Utc::now(), ReportTimespan::Today, &self.records);
            let duration = duration_format::HourMinSecFormatter.format(duration);
            Text::new(duration)
        };

        row![text!("Total tracked today: "), duration]
    }
}
