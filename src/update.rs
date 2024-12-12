use iced::Command;
use chrono::{DateTime, Local};
use crate::app::{Clock, Message};

impl Clock {
    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick => {
                let time_loc: DateTime<Local> = Local::now();
                self.current_time = time_loc.format("%I:%M:%S %p").to_string();
                self.current_date = time_loc.format("%m-%d-%Y").to_string();
            }
        }
        Command::none()
    }
}