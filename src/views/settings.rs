use iced::{Element, Length};
use iced::widget::{button, checkbox, column, container, row, scrollable, text, text_input};

use crate::messages::{AppSettings, AppTheme, LayoutStyle, Message};
use crate::theme;
use crate::icons;

pub fn view<'a>(
    settings: &'a AppSettings,
    interval_input: &'a str,
    category_input: &'a str,
    notification_throttle_input: &'a str,
    latest_version: Option<&'a crate::update_checker::UpdateInfo>,
    import_error: Option<&'a str>,
    checking_for_updates: bool,
    downloading_update: bool,
    update_message: Option<&'a str>,
    theme: &'a crate::messages::AppTheme,
) -> Element<'a, Message> {
    let title = text("Settings").size(32);

    let auto_save_section = column![
        text("Auto-Save").size(20).style(move |_iced_theme: &iced::Theme| {
            iced::widget::text::Style {
                color: Some(crate::theme::text_color(theme)),
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
            .style(move |_iced_theme: &iced::Theme| {
                iced::widget::text::Style {
                    color: Some(crate::theme::text_secondary_color(theme)),
                }
            }),
    ]
    .spacing(10)
    .padding(20);

    let inventory_section = column![
        text("Inventory").size(20).style(move |_iced_theme: &iced::Theme| {
            iced::widget::text::Style {
                color: Some(crate::theme::text_color(theme)),
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
            .style(move |_iced_theme: &iced::Theme| {
                iced::widget::text::Style {
                    color: Some(crate::theme::text_secondary_color(theme)),
                }
            }),
    ]
    .spacing(10)
    .padding(20);

    let appearance_section = column![
        text("Appearance").size(20).style(move |_iced_theme: &iced::Theme| {
            iced::widget::text::Style {
                color: Some(crate::theme::text_color(theme)),
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
                move |_theme: &iced::Theme, _status: iced::widget::button::Status| {
                    iced::widget::button::Style {
                        background: if settings.theme == AppTheme::Dark {
                            Some(iced::Background::Color(crate::theme::primary_color(theme)))
                        } else {
                            Some(iced::Background::Color(crate::theme::surface_elevated_color(theme)))
                        },
                        text_color: if settings.theme == AppTheme::Dark {
                            iced::Color::WHITE
                        } else {
                            crate::theme::text_color(theme)
                        },
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
                move |_theme: &iced::Theme, _status: iced::widget::button::Status| {
                    iced::widget::button::Style {
                        background: if settings.theme == AppTheme::Light {
                            Some(iced::Background::Color(crate::theme::primary_color(theme)))
                        } else {
                            Some(iced::Background::Color(crate::theme::surface_elevated_color(theme)))
                        },
                        text_color: if settings.theme == AppTheme::Light {
                            iced::Color::WHITE
                        } else {
                            crate::theme::text_color(theme)
                        },
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
        text("").size(8), // Spacer
        row![
            text("Layout:").size(14),
            button(if settings.layout_style == LayoutStyle::Header {
                "Header (Current)"
            } else {
                "Header"
            })
            .on_press(Message::LayoutStyleChanged(LayoutStyle::Header))
            .padding(8)
            .style(
                move |_theme: &iced::Theme, _status: iced::widget::button::Status| {
                    iced::widget::button::Style {
                        background: if settings.layout_style == LayoutStyle::Header {
                            Some(iced::Background::Color(crate::theme::primary_color(theme)))
                        } else {
                            Some(iced::Background::Color(crate::theme::surface_elevated_color(theme)))
                        },
                        text_color: if settings.layout_style == LayoutStyle::Header {
                            iced::Color::WHITE
                        } else {
                            crate::theme::text_color(theme)
                        },
                        border: iced::Border {
                            radius: 5.0.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                }
            ),
            button(if settings.layout_style == LayoutStyle::Sidebar {
                "Sidebar (Current)"
            } else {
                "Sidebar"
            })
            .on_press(Message::LayoutStyleChanged(LayoutStyle::Sidebar))
            .padding(8)
            .style(
                move |_theme: &iced::Theme, _status: iced::widget::button::Status| {
                    iced::widget::button::Style {
                        background: if settings.layout_style == LayoutStyle::Sidebar {
                            Some(iced::Background::Color(crate::theme::primary_color(theme)))
                        } else {
                            Some(iced::Background::Color(crate::theme::surface_elevated_color(theme)))
                        },
                        text_color: if settings.layout_style == LayoutStyle::Sidebar {
                            iced::Color::WHITE
                        } else {
                            crate::theme::text_color(theme)
                        },
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
        text("Layout changes apply immediately")
            .size(12)
            .style(move |_iced_theme: &iced::Theme| {
                iced::widget::text::Style {
                    color: Some(crate::theme::text_secondary_color(theme)),
                }
            }),
        checkbox(
            "Show loading screen on startup",
            settings.show_loading_screen
        )
        .on_toggle(|_| Message::ToggleLoadingScreen),
        text("Theme changes apply immediately")
            .size(12)
            .style(move |_iced_theme: &iced::Theme| {
                iced::widget::text::Style {
                    color: Some(crate::theme::text_secondary_color(theme)),
                }
            }),
    ]
    .spacing(10)
    .padding(20);

    let notifications_section = column![
        text("Notifications").size(20).style(move |_iced_theme: &iced::Theme| {
            iced::widget::text::Style {
                color: Some(crate::theme::text_color(theme)),
            }
        }),
        checkbox(
            "Enable device notifications",
            settings.device_notifications_enabled,
        )
        .on_toggle(|_| Message::ToggleDeviceNotifications),
        checkbox(
            "Notify when updates are available",
            settings.update_notifications_enabled,
        )
        .on_toggle(|_| Message::ToggleUpdateNotifications),
        row![
            text("Notification throttle (seconds):").size(14),
            text_input("30", notification_throttle_input)
                .on_input(Message::NotificationThrottleChanged)
                .width(100)
                .padding(5),
        ]
        .spacing(10)
        .align_y(iced::Alignment::Center),
        text("Used to avoid repeating the same alert too often")
            .size(12)
            .style(move |_iced_theme: &iced::Theme| {
                iced::widget::text::Style {
                    color: Some(crate::theme::text_secondary_color(theme)),
                }
            }),
    ]
    .spacing(10)
    .padding(20);

    let data_section = column![
        text("Data Management")
            .size(20)
            .style(move |_iced_theme: &iced::Theme| {
                iced::widget::text::Style {
                    color: Some(crate::theme::text_color(theme)),
                }
            }),
        row![
            button("Export Data")
                .on_press(Message::ExportData)
                .padding(10)
                .style(
                    move |_theme: &iced::Theme, _status: iced::widget::button::Status| {
                        iced::widget::button::Style {
                            background: Some(iced::Background::Color(crate::theme::success_color(theme))),
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
                .on_press(Message::OpenImportFilePicker)
                .padding(10)
                .style(
                    move |_theme: &iced::Theme, _status: iced::widget::button::Status| {
                        iced::widget::button::Style {
                            background: Some(iced::Background::Color(crate::theme::primary_color(theme))),
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
            .style(move |_iced_theme: &iced::Theme| {
                iced::widget::text::Style {
                    color: Some(crate::theme::text_secondary_color(theme)),
                }
            }),
        text("Import opens a file picker to select a JSON file")
            .size(12)
            .style(move |_iced_theme: &iced::Theme| {
                iced::widget::text::Style {
                    color: Some(crate::theme::text_secondary_color(theme)),
                }
            }),
        if let Some(error) = import_error {
            text(format!("Import error: {}", error))
                .size(12)
                .style(move |_iced_theme: &iced::Theme| {
                    iced::widget::text::Style {
                        color: Some(crate::theme::danger_color(theme)),
                    }
                })
        } else {
            text("").size(1)
        },
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
            .style(move |_iced_theme: &iced::Theme| {
                iced::widget::text::Style {
                    color: Some(crate::theme::warning_color(theme)),
                }
            }),
    ]
    .spacing(10)
    .padding(20);

    let current_version = env!("CARGO_PKG_VERSION");
    
    let update_section = column![
        text("Updates").size(20).style(move |_iced_theme: &iced::Theme| {
            iced::widget::text::Style {
                color: Some(iced::Color::from_rgb(0.7, 0.8, 0.9)),
            }
        }),
        row![
            text(format!("Current Version: v{}", current_version)).size(14),
            match latest_version {
                Some(update) => {
                    if downloading_update {
                        button(
                            row![
                                text("Downloading...").size(14),
                            ]
                            .spacing(theme::SPACING_SM)
                            .align_y(iced::Alignment::Center)
                        )
                            .padding(8)
                            .style(
                                |_theme: &iced::Theme, _status: iced::widget::button::Status| {
                                iced::widget::button::Style {
                                    background: Some(iced::Background::Color(iced::Color::from_rgb(
                                        0.5, 0.5, 0.5,
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
                    } else {
                        button(
                            row![
                                icons::Icon::Alerts.view(icons::IconSize::Small, theme),
                                text(format!("Update Available: {}", update.version)),
                            ]
                            .spacing(theme::SPACING_SM)
                            .align_y(iced::Alignment::Center)
                        )
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
                }
                None => {
                    if checking_for_updates {
                        button(
                            row![
                                text("Checking...").size(14),
                            ]
                            .spacing(theme::SPACING_SM)
                            .align_y(iced::Alignment::Center)
                        )
                            .padding(8)
                            .style(
                                |_theme: &iced::Theme, _status: iced::widget::button::Status| {
                                iced::widget::button::Style {
                                    background: Some(iced::Background::Color(iced::Color::from_rgb(
                                        0.5, 0.5, 0.5,
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
                    } else {
                        button("Check for Updates")
                            .on_press(Message::CheckForUpdates)
                            .padding(8)
                    }
                }
            },
        ]
        .spacing(10)
        .align_y(iced::Alignment::Center),
        if let Some(msg) = update_message {
            text(msg)
                .size(12)
                .style(move |_iced_theme: &iced::Theme| {
                    iced::widget::text::Style {
                        color: Some(if checking_for_updates || downloading_update {
                            crate::theme::primary_color(theme)
                        } else if msg.contains("Failed") || msg.contains("error") {
                            crate::theme::danger_color(theme)
                        } else {
                            crate::theme::success_color(theme)
                        }),
                    }
                })
        } else {
            text("Checks GitHub for new releases")
                .size(12)
                .style(move |_iced_theme: &iced::Theme| {
                    iced::widget::text::Style {
                        color: Some(crate::theme::text_secondary_color(theme)),
                    }
                })
        },
    ]
    .spacing(10)
    .padding(20);

    let info_section = column![
        text("About").size(20).style(move |_iced_theme: &iced::Theme| {
            iced::widget::text::Style {
                color: Some(iced::Color::from_rgb(0.7, 0.8, 0.9)),
            }
        }),
        text(format!("Inventory Manager v{}", current_version)).size(14),
        text("Built with Rust & Iced").size(12),
        text("").size(8),
        text("Data Location:")
            .size(12)
            .style(move |_iced_theme: &iced::Theme| {
                iced::widget::text::Style {
                    color: Some(crate::theme::text_secondary_color(theme)),
                }
            }),
        text(format!(
            "{}",
            crate::persistence::data_file_path().display()
        ))
        .size(11)
        .style(move |_iced_theme: &iced::Theme| {
            iced::widget::text::Style {
                color: Some(crate::theme::text_secondary_color(theme)),
            }
        }),
    ]
    .spacing(5)
    .padding(20);

    let content: Element<'a, Message> = scrollable(
        column![
            title,
            container(auto_save_section).style(move |_iced_theme: &iced::Theme| {
                container::Style {
                    background: Some(iced::Background::Color(crate::theme::surface_color(theme))),
                    border: iced::Border {
                        color: crate::theme::border_color(theme),
                        width: 1.0,
                        radius: 5.0.into(),
                    },
                    ..Default::default()
                }
            }),
            container(inventory_section).style(move |_iced_theme: &iced::Theme| {
                container::Style {
                    background: Some(iced::Background::Color(crate::theme::surface_color(theme))),
                    border: iced::Border {
                        color: crate::theme::border_color(theme),
                        width: 1.0,
                        radius: 5.0.into(),
                    },
                    ..Default::default()
                }
            }),
            container(appearance_section).style(move |_iced_theme: &iced::Theme| {
                container::Style {
                    background: Some(iced::Background::Color(crate::theme::surface_color(theme))),
                    border: iced::Border {
                        color: crate::theme::border_color(theme),
                        width: 1.0,
                        radius: 5.0.into(),
                    },
                    ..Default::default()
                }
            }),
            container(notifications_section).style(move |_iced_theme: &iced::Theme| {
                container::Style {
                    background: Some(iced::Background::Color(crate::theme::surface_color(theme))),
                    border: iced::Border {
                        color: crate::theme::border_color(theme),
                        width: 1.0,
                        radius: 5.0.into(),
                    },
                    ..Default::default()
                }
            }),
            container(data_section).style(move |_iced_theme: &iced::Theme| {
                container::Style {
                    background: Some(iced::Background::Color(crate::theme::surface_color(theme))),
                    border: iced::Border {
                        color: crate::theme::border_color(theme),
                        width: 1.0,
                        radius: 5.0.into(),
                    },
                    ..Default::default()
                }
            }),
            container(update_section).style(move |_iced_theme: &iced::Theme| {
                container::Style {
                    background: Some(iced::Background::Color(crate::theme::surface_color(theme))),
                    border: iced::Border {
                        color: crate::theme::border_color(theme),
                        width: 1.0,
                        radius: 5.0.into(),
                    },
                    ..Default::default()
                }
            }),
            container(info_section).style(move |_iced_theme: &iced::Theme| {
                container::Style {
                    background: Some(iced::Background::Color(crate::theme::surface_color(theme))),
                    border: iced::Border {
                        color: crate::theme::border_color(theme),
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
