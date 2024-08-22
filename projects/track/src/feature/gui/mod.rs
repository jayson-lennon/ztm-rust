//! Desktop GUI

mod element;
#[cfg(test)]
mod tests;

use error_stack::{Report, Result, ResultExt};
use iced::widget::{column, row, Column};
use iced::{
    time::{self, Duration},
    Task,
};

use super::tracker::{StartTime, TimeRecord, TimeTracker, TimeTrackerError};

pub fn run(gui_app: GuiApp) -> iced::Result {
    let app = iced::application("Time Tracker", GuiApp::update, GuiApp::view);
    app.subscription(|_| time::every(Duration::from_millis(300)).map(|_| Message::Tick))
        .run_with(|| (gui_app, Task::none()))
}

#[derive(Debug, thiserror::Error)]
#[error("a GUI error occurred")]
pub struct GuiError;

pub struct GuiApp {
    tracker: Box<dyn TimeTracker>,
    start_time: Option<StartTime>,

    tracker_error: Option<Report<TimeTrackerError>>,
    records: Vec<TimeRecord>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    StartTracking,
    StopTracking,
    Tick,
}

impl GuiApp {
    pub fn new(tracker: Box<dyn TimeTracker>) -> Result<Self, GuiError> {
        let start_time = tracker
            .is_tracking()
            .change_context(GuiError)
            .attach_printable("failed to retrieve current start time")?;
        let records = tracker
            .records()
            .change_context(GuiError)
            .attach_printable("failed to retrieve records")?;
        let gui = GuiApp {
            tracker,
            start_time,
            tracker_error: None,
            records,
        };
        Ok(gui)
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::StartTracking => {
                self.tracker_error = None;
                match self.tracker.start() {
                    Ok(start_time) => self.start_time = Some(start_time),
                    Err(e) => self.tracker_error = Some(e),
                }
                self.reload_records();
            }
            Message::StopTracking => {
                self.tracker_error = None;
                match self.tracker.stop() {
                    Ok(_) => self.start_time = None,
                    Err(e) => self.tracker_error = Some(e),
                }
                match self.tracker.records() {
                    Ok(records) => self.records = records,
                    Err(e) => self.tracker_error = Some(e),
                }
            }
            Message::Tick => {}
        }
    }

    fn reload_records(&mut self) {
        match self.tracker.records() {
            Ok(records) => self.records = records,
            Err(e) => self.tracker_error = Some(e),
        }
    }

    #[rustfmt::skip]
    fn view(&self) -> Column<Message> {
        let err_msg = self.err_msg();
        let start_stop_btn = self.start_stop_btn();
        let current_tracking_time = self.current_tracking_time();
        let time_tracked_today = self.time_tracked_today();

        column![
            row![err_msg],
            current_tracking_time,
            start_stop_btn,
            time_tracked_today,
            // ------------
            //  records
        ]
        .padding(20)
    }
}

impl std::fmt::Debug for GuiApp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GuiApp")
            .field("tracker", &"<tracker>")
            .field("time_started", &self.start_time)
            .field("records", &self.records)
            .finish()
    }
}
