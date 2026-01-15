use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Element, Length};
use crate::Message;
use crate::update_checker::UpdateInfo;
use crate::messages::AppTheme;

pub fn view_clear_confirm<'a>(theme: &'a AppTheme) -> Element<'a, Message> {
    container(
        container(
            column![
                text("Clear All Data?").size(24),
                text("").size(10),
                text("This will permanently delete:").size(14),
                text("• All inventory items").size(13),
                text("• All notes").size(13),
                text("").size(10),
                text("This action cannot be undone!")
                    .size(14)
                    .style(move |_iced_theme: &iced::Theme| {
                        text::Style {
                            color: Some(crate::theme::danger_color(theme)),
                        }
                    }),
                text("").size(20),
                row![
                    button("Yes, Delete Everything")
                        .on_press(Message::ConfirmClearAllData)
                        .padding(10)
                        .style(move |_iced_theme: &iced::Theme, _status: button::Status| {
                            button::Style {
                                background: Some(iced::Background::Color(
                                    crate::theme::danger_color(theme),
                                )),
                                text_color: iced::Color::WHITE,
                                border: iced::Border {
                                    radius: 5.0.into(),
                                    ..Default::default()
                                },
                                ..Default::default()
                            }
                        }),
                    button("Cancel")
                        .on_press(Message::CancelClearAllData)
                        .padding(10)
                        .style(move |_iced_theme: &iced::Theme, _status: button::Status| {
                            button::Style {
                                background: Some(iced::Background::Color(
                                    crate::theme::surface_elevated_color(theme),
                                )),
                                text_color: iced::Color::WHITE,
                                border: iced::Border {
                                    radius: 5.0.into(),
                                    ..Default::default()
                                },
                                ..Default::default()
                            }
                        }),
                ]
                .spacing(10),
            ]
            .spacing(5)
            .padding(30),
        )
        .width(400)
        .style(move |_iced_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgba(
                crate::theme::surface_color(theme).r,
                crate::theme::surface_color(theme).g,
                crate::theme::surface_color(theme).b,
                0.7,
            ))),
            border: iced::Border {
                color: crate::theme::border_color(theme),
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
    .style(move |_iced_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(iced::Color::from_rgba(
            0.0, 0.0, 0.0, 0.7,
        ))),
        ..Default::default()
    })
    .into()
}

pub fn view_update_notification<'a>(update_info: &'a UpdateInfo, theme: &'a AppTheme) -> Element<'a, Message> {
    let release_notes = if update_info.release_notes.len() > 300 {
        format!("{}...", &update_info.release_notes[..300])
    } else {
        update_info.release_notes.clone()
    };
    
    let version = update_info.version.clone();
    let download_url = update_info.download_url.clone();
    
    container(
        container(
            column![
                text("Update Available").size(28).style(move |_iced_theme: &iced::Theme| {
                    text::Style {
                        color: Some(iced::Color::from_rgb(0.3, 0.8, 0.4)),
                    }
                }),
                text("").size(10),
                text(format!("New Version: {}", version)).size(20),
                text(format!("Current Version: v{}", env!("CARGO_PKG_VERSION"))).size(14)
                    .style(move |_iced_theme: &iced::Theme| {
                        text::Style {
                            color: Some(iced::Color::from_rgb(0.7, 0.7, 0.7)),
                        }
                    }),
                text("").size(10),
                text("What's New:").size(16),
                scrollable(
                    text(release_notes)
                        .size(13)
                        .style(move |_iced_theme: &iced::Theme| {
                            text::Style {
                                color: Some(iced::Color::from_rgb(0.8, 0.8, 0.8)),
                            }
                        })
                ).height(150),
                text("").size(15),
                row![
                    button("Update Now")
                        .on_press(Message::DownloadUpdate(download_url))
                        .padding(12)
                        .style(move |_iced_theme: &iced::Theme, _status: button::Status| {
                            button::Style {
                                background: Some(iced::Background::Color(
                                    iced::Color::from_rgb(0.2, 0.6, 0.3),
                                )),
                                text_color: iced::Color::WHITE,
                                border: iced::Border {
                                    radius: 5.0.into(),
                                    ..Default::default()
                                },
                                ..Default::default()
                            }
                        }),
                    button("Later")
                        .on_press(Message::CloseUpdateNotification)
                        .padding(12)
                        .style(move |_iced_theme: &iced::Theme, _status: button::Status| {
                            button::Style {
                                background: Some(iced::Background::Color(
                                    crate::theme::surface_elevated_color(theme),
                                )),
                                text_color: iced::Color::WHITE,
                                border: iced::Border {
                                    radius: 5.0.into(),
                                    ..Default::default()
                                },
                                ..Default::default()
                            }
                        }),
                ]
                .spacing(15),
            ]
            .spacing(5)
            .padding(30),
        )
        .width(500)
        .style(move |_iced_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgb(
                0.15, 0.15, 0.15,
            ))),
            border: iced::Border {
                color: crate::theme::success_color(theme),
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
    .style(move |_iced_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(iced::Color::from_rgba(
            0.0, 0.0, 0.0, 0.7,
        ))),
        ..Default::default()
    })
    .into()
}

