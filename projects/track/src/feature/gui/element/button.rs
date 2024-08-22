use iced::widget::{button, Button};

use crate::feature::gui::{GuiApp, Message};

impl GuiApp {
    pub(in crate::feature::gui) fn start_stop_btn(&self) -> Button<Message> {
        if self.start_time.is_some() {
            button("Stop")
                .on_press(Message::StopTracking)
                .style(button::danger)
        } else {
            button("Start")
                .on_press(Message::StartTracking)
                .style(button::primary)
        }
    }
}
