use iced::widget::{button, column, container, row, text, text_input};
use iced::{Color, Element, Length};

use crate::messages::{AppTheme, Message};
use crate::theme;
use crate::icons;

pub fn view<'a>(
    username: &'a str,
    password: &'a str,
    error_message: Option<&'a str>,
    app_theme: &'a AppTheme,
) -> Element<'a, Message> {
    // App title with gradient-like effect using multiple colors
    let title = text("Inventory Manager")
        .size(theme::TEXT_DISPLAY)
        .style(move |_theme: &iced::Theme| text::Style {
            color: Some(theme::primary_color(app_theme)),
        });
    
    let subtitle = text("Welcome back! Please log in to continue")
        .size(theme::TEXT_BODY_LARGE)
        .style(move |_theme: &iced::Theme| text::Style {
            color: Some(theme::text_secondary_color(app_theme)),
        });

    // Username input with icon
    let username_field = column![
        text("Username")
            .size(theme::TEXT_BODY)
            .style(move |_theme: &iced::Theme| text::Style {
                color: Some(theme::text_color(app_theme)),
            }),
        text_input("Enter your username", username)
            .on_input(Message::UsernameChanged)
            .on_submit(Message::AttemptLogin)
            .padding(theme::SPACING_LG)
            .size(theme::TEXT_BODY)
            .style(move |theme_iced: &iced::Theme, status: text_input::Status| {
                let default_style = text_input::default(theme_iced, status);
                text_input::Style {
                    background: iced::Background::Color(theme::surface_elevated_color(app_theme)),
                    border: iced::Border {
                        color: match status {
                            text_input::Status::Focused => theme::primary_color(app_theme),
                            _ => theme::border_color(app_theme),
                        },
                        width: if matches!(status, text_input::Status::Focused) { 2.0 } else { 1.0 },
                        radius: theme::RADIUS_MD.into(),
                    },
                    icon: default_style.icon,
                    placeholder: theme::text_tertiary_color(app_theme),
                    value: theme::text_color(app_theme),
                    selection: theme::primary_color(app_theme),
                }
            }),
    ]
    .spacing(theme::SPACING_SM);

    // Password input with icon
    let password_field = column![
        text("Password")
            .size(theme::TEXT_BODY)
            .style(move |_theme: &iced::Theme| text::Style {
                color: Some(theme::text_color(app_theme)),
            }),
        text_input("Enter your password", password)
            .on_input(Message::PasswordChanged)
            .on_submit(Message::AttemptLogin)
            .secure(true)
            .padding(theme::SPACING_LG)
            .size(theme::TEXT_BODY)
            .style(move |theme_iced: &iced::Theme, status: text_input::Status| {
                let default_style = text_input::default(theme_iced, status);
                text_input::Style {
                    background: iced::Background::Color(theme::surface_elevated_color(app_theme)),
                    border: iced::Border {
                        color: match status {
                            text_input::Status::Focused => theme::primary_color(app_theme),
                            _ => theme::border_color(app_theme),
                        },
                        width: if matches!(status, text_input::Status::Focused) { 2.0 } else { 1.0 },
                        radius: theme::RADIUS_MD.into(),
                    },
                    icon: default_style.icon,
                    placeholder: theme::text_tertiary_color(app_theme),
                    value: theme::text_color(app_theme),
                    selection: theme::primary_color(app_theme),
                }
            }),
    ]
    .spacing(theme::SPACING_SM);

    // Modern gradient login button
    let login_button = button(
        container(
            row![
                icons::Icon::Lock.view(icons::IconSize::Medium, app_theme),
                text("Log In").size(theme::TEXT_BODY_LARGE),
            ]
            .spacing(theme::SPACING_SM)
            .align_y(iced::Alignment::Center)
        )
        .width(Length::Fill)
        .center_x(Length::Fill)
    )
    .on_press(Message::AttemptLogin)
    .padding(theme::SPACING_LG)
    .width(Length::Fill)
    .style(move |_theme: &iced::Theme, status: button::Status| {
        let bg_color = match status {
            button::Status::Hovered => theme::primary_dark_color(app_theme),
            _ => theme::primary_color(app_theme),
        };
        
        button::Style {
            background: Some(iced::Background::Color(bg_color)),
            text_color: Color::WHITE,
            border: iced::Border {
                radius: theme::RADIUS_MD.into(),
                ..Default::default()
            },
            shadow: iced::Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                offset: iced::Vector::new(0.0, 4.0),
                blur_radius: 12.0,
            },
            ..Default::default()
        }
    });

    // Build the login card content
    let mut card_content = column![
        title,
        subtitle,
        text("").size(theme::SPACING_XL),
        username_field,
        password_field,
        text("").size(theme::SPACING_SM),
        login_button,
    ]
    .spacing(theme::SPACING_LG)
    .padding(theme::SPACING_3XL)
    .width(500)
    .align_x(iced::Alignment::Center);

    // Error message if present
    if let Some(error) = error_message {
        card_content = card_content.push(text("").size(theme::SPACING_SM));
        card_content = card_content.push(
            container(
                row![
                    icons::Icon::AlertTriangle.view(icons::IconSize::Medium, app_theme),
                    text(error).size(theme::TEXT_BODY)
                ]
                .spacing(theme::SPACING_SM)
                .align_y(iced::Alignment::Center)
            )
            .padding(theme::SPACING_LG)
            .width(Length::Fill)
            .style(move |_theme: &iced::Theme| container::Style {
                background: Some(iced::Background::Color(
                    Color::from_rgba(
                        theme::danger_color(app_theme).r,
                        theme::danger_color(app_theme).g,
                        theme::danger_color(app_theme).b,
                        0.2,
                    )
                )),
                border: iced::Border {
                    color: theme::danger_color(app_theme),
                    width: 1.0,
                    radius: theme::RADIUS_MD.into(),
                },
                ..Default::default()
            })
        );
    }

    // Helper text
    card_content = card_content.push(text("").size(theme::SPACING_LG));
    card_content = card_content.push(
        container(
            column![
                row![
                    icons::Icon::Lightbulb.view(icons::IconSize::Small, app_theme),
                    text("Quick Start")
                        .size(theme::TEXT_CAPTION)
                        .style(move |_theme: &iced::Theme| text::Style {
                            color: Some(theme::text_secondary_color(app_theme)),
                        }),
                ]
                .spacing(theme::SPACING_XS)
                .align_y(iced::Alignment::Center),
                text("Default credentials: admin / admin123")
                    .size(theme::TEXT_CAPTION)
                    .style(move |_theme: &iced::Theme| text::Style {
                        color: Some(theme::text_tertiary_color(app_theme)),
                    }),
            ]
            .spacing(theme::SPACING_XS)
        )
        .padding(theme::SPACING_LG)
        .width(Length::Fill)
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(theme::surface_elevated_color(app_theme))),
            border: iced::Border {
                color: theme::border_color(app_theme),
                width: 1.0,
                radius: theme::RADIUS_MD.into(),
            },
            ..Default::default()
        })
    );

    // Modern login card with gradient border effect
    let login_card = container(card_content)
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(theme::surface_color(app_theme))),
            border: iced::Border {
                color: theme::primary_color(app_theme),
                width: 2.0,
                radius: theme::RADIUS_LG.into(),
            },
            shadow: iced::Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                offset: iced::Vector::new(0.0, 12.0),
                blur_radius: 36.0,
            },
            ..Default::default()
        });

    // Full screen container with gradient background
    container(login_card)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(theme::bg_color(app_theme))),
            ..Default::default()
        })
        .into()
}
