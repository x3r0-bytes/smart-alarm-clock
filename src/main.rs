mod app;
mod update;
mod view;

use iced::{Application, Settings, Theme};
use iced::time;
use std::time::Duration;
use crate::app::{Clock, Message};
use chrono;

impl Application for Clock {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, iced::Command<Message>) {
        Clock::new()
    }

    fn title(&self) -> String {
        String::from("Smart Alarm Clock")
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        self.update(message)
    }

    fn view(&self) -> iced::Element<Message> {
        self.view()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        time::every(Duration::from_secs(1)).map(|_| Message::Tick)
    }
}

fn main() -> iced::Result {
    Clock::run(Settings::default())
}