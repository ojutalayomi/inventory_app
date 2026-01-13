use iced::widget::{button, column, container, pick_list, row, scrollable, text, text_input};
use iced::{Element, Length};

use crate::messages::Message;
use crate::user::{User, UserRole};

pub fn view<'a>(
    users: &[&'a User],
    current_user_role: UserRole,
    new_username: &'a str,
    new_password: &'a str,
    new_role: Option<UserRole>,
    error_message: Option<&'a str>,
    theme: &'a crate::messages::AppTheme,
) -> Element<'a, Message> {
    if !current_user_role.can_manage_users() {
        return container(
            text("Access Denied: Admin privileges required")
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

    let title = text("User Management").size(28);

    // Add new user form
    let add_user_section = container(
        column![
            text("Add New User").size(20),
            text("").size(5),
            row![
                column![
                    text("Username:").size(14),
                    text_input("Username", new_username)
                        .on_input(Message::NewUsernameChanged)
                        .padding(8)
                        .width(200),
                ]
                .spacing(5),
                column![
                    text("Password:").size(14),
                    text_input("Password", new_password)
                        .on_input(Message::NewPasswordChanged)
                        .secure(true)
                        .padding(8)
                        .width(200),
                ]
                .spacing(5),
                column![
                    text("Role:").size(14),
                    pick_list(
                        vec![
                            UserRole::Admin,
                            UserRole::Manager,
                            UserRole::User,
                            UserRole::Viewer
                        ],
                        new_role,
                        Message::NewRoleChanged,
                    )
                    .padding(8)
                    .width(150),
                ]
                .spacing(5),
                column![
                    text("").size(14),
                    button("Add User").on_press(Message::CreateUser).padding(8),
                ]
                .spacing(5),
            ]
            .spacing(10),
        ]
        .spacing(10)
        .padding(15),
    )
    .style(move |_iced_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(crate::theme::surface_color(theme))),
        border: iced::Border {
            color: crate::theme::border_color(theme),
            width: 1.0,
            radius: 5.0.into(),
        },
        ..Default::default()
    });

    // Users list
    let users_header = row![
        text("Username").width(Length::FillPortion(2)),
        text("Role").width(Length::FillPortion(1)),
        text("Status").width(Length::FillPortion(1)),
        text("Last Login").width(Length::FillPortion(2)),
        text("Actions").width(Length::FillPortion(2)),
    ]
    .spacing(10)
    .padding(10);

    let mut users_list = column![users_header].spacing(5);

    for user in users {
        let status_text = if user.active { "Active" } else { "Inactive" };
        let status_color = if user.active {
            iced::Color::from_rgb(0.3, 0.8, 0.3)
        } else {
            iced::Color::from_rgb(0.8, 0.3, 0.3)
        };

        let last_login = user
            .last_login
            .map(|ts| {
                chrono::DateTime::from_timestamp(ts, 0)
                    .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
                    .unwrap_or_else(|| "Unknown".to_string())
            })
            .unwrap_or_else(|| "Never".to_string());

        let role_str = format!("{:?}", user.role);

        let user_row = container(
            row![
                text(&user.username).width(Length::FillPortion(2)),
                text(role_str).width(Length::FillPortion(1)),
                text(status_text).width(Length::FillPortion(1)).style(
                    move |_theme: &iced::Theme| {
                        iced::widget::text::Style {
                            color: Some(status_color),
                        }
                    }
                ),
                text(last_login).width(Length::FillPortion(2)),
                row![
                    button("Edit")
                        .on_press(Message::EditUser(user.id.clone()))
                        .padding(5),
                    button("Delete")
                        .on_press(Message::DeleteUser(user.id.clone()))
                        .padding(5)
                        .style(
                            |_theme: &iced::Theme, _status: iced::widget::button::Status| {
                                iced::widget::button::Style {
                                    background: Some(iced::Background::Color(
                                        crate::theme::danger_color(theme),
                                    )),
                                    text_color: iced::Color::WHITE,
                                    border: iced::Border {
                                        radius: 3.0.into(),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                }
                            }
                        ),
                ]
                .spacing(5)
                .width(Length::FillPortion(2)),
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

        users_list = users_list.push(user_row);
    }

    let users_section = container(
        column![
            text("Existing Users").size(20),
            scrollable(users_list).height(400),
        ]
        .spacing(10),
    )
    .padding(15);

    let mut content = column![
        title,
        text("").size(10),
        add_user_section,
        text("").size(15),
        users_section,
    ]
    .spacing(5)
    .padding(20);

    if let Some(error) = error_message {
        content = content.push(
            container(text(error).size(14).style(move |_iced_theme: &iced::Theme| {
                iced::widget::text::Style {
                    color: Some(crate::theme::danger_color(theme)),
                }
            }))
            .padding(10),
        );
    }

    scrollable(container(content).width(Length::Fill)).into()
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Admin => write!(f, "Admin"),
            UserRole::Manager => write!(f, "Manager"),
            UserRole::User => write!(f, "User"),
            UserRole::Viewer => write!(f, "Viewer"),
        }
    }
}
