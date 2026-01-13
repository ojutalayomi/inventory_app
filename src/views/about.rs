use iced::widget::{button, column, container, text};
use iced::{Color, Element, Length};

use crate::messages::{AppTheme, Message};
use crate::theme;
use crate::icons;

pub fn view<'a>(app_theme: &'a AppTheme) -> Element<'a, Message> {
    let close_button = button(
        text("✕ Close")
            .size(theme::TEXT_BODY)
    )
    .on_press(Message::CloseAbout)
    .padding([theme::SPACING_MD, theme::SPACING_XL])
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
            ..Default::default()
        }
    });

    let current_version = env!("CARGO_PKG_VERSION");

    container(
        container(
            column![
                icons::Icon::Box.view(icons::IconSize::XLarge, app_theme),
                text("Inventory Manager")
                    .size(theme::TEXT_H1)
                    .style(move |_theme: &iced::Theme| text::Style {
                        color: Some(theme::primary_color(app_theme)),
                    }),
                text(format!("Version {}", current_version))
                    .size(theme::TEXT_H3)
                    .style(move |_theme: &iced::Theme| text::Style {
                        color: Some(theme::text_secondary_color(app_theme)),
                    }),
                text("").size(theme::SPACING_LG),
                container(
                    column![
                        text("A modern desktop application for managing")
                            .size(theme::TEXT_BODY),
                        text("inventory, taking notes, and performing")
                            .size(theme::TEXT_BODY),
                        text("quick calculations.")
                            .size(theme::TEXT_BODY),
                    ]
                    .spacing(theme::SPACING_XS)
                    .align_x(iced::Alignment::Center)
                )
                .style(move |_theme: &iced::Theme| container::Style {
                    text_color: Some(theme::text_color(app_theme)),
                    ..Default::default()
                }),
                text("").size(theme::SPACING_LG),
                container(
                    text("Built with Rust & Iced")
                        .size(theme::TEXT_BODY)
                        .style(move |_theme: &iced::Theme| text::Style {
                            color: Some(theme::accent_color(app_theme)),
                        })
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
                }),
                text("").size(theme::SPACING_LG),
                close_button,
            ]
            .spacing(theme::SPACING_SM)
            .padding(theme::SPACING_3XL)
            .align_x(iced::Alignment::Center),
        )
        .width(500)
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
        }),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .style(move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.7))),
        ..Default::default()
    })
    .into()
}
