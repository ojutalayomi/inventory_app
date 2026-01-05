use iced::widget::{button, column, container, text};
use iced::{Element, Length};

use crate::messages::Message;

pub fn view<'a>() -> Element<'a, Message> {
    container(
        container(
            column![
                text("Inventory Manager").size(32),
                text("Version 0.1.0").size(18),
                text("").size(14),
                text("A modern desktop application for managing").size(14),
                text("inventory, taking notes, and performing").size(14),
                text("quick calculations.").size(14),
                text("").size(14),
                text("Built with Rust & Iced").size(14),
                text("").size(14),
                button("Close").on_press(Message::CloseAbout).padding(10),
            ]
            .spacing(8)
            .padding(40),
        )
        .width(400)
        .style(|_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgb(
                0.15, 0.15, 0.15,
            ))),
            border: iced::Border {
                color: iced::Color::from_rgb(0.5, 0.5, 0.5),
                width: 2.0,
                radius: 10.0.into(),
            },
            ..Default::default()
        }),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .style(|_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(iced::Color::from_rgba(
            0.0, 0.0, 0.0, 0.8,
        ))),
        ..Default::default()
    })
    .into()
}
