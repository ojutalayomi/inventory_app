use iced::widget::{button, column, container, row, scrollable, text, markdown};
use iced::{Element, Length};
use crate::Message;
use crate::update_checker::UpdateInfo;
use crate::messages::AppTheme;

pub fn view_clear_confirm<'a>(theme: &'a AppTheme) -> Element<'a, Message> {
    container(
        container(
            column![
                text("Clear All Data?")
                    .size(24)
                    .style(move |_iced_theme: &iced::Theme| {
                        text::Style {
                            color: Some(crate::theme::text_color(theme)),
                        }
                    }),
                text("").size(10),
                text("This will permanently delete:")
                    .size(14)
                    .style(move |_iced_theme: &iced::Theme| {
                        text::Style {
                            color: Some(crate::theme::text_color(theme)),
                        }
                    }),
                text("• All inventory items")
                    .size(13)
                    .style(move |_iced_theme: &iced::Theme| {
                        text::Style {
                            color: Some(crate::theme::text_secondary_color(theme)),
                        }
                    }),
                text("• All notes")
                    .size(13)
                    .style(move |_iced_theme: &iced::Theme| {
                        text::Style {
                            color: Some(crate::theme::text_secondary_color(theme)),
                        }
                    }),
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
                                text_color: crate::theme::text_color(theme),
                                border: iced::Border {
                                    color: crate::theme::border_color(theme),
                                    width: 1.0,
                                    radius: 5.0.into(),
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
            background: Some(iced::Background::Color(
                crate::theme::surface_elevated_color(theme),
            )),
            border: iced::Border {
                color: crate::theme::danger_color(theme),
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
            crate::theme::surface_color(theme).r,
            crate::theme::surface_color(theme).g,
            crate::theme::surface_color(theme).b,
            0.7,
        ))),
        ..Default::default()
    })
    .into()
}

pub fn view_update_notification<'a>(
    update_info: &'a UpdateInfo,
    release_notes_items: Option<&'a [markdown::Item]>,
    theme: &'a AppTheme,
) -> Element<'a, Message> {
    let release_notes_slice = if update_info.release_notes.len() > 2000 {
        &update_info.release_notes[..2000]
    } else {
        &update_info.release_notes
    };
    
    let version = update_info.version.clone();
    let download_url = update_info.download_url.clone();
    
    // Create markdown settings with theme-aware styling
    let markdown_settings = markdown::Settings {
        text_size: iced::Pixels(13.0),
        h1_size: iced::Pixels(20.0),
        h2_size: iced::Pixels(18.0),
        h3_size: iced::Pixels(16.0),
        h4_size: iced::Pixels(14.0),
        h5_size: iced::Pixels(13.0),
        h6_size: iced::Pixels(13.0),
        code_size: iced::Pixels(12.0),
    };
    
    // Create markdown style with theme-aware colors
    let markdown_style = markdown::Style {
        inline_code_highlight: markdown::Highlight {
            background: iced::Background::Color(crate::theme::surface_elevated_color(theme)),
            border: iced::Border::default(),
        },
        inline_code_padding: iced::Padding::from([4.0, 6.0]),
        inline_code_color: crate::theme::primary_color(theme),
        link_color: crate::theme::primary_color(theme),
    };
    
    container(
        container(
            column![
                text("Update Available").size(28).style(move |_iced_theme: &iced::Theme| {
                    text::Style {
                        color: Some(crate::theme::success_color(theme)),
                    }
                }),
                text("").size(10),
                text(format!("New Version: {}", version))
                    .size(20)
                    .style(move |_iced_theme: &iced::Theme| {
                        text::Style {
                            color: Some(crate::theme::text_color(theme)),
                        }
                    }),
                text(format!("Current Version: v{}", env!("CARGO_PKG_VERSION")))
                    .size(14)
                    .style(move |_iced_theme: &iced::Theme| {
                        text::Style {
                            color: Some(crate::theme::text_secondary_color(theme)),
                        }
                    }),
                text("").size(10),
                text("What's New:")
                    .size(16)
                    .style(move |_iced_theme: &iced::Theme| {
                        text::Style {
                            color: Some(crate::theme::text_color(theme)),
                        }
                    }),
                scrollable(
                    if let Some(items) = release_notes_items {
                        markdown::view(items, markdown_settings, markdown_style)
                            .map(|_uri| Message::CloseUpdateNotification)
                    } else {
                        text(release_notes_slice)
                            .size(13)
                            .style(move |_iced_theme: &iced::Theme| {
                                text::Style {
                                    color: Some(crate::theme::text_color(theme)),
                                }
                            })
                            .into()
                    }
                ).height(150),
                text("").size(15),
                row![
                    button("Update Now")
                        .on_press(Message::DownloadUpdate(download_url))
                        .padding(12)
                        .style(move |_iced_theme: &iced::Theme, _status: button::Status| {
                            button::Style {
                                background: Some(iced::Background::Color(
                                    crate::theme::success_color(theme),
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
                                text_color: crate::theme::text_color(theme),
                                border: iced::Border {
                                    color: crate::theme::border_color(theme),
                                    width: 1.0,
                                    radius: 5.0.into(),
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
            background: Some(iced::Background::Color(
                crate::theme::surface_elevated_color(theme),
            )),
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
            crate::theme::surface_color(theme).r,
            crate::theme::surface_color(theme).g,
            crate::theme::surface_color(theme).b,
            0.7,
        ))),
        ..Default::default()
    })
    .into()
}

