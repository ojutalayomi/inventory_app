use iced::{Element, Length};
use iced::widget::{button, checkbox, column, container, row, scrollable, text, text_input};

use crate::messages::{AppSettings, AppTheme, Message};

pub fn view<'a>(
    settings: &'a AppSettings,
    interval_input: &'a str,
    category_input: &'a str,
    latest_version: Option<&'a crate::update_checker::UpdateInfo>,
) -> Element<'a, Message> {
    let title = text("Settings").size(32);

    let auto_save_section = column![
        text("Auto-Save").size(20).style(|_theme: &iced::Theme| {
            iced::widget::text::Style {
                color: Some(iced::Color::from_rgb(0.7, 0.8, 0.9)),
            }
        }),
        checkbox("Enable auto-save", settings.auto_save_enabled)
            .on_toggle(|_| Message::ToggleAutoSave),
        row![
            text("Save interval (seconds):").size(14),
            text_input("5", interval_input)
                .on_input(Message::AutoSaveIntervalChanged)
                .width(100)
                .padding(5),
        ]
        .spacing(10)
        .align_y(iced::Alignment::Center),
        text("Note: Changes are saved automatically when auto-save is enabled")
            .size(12)
            .style(|_theme: &iced::Theme| {
                iced::widget::text::Style {
                    color: Some(iced::Color::from_rgb(0.5, 0.5, 0.5)),
                }
            }),
    ]
    .spacing(10)
    .padding(20);

    let inventory_section = column![
        text("Inventory").size(20).style(|_theme: &iced::Theme| {
            iced::widget::text::Style {
                color: Some(iced::Color::from_rgb(0.7, 0.8, 0.9)),
            }
        }),
        row![
            text("Default category:").size(14),
            text_input("General", category_input)
                .on_input(Message::DefaultCategoryChanged)
                .width(200)
                .padding(5),
        ]
        .spacing(10)
        .align_y(iced::Alignment::Center),
        text("This category will be pre-filled when adding new items")
            .size(12)
            .style(|_theme: &iced::Theme| {
                iced::widget::text::Style {
                    color: Some(iced::Color::from_rgb(0.5, 0.5, 0.5)),
                }
            }),
    ]
    .spacing(10)
    .padding(20);

    let appearance_section = column![
        text("Appearance").size(20).style(|_theme: &iced::Theme| {
            iced::widget::text::Style {
                color: Some(iced::Color::from_rgb(0.7, 0.8, 0.9)),
            }
        }),
        row![
            text("Theme:").size(14),
            button(if settings.theme == AppTheme::Dark {
                "Dark (Current)"
            } else {
                "Dark"
            })
            .on_press(Message::ThemeChanged(AppTheme::Dark))
            .padding(8)
            .style(
                |_theme: &iced::Theme, _status: iced::widget::button::Status| {
                    iced::widget::button::Style {
                        background: if settings.theme == AppTheme::Dark {
                            Some(iced::Background::Color(iced::Color::from_rgb(
                                0.2, 0.4, 0.6,
                            )))
                        } else {
                            Some(iced::Background::Color(iced::Color::from_rgb(
                                0.25, 0.25, 0.25,
                            )))
                        },
                        text_color: iced::Color::WHITE,
                        border: iced::Border {
                            radius: 5.0.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                }
            ),
            button(if settings.theme == AppTheme::Light {
                "Light (Current)"
            } else {
                "Light"
            })
            .on_press(Message::ThemeChanged(AppTheme::Light))
            .padding(8)
            .style(
                |_theme: &iced::Theme, _status: iced::widget::button::Status| {
                    iced::widget::button::Style {
                        background: if settings.theme == AppTheme::Light {
                            Some(iced::Background::Color(iced::Color::from_rgb(
                                0.2, 0.4, 0.6,
                            )))
                        } else {
                            Some(iced::Background::Color(iced::Color::from_rgb(
                                0.25, 0.25, 0.25,
                            )))
                        },
                        text_color: iced::Color::WHITE,
                        border: iced::Border {
                            radius: 5.0.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                }
            ),
        ]
        .spacing(10)
        .align_y(iced::Alignment::Center),
        checkbox(
            "Show loading screen on startup",
            settings.show_loading_screen
        )
        .on_toggle(|_| Message::ToggleLoadingScreen),
        text("Theme changes apply immediately")
            .size(12)
            .style(|_theme: &iced::Theme| {
                iced::widget::text::Style {
                    color: Some(iced::Color::from_rgb(0.5, 0.5, 0.5)),
                }
            }),
    ]
    .spacing(10)
    .padding(20);

    let data_section = column![
        text("Data Management")
            .size(20)
            .style(|_theme: &iced::Theme| {
                iced::widget::text::Style {
                    color: Some(iced::Color::from_rgb(0.7, 0.8, 0.9)),
                }
            }),
        row![
            button("Export Data")
                .on_press(Message::ExportData)
                .padding(10)
                .style(
                    |_theme: &iced::Theme, _status: iced::widget::button::Status| {
                        iced::widget::button::Style {
                            background: Some(iced::Background::Color(iced::Color::from_rgb(
                                0.2, 0.5, 0.3,
                            ))),
                            text_color: iced::Color::WHITE,
                            border: iced::Border {
                                radius: 5.0.into(),
                                ..Default::default()
                            },
                            ..Default::default()
                        }
                    }
                ),
            button("Import Data")
                .on_press(Message::ImportData)
                .padding(10)
                .style(
                    |_theme: &iced::Theme, _status: iced::widget::button::Status| {
                        iced::widget::button::Style {
                            background: Some(iced::Background::Color(iced::Color::from_rgb(
                                0.4, 0.4, 0.6,
                            ))),
                            text_color: iced::Color::WHITE,
                            border: iced::Border {
                                radius: 5.0.into(),
                                ..Default::default()
                            },
                            ..Default::default()
                        }
                    }
                ),
        ]
        .spacing(10),
        text("Export saves data to Desktop as JSON file")
            .size(12)
            .style(|_theme: &iced::Theme| {
                iced::widget::text::Style {
                    color: Some(iced::Color::from_rgb(0.5, 0.5, 0.5)),
                }
            }),
        text("Import reads from ~/Downloads/inventory_import.json")
            .size(12)
            .style(|_theme: &iced::Theme| {
                iced::widget::text::Style {
                    color: Some(iced::Color::from_rgb(0.5, 0.5, 0.5)),
                }
            }),
        text("").size(10),
        button("Clear All Data")
            .on_press(Message::ClearAllData)
            .padding(10)
            .style(
                |_theme: &iced::Theme, _status: iced::widget::button::Status| {
                    iced::widget::button::Style {
                        background: Some(iced::Background::Color(iced::Color::from_rgb(
                            0.7, 0.2, 0.2,
                        ))),
                        text_color: iced::Color::WHITE,
                        border: iced::Border {
                            radius: 5.0.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                }
            ),
        text("Caution: You will be asked to confirm before deletion")
            .size(12)
            .style(|_theme: &iced::Theme| {
                iced::widget::text::Style {
                    color: Some(iced::Color::from_rgb(0.8, 0.6, 0.3)),
                }
            }),
    ]
    .spacing(10)
    .padding(20);

    let current_version = env!("CARGO_PKG_VERSION");
    
    let update_section = column![
        text("Updates").size(20).style(|_theme: &iced::Theme| {
            iced::widget::text::Style {
                color: Some(iced::Color::from_rgb(0.7, 0.8, 0.9)),
            }
        }),
        row![
            text(format!("Current Version: v{}", current_version)).size(14),
            match latest_version {
                Some(update) => {
                    button(text(format!("🔔 Update Available: {}", update.version)))
                        .on_press(Message::DownloadUpdate(update.download_url.clone()))
                        .padding(8)
                        .style(
                            |_theme: &iced::Theme, _status: iced::widget::button::Status| {
                            iced::widget::button::Style {
                                background: Some(iced::Background::Color(iced::Color::from_rgb(
                                    0.2, 0.6, 0.3,
                                ))),
                                text_color: iced::Color::WHITE,
                                border: iced::Border {
                                    radius: 5.0.into(),
                                    ..Default::default()
                                },
                                ..Default::default()
                            }
                        }
                    )
                }
                None => {
                    button("Check for Updates")
                        .on_press(Message::CheckForUpdates)
                        .padding(8)
                }
            },
        ]
        .spacing(10)
        .align_y(iced::Alignment::Center),
        text("Checks GitHub for new releases")
            .size(12)
            .style(|_theme: &iced::Theme| {
                iced::widget::text::Style {
                    color: Some(iced::Color::from_rgb(0.5, 0.5, 0.5)),
                }
            }),
    ]
    .spacing(10)
    .padding(20);

    let info_section = column![
        text("About").size(20).style(|_theme: &iced::Theme| {
            iced::widget::text::Style {
                color: Some(iced::Color::from_rgb(0.7, 0.8, 0.9)),
            }
        }),
        text(format!("Inventory Manager v{}", current_version)).size(14),
        text("Built with Rust & Iced").size(12),
        text("").size(8),
        text("Data Location:")
            .size(12)
            .style(|_theme: &iced::Theme| {
                iced::widget::text::Style {
                    color: Some(iced::Color::from_rgb(0.6, 0.6, 0.6)),
                }
            }),
        text(format!(
            "{}",
            crate::persistence::data_file_path().display()
        ))
        .size(11)
        .style(|_theme: &iced::Theme| {
            iced::widget::text::Style {
                color: Some(iced::Color::from_rgb(0.5, 0.5, 0.5)),
            }
        }),
    ]
    .spacing(5)
    .padding(20);

    let content: Element<'a, Message> = scrollable(
        column![
            title,
            container(auto_save_section).style(|_theme: &iced::Theme| {
                container::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgb(
                        0.12, 0.12, 0.12,
                    ))),
                    border: iced::Border {
                        color: iced::Color::from_rgb(0.2, 0.2, 0.2),
                        width: 1.0,
                        radius: 5.0.into(),
                    },
                    ..Default::default()
                }
            }),
            container(inventory_section).style(|_theme: &iced::Theme| {
                container::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgb(
                        0.12, 0.12, 0.12,
                    ))),
                    border: iced::Border {
                        color: iced::Color::from_rgb(0.2, 0.2, 0.2),
                        width: 1.0,
                        radius: 5.0.into(),
                    },
                    ..Default::default()
                }
            }),
            container(appearance_section).style(|_theme: &iced::Theme| {
                container::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgb(
                        0.12, 0.12, 0.12,
                    ))),
                    border: iced::Border {
                        color: iced::Color::from_rgb(0.2, 0.2, 0.2),
                        width: 1.0,
                        radius: 5.0.into(),
                    },
                    ..Default::default()
                }
            }),
            container(data_section).style(|_theme: &iced::Theme| {
                container::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgb(
                        0.12, 0.12, 0.12,
                    ))),
                    border: iced::Border {
                        color: iced::Color::from_rgb(0.2, 0.2, 0.2),
                        width: 1.0,
                        radius: 5.0.into(),
                    },
                    ..Default::default()
                }
            }),
            container(update_section).style(|_theme: &iced::Theme| {
                container::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgb(
                        0.12, 0.12, 0.12,
                    ))),
                    border: iced::Border {
                        color: iced::Color::from_rgb(0.2, 0.2, 0.2),
                        width: 1.0,
                        radius: 5.0.into(),
                    },
                    ..Default::default()
                }
            }),
            container(info_section).style(|_theme: &iced::Theme| {
                container::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgb(
                        0.12, 0.12, 0.12,
                    ))),
                    border: iced::Border {
                        color: iced::Color::from_rgb(0.2, 0.2, 0.2),
                        width: 1.0,
                        radius: 5.0.into(),
                    },
                    ..Default::default()
                }
            }),
        ].width(Length::Fill)
        .spacing(15)
        .padding(20),
    ).into();

    content
}
