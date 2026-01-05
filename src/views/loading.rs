use iced::widget::{column, container, text};
use iced::{Element, Length};

use crate::messages::Message;

pub fn view<'a>() -> Element<'a, Message> {
    container(
        column![
            container(text("Inventory Manager").size(48))
                .width(Length::Fill)
                .center_x(Length::Fill),
            container(text("Loading...").size(24))
                .width(Length::Fill)
                .center_x(Length::Fill),
        ]
        .spacing(20)
        .padding(40),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .style(|_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(iced::Color::from_rgb(
            0.1, 0.1, 0.15,
        ))),
        ..Default::default()
    })
    .into()
}
