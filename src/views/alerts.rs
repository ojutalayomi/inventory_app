use iced::widget::{button, checkbox, column, container, row, scrollable, text, text_input};
use iced::{Element, Length};

use crate::alerts::{AlertManager, StockAlert};
use crate::messages::Message;
use crate::user::UserRole;

pub fn view<'a>(
    alert_manager: &'a AlertManager,
    _current_user_role: UserRole,
) -> Element<'a, Message> {
    let title = text("Stock Alerts & Notifications").size(28);

    let settings = alert_manager.settings();
    let active_alerts = alert_manager.get_active_alerts();
    let critical_alerts = alert_manager.get_critical_alerts();

    // Settings Panel
    let enabled_checkbox = checkbox("Enable Stock Alerts", settings.enabled)
        .on_toggle(|_| Message::ToggleAlertsEnabled);

    let notifications_checkbox = checkbox("Show Notifications", settings.show_notifications)
        .on_toggle(|_| Message::ToggleAlertNotifications);

    let low_stock_input = text_input(
        "Low Stock Threshold",
        &settings.low_stock_threshold.to_string(),
    )
    .on_input(Message::AlertLowStockThresholdChanged)
    .padding(8)
    .width(Length::Fixed(100.0));

    let critical_input = text_input(
        "Critical Threshold",
        &settings.critically_low_threshold.to_string(),
    )
    .on_input(Message::AlertCriticalThresholdChanged)
    .padding(8)
    .width(Length::Fixed(100.0));

    let settings_panel = container(
        column![
            text("Alert Settings").size(20),
            text("").size(5),
            enabled_checkbox,
            notifications_checkbox,
            text("").size(10),
            row![
                text("Low Stock Threshold:").size(14).width(Length::Fixed(180.0)),
                low_stock_input,
                text("items").size(12),
            ]
            .spacing(10)
            .align_y(iced::Alignment::Center),
            row![
                text("Critical Threshold:").size(14).width(Length::Fixed(180.0)),
                critical_input,
                text("items").size(12),
            ]
            .spacing(10)
            .align_y(iced::Alignment::Center),
        ]
        .spacing(10)
        .padding(15),
    )
    .style(|_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(iced::Color::from_rgb(
            0.12, 0.12, 0.12,
        ))),
        border: iced::Border {
            color: iced::Color::from_rgb(0.3, 0.3, 0.3),
            width: 1.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    });

    // Summary
    let summary = container(
        row![
            column![
                text("Total Alerts:").size(14),
                text(active_alerts.len()).size(24),
            ]
            .spacing(5)
            .align_x(iced::Alignment::Center),
            column![
                text("Critical:").size(14).style(|_theme: &iced::Theme| {
                    iced::widget::text::Style {
                        color: Some(iced::Color::from_rgb(0.9, 0.3, 0.3)),
                    }
                }),
                text(critical_alerts.len()).size(24).style(|_theme: &iced::Theme| {
                    iced::widget::text::Style {
                        color: Some(iced::Color::from_rgb(0.9, 0.3, 0.3)),
                    }
                }),
            ]
            .spacing(5)
            .align_x(iced::Alignment::Center),
            column![
                text("Unacknowledged:").size(14),
                text(alert_manager.get_unacknowledged_count()).size(24).style(
                    |_theme: &iced::Theme| iced::widget::text::Style {
                        color: Some(iced::Color::from_rgb(0.9, 0.7, 0.3)),
                    }
                ),
            ]
            .spacing(5)
            .align_x(iced::Alignment::Center),
        ]
        .spacing(40)
        .padding(15),
    )
    .style(|_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(iced::Color::from_rgb(
            0.12, 0.12, 0.12,
        ))),
        border: iced::Border {
            color: iced::Color::from_rgb(0.3, 0.3, 0.3),
            width: 1.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    });

    // Action buttons
    let ack_all_button = button("Acknowledge All")
        .on_press(Message::AcknowledgeAllAlerts)
        .padding(8);

    let clear_button = button("Clear Acknowledged")
        .on_press(Message::ClearAcknowledgedAlerts)
        .padding(8)
        .style(|_theme: &iced::Theme, _status: iced::widget::button::Status| {
            iced::widget::button::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb(
                    0.5, 0.3, 0.3,
                ))),
                text_color: iced::Color::WHITE,
                border: iced::Border {
                    radius: 5.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        });

    let actions = row![ack_all_button, clear_button,].spacing(10);

    // Alerts List
    let mut alerts_list = column![].spacing(10);

    if active_alerts.is_empty() {
        alerts_list = alerts_list.push(
            container(
                text("No active alerts")
                    .size(16)
                    .style(|_theme: &iced::Theme| iced::widget::text::Style {
                        color: Some(iced::Color::from_rgb(0.5, 0.5, 0.5)),
                    }),
            )
            .padding(20)
            .width(Length::Fill)
            .center_x(Length::Fill),
        );
    } else {
        for alert in active_alerts {
            alerts_list = alerts_list.push(build_alert_card(alert));
        }
    }

    let content = scrollable(column![
        title,
        text("").size(10),
        settings_panel,
        text("").size(10),
        summary,
        text("").size(10),
        row![text("Active Alerts").size(20), actions,]
            .spacing(20)
            .align_y(iced::Alignment::Center),
        text("").size(5),
        scrollable(alerts_list).height(400),
    ]
    .spacing(5)
    .padding(20));

    content.into()
}

fn build_alert_card<'a>(alert: &'a StockAlert) -> Element<'a, Message> {
    let icon_text = text(alert.alert_type.icon()).size(32);

    let status_badge = container(
        text(format!("{}", alert.alert_type))
            .size(12)
            .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(iced::Color::WHITE),
            }),
    )
    .padding(5)
    .style(move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(alert.alert_type.color())),
        border: iced::Border {
            radius: 3.0.into(),
            ..Default::default()
        },
        ..Default::default()
    });

    let ack_button = if !alert.acknowledged {
        button("Acknowledge")
            .on_press(Message::AcknowledgeAlert(alert.id.clone()))
            .padding(5)
    } else {
        button("✓ Acknowledged")
            .padding(5)
            .style(|_theme: &iced::Theme, _status: iced::widget::button::Status| {
                iced::widget::button::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgb(
                        0.3, 0.6, 0.3,
                    ))),
                    text_color: iced::Color::WHITE,
                    ..Default::default()
                }
            })
    };

    container(
        row![
            icon_text,
            column![
                row![
                    text(&alert.item_name).size(18),
                    status_badge,
                ]
                .spacing(10)
                .align_y(iced::Alignment::Center),
                text(format!("SKU: {}", alert.item_sku)).size(12).style(
                    |_theme: &iced::Theme| iced::widget::text::Style {
                        color: Some(iced::Color::from_rgb(0.7, 0.7, 0.7)),
                    }
                ),
                text("").size(5),
                row![
                    text(format!("Current Stock: {} items", alert.current_quantity)).size(14),
                    text(format!("Threshold: {} items", alert.threshold))
                        .size(12)
                        .style(|_theme: &iced::Theme| iced::widget::text::Style {
                            color: Some(iced::Color::from_rgb(0.6, 0.6, 0.6)),
                        }),
                ]
                .spacing(20),
                text(format!("Detected: {}", alert.formatted_timestamp()))
                    .size(11)
                    .style(|_theme: &iced::Theme| iced::widget::text::Style {
                        color: Some(iced::Color::from_rgb(0.5, 0.5, 0.5)),
                    }),
            ]
            .spacing(5)
            .width(Length::Fill),
            ack_button,
        ]
        .spacing(15)
        .padding(15)
        .align_y(iced::Alignment::Center),
    )
    .style(|_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(iced::Color::from_rgb(
            0.1, 0.1, 0.1,
        ))),
        border: iced::Border {
            color: iced::Color::from_rgb(0.3, 0.3, 0.3),
            width: 1.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    })
    .into()
}

