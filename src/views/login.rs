use iced::widget::{button, column, container, text, text_input};
use iced::{Element, Length};

use crate::messages::Message;

pub fn view<'a>(
    username: &'a str,
    password: &'a str,
    error_message: Option<&'a str>,
) -> Element<'a, Message> {
    let title = text("Inventory Manager").size(32);
    let subtitle = text("Please log in to continue")
        .size(16)
        .style(|_theme: &iced::Theme| iced::widget::text::Style {
            color: Some(iced::Color::from_rgb(0.7, 0.7, 0.7)),
        });

    let username_input = column![
        text("Username:").size(14),
        text_input("Enter username", username)
            .on_input(Message::UsernameChanged)
            .on_submit(Message::AttemptLogin)
            .padding(10),
    ]
    .spacing(5);

    let password_input = column![
        text("Password:").size(14),
        text_input("Enter password", password)
            .on_input(Message::PasswordChanged)
            .on_submit(Message::AttemptLogin)
            .secure(true)
            .padding(10),
    ]
    .spacing(5);

    let login_button = button(
        container(text("Log In").size(16))
            .width(Length::Fill)
            .center_x(Length::Fill),
    )
    .on_press(Message::AttemptLogin)
    .padding(12)
    .width(Length::Fill)
    .style(
        |theme: &iced::Theme, status: iced::widget::button::Status| match status {
            iced::widget::button::Status::Active => iced::widget::button::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb(
                    0.2, 0.5, 0.8,
                ))),
                text_color: iced::Color::WHITE,
                border: iced::Border {
                    radius: 5.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            },
            iced::widget::button::Status::Hovered => iced::widget::button::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb(
                    0.3, 0.6, 0.9,
                ))),
                text_color: iced::Color::WHITE,
                border: iced::Border {
                    radius: 5.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            },
            _ => button::primary(theme, status),
        },
    );

    let mut content = column![
        title,
        subtitle,
        text("").size(20),
        username_input,
        password_input,
        text("").size(10),
        login_button,
    ]
    .spacing(10)
    .padding(40)
    .width(400);

    if let Some(error) = error_message {
        content = content.push(text("").size(10));
        content = content.push(text(error).size(14).style(|_theme: &iced::Theme| {
            iced::widget::text::Style {
                color: Some(iced::Color::from_rgb(0.9, 0.3, 0.3)),
            }
        }));
    }

    content = content.push(text("").size(20));
    content = content.push(
        text("Default credentials: admin / admin123")
            .size(12)
            .style(|_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(iced::Color::from_rgb(0.5, 0.5, 0.5)),
            }),
    );

    container(
        container(content).style(|_theme: &iced::Theme| container::Style {
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
    .into()
}
