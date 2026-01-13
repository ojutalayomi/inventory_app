use chrono::{DateTime, Utc};
use iced::widget::{
    Column, button, column, container, row, scrollable, text, text_editor, text_input,
};
use iced::{Element, Length};

use crate::messages::Message;
use crate::note::Note;

pub fn view<'a>(
    notes: &'a [Note],
    selected_note_id: Option<&'a String>,
    editor_content: &'a text_editor::Content,
    note_title: &'a str,
    delete_confirm_id: Option<&'a String>,
    theme: &'a crate::messages::AppTheme,
) -> Element<'a, Message> {
    // Left sidebar with notes list
    let mut notes_list = Column::new().spacing(5).padding(10);

    notes_list = notes_list.push(
        button("+ New Note")
            .on_press(Message::CreateNote)
            .width(Length::Fill)
            .padding(10)
            .style(
                |_theme: &iced::Theme, _status: iced::widget::button::Status| {
                    iced::widget::button::Style {
                        background: Some(iced::Background::Color(iced::Color::from_rgb(
                            0.2, 0.5, 0.3,
                        ))),
                        border: iced::Border {
                            color: iced::Color::from_rgb(0.3, 0.6, 0.4),
                            width: 1.0,
                            radius: 5.0.into(),
                        },
                        text_color: iced::Color::WHITE,
                        ..Default::default()
                    }
                },
            ),
    );

    notes_list = notes_list.push(text("").size(10));

    if notes.is_empty() {
        notes_list = notes_list.push(
            text("No notes yet.\nClick 'New Note' to start.")
                .size(13)
                .style(|_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(iced::Color::from_rgb(0.5, 0.5, 0.5)),
                }),
        );
    } else {
        for note in notes.iter() {
            let is_selected = selected_note_id.map(|id| id == &note.id).unwrap_or(false);

            let date = DateTime::from_timestamp(note.updated_at, 0)
                .unwrap_or_else(|| Utc::now())
                .format("%b %d, %Y")
                .to_string();

            let note_item = container(
                column![
                    row![
                        text(&note.title).size(14).width(Length::Fill),
                        button("×")
                            .on_press(Message::DeleteNote(note.id.clone()))
                            .padding(2)
                            .style(
                                |_theme: &iced::Theme, _status: iced::widget::button::Status| {
                                    iced::widget::button::Style {
                                        background: Some(iced::Background::Color(
                                            iced::Color::from_rgb(0.6, 0.2, 0.2),
                                        )),
                                        text_color: iced::Color::WHITE,
                                        ..Default::default()
                                    }
                                }
                            ),
                    ]
                    .spacing(5)
                    .align_y(iced::Alignment::Center),
                    text(date).size(11).style(|_theme: &iced::Theme| {
                        iced::widget::text::Style {
                            color: Some(iced::Color::from_rgb(0.6, 0.6, 0.6)),
                        }
                    }),
                ]
                .spacing(3)
                .padding(8),
            )
            .width(Length::Fill)
            .style(move |_iced_theme: &iced::Theme| container::Style {
                background: if is_selected {
                    Some(iced::Background::Color(crate::theme::surface_elevated_color(theme)))
                } else {
                    Some(iced::Background::Color(crate::theme::surface_color(theme)))
                },
                border: iced::Border {
                    color: if is_selected {
                        crate::theme::primary_color(theme)
                    } else {
                        crate::theme::border_color(theme)
                    },
                    width: 1.0,
                    radius: 5.0.into(),
                },
                ..Default::default()
            });

            let clickable = button(note_item)
                .on_press(Message::SelectNote(note.id.clone()))
                .width(Length::Fill)
                .padding(0)
                .style(
                    |_theme: &iced::Theme, _status: iced::widget::button::Status| {
                        iced::widget::button::Style {
                            background: Some(iced::Background::Color(iced::Color::TRANSPARENT)),
                            ..Default::default()
                        }
                    },
                );

            notes_list = notes_list.push(clickable);
        }
    }

    let sidebar = container(scrollable(notes_list))
        .width(280)
        .height(Length::Fill)
        .style(move |_iced_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(crate::theme::surface_color(theme))),
            border: iced::Border {
                color: crate::theme::border_color(theme),
                width: 0.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        });

    let main_area = container(
        column![
            row![
                text("Note Title:").size(14),
                text_input("Untitled", note_title)
                    .on_input(Message::UpdateNoteTitle)
                    .padding(8),
                button("New Note").on_press(Message::CreateNote).padding(8),
                if let Some(note_id) = selected_note_id {
                    button("Delete Note")
                        .on_press(Message::DeleteNote(note_id.clone()))
                        .padding(8)
                } else {
                    button("Delete Note").padding(8)
                },
            ]
            .spacing(10)
            .align_y(iced::Alignment::Center),
            container(
                text_editor(editor_content)
                    .on_action(Message::UpdateNoteContent)
                    .padding(10)
                    .height(iced::Length::Fill)
            )
            .height(iced::Length::Fill),
            row![
                text(format!("Lines: {}", editor_content.line_count())).size(12),
                text("|").size(12),
                text(format!("Characters: {}", editor_content.text().len())).size(12),
            ]
            .spacing(5),
        ]
        .spacing(10)
        .padding(10),
    )
    .width(Length::Fill)
    .height(Length::Fill);

    // Create main layout
    let content = row![sidebar, main_area]
        .spacing(0)
        .width(Length::Fill)
        .height(Length::Fill);

    // If delete confirm dialog needed, overlay it
    if let Some(_note_id) = delete_confirm_id {
        iced::widget::stack![
            content,
            container(
                container(
                    column![
                        text("Delete Note?").size(24),
                        text("").size(10),
                        text("This action cannot be undone.").size(14),
                        text("").size(20),
                        row![
                            button("Delete")
                                .on_press(Message::ConfirmDeleteNote)
                                .padding(10)
                                .style(
                                    move |_iced_theme: &iced::Theme, _status: iced::widget::button::Status| {
                                        iced::widget::button::Style {
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
                                    }
                                ),
                            button("Cancel")
                                .on_press(Message::CloseDeleteConfirm)
                                .padding(10)
                                .style(
                                    move |_iced_theme: &iced::Theme, _status: iced::widget::button::Status| {
                                        iced::widget::button::Style {
                                            background: Some(iced::Background::Color(
                                                crate::theme::surface_elevated_color(theme),
                                            )),
                                            text_color: crate::theme::text_color(theme),
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
                    ]
                    .spacing(5)
                    .padding(30),
                )
                .width(350)
                .style(move |_iced_theme: &iced::Theme| container::Style {
                    background: Some(iced::Background::Color(crate::theme::surface_color(theme))),
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
        ]
        .into()
    } else {
        content.into()
    }
}
