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
            .style(move |_theme: &iced::Theme| container::Style {
                background: if is_selected {
                    Some(iced::Background::Color(iced::Color::from_rgb(
                        0.25, 0.35, 0.45,
                    )))
                } else {
                    Some(iced::Background::Color(iced::Color::from_rgb(
                        0.15, 0.15, 0.15,
                    )))
                },
                border: iced::Border {
                    color: if is_selected {
                        iced::Color::from_rgb(0.35, 0.45, 0.55)
                    } else {
                        iced::Color::from_rgb(0.25, 0.25, 0.25)
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
        .style(|_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgb(
                0.1, 0.1, 0.1,
            ))),
            border: iced::Border {
                color: iced::Color::from_rgb(0.2, 0.2, 0.2),
                width: 1.0,
                ..Default::default()
            },
            ..Default::default()
        });

    // Right editor area
    let editor_area = if let Some(_selected_id) = selected_note_id {
        column![
            text("Note Title:").size(14),
            text_input("Enter note title", note_title)
                .on_input(Message::UpdateNoteTitle)
                .padding(8)
                .size(16),
            text("").size(5),
            text_editor(editor_content)
                .on_action(Message::UpdateNoteContent)
                .height(Length::Fill)
                .padding(10),
            row![
                text(format!("Lines: {}", editor_content.line_count())).size(12),
                text(format!("Characters: {}", editor_content.text().len())).size(12),
            ]
            .spacing(15)
            .padding(5),
        ]
        .spacing(10)
        .padding(15)
    } else {
        column![
            container(
                text("Select a note or create a new one to start editing")
                    .size(18)
                    .style(|_theme: &iced::Theme| {
                        iced::widget::text::Style {
                            color: Some(iced::Color::from_rgb(0.5, 0.5, 0.5)),
                        }
                    })
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
        ]
    };

    let main_content = container(editor_area)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(10);

    let layout = row![sidebar, main_content].spacing(0).height(Length::Fill);

    // Show delete confirmation dialog if needed
    if let Some(_delete_id) = delete_confirm_id {
        let dialog = container(
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
                                |_theme: &iced::Theme, _status: iced::widget::button::Status| {
                                    iced::widget::button::Style {
                                        background: Some(iced::Background::Color(
                                            iced::Color::from_rgb(0.6, 0.2, 0.2),
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
                                |_theme: &iced::Theme, _status: iced::widget::button::Status| {
                                    iced::widget::button::Style {
                                        background: Some(iced::Background::Color(
                                            iced::Color::from_rgb(0.3, 0.3, 0.3),
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
                    ]
                    .spacing(10),
                ]
                .spacing(5)
                .padding(30),
            )
            .width(350)
            .style(|_theme: &iced::Theme| container::Style {
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
        .style(|_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgba(
                0.0, 0.0, 0.0, 0.7,
            ))),
            ..Default::default()
        });

        iced::widget::stack![layout, dialog].into()
    } else {
        layout.into()
    }
}
