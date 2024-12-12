use iced::widget::{column, text, container};
use iced::{Element, Length, alignment::Horizontal};
use crate::app::{Clock, Message};

impl Clock {
    pub fn view(&self) -> Element<Message> {
        container(
            column![
                text(format!("{}", self.current_date))
                    .size(20)
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Center),
                text(format!("{}", self.current_time))
                    .size(40)
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Center)
            ]
            .spacing(20)
            .width(Length::Fill)
            .height(Length::Fill)
        )
        .center_x()
        .center_y()
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}