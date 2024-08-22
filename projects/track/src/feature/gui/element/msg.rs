use iced::widget::{text, Text};

use crate::feature::gui::GuiApp;

impl GuiApp {
    pub fn err_msg(&self) -> Text {
        if let Some(err) = &self.tracker_error {
            text(format!("{err}")).style(text::danger)
        } else {
            text("")
        }
    }
}
