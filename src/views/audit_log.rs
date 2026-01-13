use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Element, Length};

use crate::audit::{AuditAction, AuditEntry};
use crate::messages::Message;
use crate::user::UserRole;

pub fn view<'a>(entries: &[&'a AuditEntry], current_user_role: UserRole, theme: &'a crate::messages::AppTheme) -> Element<'a, Message> {
    if !current_user_role.can_view_audit() {
        return container(
            text("Access Denied: Manager or Admin privileges required")
                .size(20)
                .style(move |_iced_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(crate::theme::danger_color(theme)),
                }),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into();
    }

    let title = text("Audit Log").size(28);

    let export_button = button("Export to CSV")
        .on_press(Message::ExportAuditLog)
        .padding(10);

    let header_row = row![title, export_button,]
        .spacing(20)
        .align_y(iced::Alignment::Center);

    // Table header
    let table_header = container(
        row![
            text("Timestamp").width(Length::FillPortion(2)),
            text("User").width(Length::FillPortion(2)),
            text("Action").width(Length::FillPortion(2)),
            text("Entity").width(Length::FillPortion(1)),
            text("Details").width(Length::FillPortion(4)),
        ]
        .spacing(10)
        .padding(10),
    )
    .style(move |_iced_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(crate::theme::surface_elevated_color(theme))),
        border: iced::Border {
            color: crate::theme::border_color(theme),
            width: 1.0,
            radius: 3.0.into(),
        },
        ..Default::default()
    });

    let mut entries_list = column![].spacing(5);

    if entries.is_empty() {
        entries_list = entries_list.push(
            container(
                text("No audit entries yet")
                    .size(16)
                    .style(move |_iced_theme: &iced::Theme| iced::widget::text::Style {
                        color: Some(crate::theme::border_color(theme)),
                    }),
            )
            .padding(20)
            .width(Length::Fill)
            .center_x(Length::Fill),
        );
    } else {
        // Pre-format all strings to avoid temporary value issues
        let formatted_entries: Vec<_> = entries
            .iter()
            .map(|entry| {
                (
                    entry.formatted_timestamp(),
                    entry.username.clone(),
                    format!("{}", entry.action),
                    format!(
                        "{} {}",
                        entry.entity_type,
                        entry.entity_id.as_deref().unwrap_or("")
                    ),
                    entry.details.clone(),
                    match entry.action {
                        AuditAction::ItemDeleted
                        | AuditAction::NoteDeleted
                        | AuditAction::UserDeleted
                        | AuditAction::DataCleared => crate::theme::danger_color(theme),
                        AuditAction::ItemCreated
                        | AuditAction::NoteCreated
                        | AuditAction::UserCreated => iced::Color::from_rgb(0.3, 0.8, 0.3),
                        AuditAction::ItemUpdated
                        | AuditAction::NoteUpdated
                        | AuditAction::UserUpdated
                        | AuditAction::SettingsChanged => iced::Color::from_rgb(0.5, 0.7, 0.9),
                        _ => iced::Color::from_rgb(0.7, 0.7, 0.7),
                    },
                )
            })
            .collect();

        for (timestamp, username, action, entity, details, action_color) in formatted_entries {
            let entry_row = container(
                row![
                    text(timestamp).width(Length::FillPortion(2)),
                    text(username).width(Length::FillPortion(2)),
                    text(action).width(Length::FillPortion(2)).style(
                        move |_theme: &iced::Theme| {
                            iced::widget::text::Style {
                                color: Some(action_color),
                            }
                        }
                    ),
                    text(entity).width(Length::FillPortion(1)).size(12),
                    text(details).width(Length::FillPortion(4)).size(13),
                ]
                .spacing(10)
                .padding(10),
            )
            .style(move |_iced_theme: &iced::Theme| container::Style {
                background: Some(iced::Background::Color(crate::theme::surface_color(theme))),
                border: iced::Border {
                    color: crate::theme::border_color(theme),
                    width: 1.0,
                    radius: 3.0.into(),
                },
                ..Default::default()
            });

            entries_list = entries_list.push(entry_row);
        }
    }

    let content = column![
        header_row,
        text("").size(10),
        table_header,
        text("").size(5),
        scrollable(entries_list).height(500),
    ]
    .spacing(5)
    .padding(20);

    content.into()
}
