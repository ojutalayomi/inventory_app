use iced::widget::{button, column, container, row, text};
use iced::{Color, Element, Length};
use crate::{InventoryApp, Message};
use crate::messages::{LayoutStyle, View};
use crate::theme;
use crate::icons;

impl InventoryApp {
    pub fn view_loaded(&self) -> Element<Message> {
        let session = self
            .session
            .as_ref()
            .expect("Must be logged in to view app");

        let theme = &self.settings.theme;
        let alert_count = self.alert_manager.get_unacknowledged_count();
        let layout_style = &self.settings.layout_style;

        // Helper function to create modern tab buttons (for header layout)
        let make_tab = |label: String, view: View, icon: icons::Icon| {
            let is_active = self.current_view == view;
            
            let tab_content = row![
                icon.view(icons::IconSize::Medium, theme),
                text(label).size(theme::TEXT_BODY),
            ]
            .spacing(theme::SPACING_SM)
            .align_y(iced::Alignment::Center);
            
            button(tab_content)
                .on_press(Message::SwitchView(view))
                .padding([theme::SPACING_MD, theme::SPACING_XL])
                .style(move |_theme: &iced::Theme, status: button::Status| {
                    let base_color = if is_active {
                        theme::primary_color(theme)
                    } else {
                        theme::surface_color(theme)
                    };
                    
                    let bg_color = match status {
                        button::Status::Hovered if !is_active => {
                            theme::surface_elevated_color(theme)
                        }
                        _ => base_color,
                    };
                    
                    let text_color = if is_active {
                        iced::Color::WHITE
                    } else {
                        theme::text_color(theme)
                    };
                    
                    button::Style {
                        background: Some(iced::Background::Color(bg_color)),
                        text_color,
                        border: iced::Border {
                            radius: theme::RADIUS_MD.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                })
        };

        // Helper function to create sidebar items
        let make_sidebar_item = |label: String, view: View, icon: icons::Icon| {
            let is_active = self.current_view == view;
            let collapsed = self.sidebar_collapsed;
            
            let item_content: Element<Message> = if collapsed {
                // Collapsed: show icon only, centered
                container(
                    icon.view(icons::IconSize::Medium, theme)
                )
                .width(Length::Fill)
                .center_x(Length::Fill)
                .into()
            } else {
                // Expanded: show icon + label horizontally (like header tabs)
                row![
                    icon.view(icons::IconSize::Medium, theme),
                    text(label)
                        .size(theme::TEXT_BODY)
                        .style(move |_theme: &iced::Theme| text::Style {
                            color: Some(if is_active {
                                iced::Color::WHITE
                            } else {
                                theme::text_color(theme)
                            }),
                        }),
                ]
                .spacing(theme::SPACING_SM)
                .align_y(iced::Alignment::Center)
                .width(Length::Fill)
                .into()
            };
            
            button(item_content)
                .on_press(Message::SwitchView(view))
                .padding(if collapsed {
                    [theme::SPACING_MD, theme::SPACING_MD]
                } else {
                    [theme::SPACING_MD, theme::SPACING_LG]
                })
                .width(Length::Fill)
                .style(move |_theme: &iced::Theme, status: button::Status| {
                    let base_color = if is_active {
                        theme::primary_color(theme)
                    } else {
                        theme::surface_color(theme)
                    };
                    
                    let bg_color = match status {
                        button::Status::Hovered if !is_active => {
                            theme::surface_elevated_color(theme)
                        }
                        _ => base_color,
                    };
                    
                    button::Style {
                        background: Some(iced::Background::Color(bg_color)),
                        text_color: if is_active {
                            iced::Color::WHITE
                        } else {
                            theme::text_color(theme)
                        },
                        border: iced::Border {
                            radius: theme::RADIUS_MD.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                })
        };
        
        // Create navigation items (used for both header and sidebar)
        let alerts_label = if alert_count > 0 {
            format!("Alerts ({})", alert_count)
        } else {
            "Alerts".to_string()
        };
        
        // Build navigation items list
        let mut nav_items: Vec<(String, View, icons::Icon)> = vec![
            ("Inventory".to_string(), View::Inventory, icons::Icon::Inventory),
            ("Notes".to_string(), View::Editor, icons::Icon::Notes),
            (alerts_label, View::Alerts, icons::Icon::Alerts),
            ("Settings".to_string(), View::Settings, icons::Icon::Settings),
        ];
        
        // Only admins can access user management
        if session.role.can_manage_users() {
            nav_items.push(("Users".to_string(), View::UserManagement, icons::Icon::Users));
        }
        
        // Only managers and admins can access audit log
        if session.role.can_view_audit() {
            nav_items.push(("Audit Log".to_string(), View::AuditLog, icons::Icon::AuditLog));
        }
        
        // Create header tabs
        let mut nav_tabs: Vec<Element<Message>> = nav_items
            .iter()
            .map(|(label, view, icon)| make_tab(label.clone(), view.clone(), *icon).into())
            .collect();
        
        // About button for header (styled differently as it's not a view)
        let about_btn_header = button(
            row![
                icons::Icon::About.view(icons::IconSize::Medium, theme),
                text("About").size(theme::TEXT_BODY),
            ]
            .spacing(theme::SPACING_SM)
            .align_y(iced::Alignment::Center)
        )
            .on_press(Message::ShowAbout)
            .padding([theme::SPACING_MD, theme::SPACING_XL])
            .style(move |_theme: &iced::Theme, status: button::Status| {
                let bg_color = match status {
                    button::Status::Hovered => theme::surface_elevated_color(theme),
                    _ => theme::surface_color(theme),
                };
                
                button::Style {
                    background: Some(iced::Background::Color(bg_color)),
                    text_color: theme::text_color(theme),
                    border: iced::Border {
                        radius: theme::RADIUS_MD.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            });
        
        nav_tabs.push(about_btn_header.into());
        
        // Create sidebar items
        let mut sidebar_items: Vec<Element<Message>> = nav_items
            .iter()
            .map(|(label, view, icon)| make_sidebar_item(label.clone(), view.clone(), *icon).into())
            .collect();
        
        // About button for sidebar (special handling - not a view)
        let about_btn_sidebar = {
            let collapsed = self.sidebar_collapsed;
            let item_content: Element<Message> = if collapsed {
                // Collapsed: show icon only, centered
                container(
                    icons::Icon::About.view(icons::IconSize::Medium, theme)
                )
                .width(Length::Fill)
                .center_x(Length::Fill)
                .into()
            } else {
                // Expanded: show icon + label horizontally
                row![
                    icons::Icon::About.view(icons::IconSize::Medium, theme),
                    text("About")
                        .size(theme::TEXT_BODY)
                        .style(move |_theme: &iced::Theme| text::Style {
                            color: Some(theme::text_color(theme)),
                        }),
                ]
                .spacing(theme::SPACING_SM)
                .align_y(iced::Alignment::Center)
                .width(Length::Fill)
                .into()
            };
            
            button(item_content)
                .on_press(Message::ShowAbout)
                .padding(if collapsed {
                    [theme::SPACING_MD, theme::SPACING_MD]
                } else {
                    [theme::SPACING_MD, theme::SPACING_LG]
                })
                .width(Length::Fill)
                .style(move |_theme: &iced::Theme, status: button::Status| {
                    let bg_color = match status {
                        button::Status::Hovered => theme::surface_elevated_color(theme),
                        _ => theme::surface_color(theme),
                    };
                    
                    button::Style {
                        background: Some(iced::Background::Color(bg_color)),
                        text_color: theme::text_color(theme),
                        border: iced::Border {
                            radius: theme::RADIUS_MD.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                })
        };
        sidebar_items.push(about_btn_sidebar.into());
        
        // Helper function to create user pill
        let make_user_pill = || {
            let user_name = session.username.clone();
            let role_text = format!("{:?}", session.role);
            container(
                row![
                    icons::Icon::User.view(icons::IconSize::Medium, theme),
                    column![
                        text(user_name)
                            .size(theme::TEXT_BODY)
                            .style(move |_theme: &iced::Theme| text::Style {
                                color: Some(theme::text_color(theme)),
                            }),
                        text(role_text)
                            .size(theme::TEXT_CAPTION)
                            .style(move |_theme: &iced::Theme| text::Style {
                                color: Some(theme::text_secondary_color(theme)),
                            }),
                    ]
                    .spacing(2),
                ]
                .spacing(theme::SPACING_SM)
                .align_y(iced::Alignment::Center)
            )
            .padding([theme::SPACING_SM, theme::SPACING_LG])
            .style(move |_theme: &iced::Theme| container::Style {
                background: Some(iced::Background::Color(theme::surface_color(theme))),
                border: iced::Border {
                    color: theme::border_color(theme),
                    width: 1.0,
                    radius: theme::RADIUS_FULL.into(),
                },
                ..Default::default()
            })
        };
        
        // Helper function to create logout button
        let make_logout_btn = || {
            button(
                row![
                    icons::Icon::Logout.view(icons::IconSize::Small, theme),
                    text("Logout").size(theme::TEXT_BODY),
                ]
                .spacing(theme::SPACING_SM)
                .align_y(iced::Alignment::Center)
            )
                .on_press(Message::Logout)
                .padding([theme::SPACING_MD, theme::SPACING_LG])
                .style(move |_theme: &iced::Theme, status: button::Status| {
                    let bg_color = match status {
                        button::Status::Hovered => theme::danger_color(theme),
                        _ => Color::from_rgba(
                            theme::danger_color(theme).r,
                            theme::danger_color(theme).g,
                            theme::danger_color(theme).b,
                            0.8,
                        ),
                    };
                    
                    button::Style {
                        background: Some(iced::Background::Color(bg_color)),
                        text_color: iced::Color::WHITE,
                        border: iced::Border {
                            radius: theme::RADIUS_MD.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                })
        };
        
        // Create user pill and logout button for header
        let user_pill = make_user_pill();
        let logout_btn = make_logout_btn();
        
        // Top bar (for sidebar layout) - always visible with user info
        let top_bar = {
            let user_pill_top = make_user_pill();
            let logout_btn_top = make_logout_btn();
            container(
                row![
                    iced::widget::horizontal_space(),
                    row![user_pill_top, logout_btn_top]
                        .spacing(theme::SPACING_LG)
                        .align_y(iced::Alignment::Center)
                        .width(Length::Shrink),
                ]
                .align_y(iced::Alignment::Center)
                .spacing(theme::SPACING_MD)
                .width(Length::Fill)
            )
        }
        .padding(theme::SPACING_LG)
        .width(Length::Fill)
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(theme::surface_color(theme))),
            border: iced::Border {
                color: theme::border_color(theme),
                width: 1.0,
                radius: 0.0.into(),
            },
            shadow: iced::Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                offset: iced::Vector::new(0.0, 2.0),
                blur_radius: 8.0,
            },
            ..Default::default()
        });

        // Sidebar with collapse toggle
        let sidebar = {
            let collapsed = self.sidebar_collapsed;
            
            // Toggle button
            let toggle_btn = button(
                text(if collapsed { "▶" } else { "◀" })
                    .size(theme::TEXT_BODY)
                    .style(move |_theme: &iced::Theme| text::Style {
                        color: Some(theme::text_color(theme)),
                    })
            )
            .on_press(Message::ToggleSidebar)
            .padding(theme::SPACING_SM)
            .style(move |_theme: &iced::Theme, status: button::Status| {
                let bg_color = match status {
                    button::Status::Hovered => theme::surface_elevated_color(theme),
                    _ => theme::surface_color(theme),
                };
                
                button::Style {
                    background: Some(iced::Background::Color(bg_color)),
                    text_color: theme::text_color(theme),
                    border: iced::Border {
                        radius: theme::RADIUS_SM.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            });
            
            // User info and logout for sidebar (at bottom)
            // Create sidebar-specific user pill
            let user_pill_sidebar = {
                let user_name = session.username.clone();
                let role_text = format!("{:?}", session.role);
                if collapsed {
                    // Collapsed: icon only
                    container(
                        icons::Icon::User.view(icons::IconSize::Medium, theme)
                    )
                    .width(Length::Fill)
                    .padding(theme::SPACING_MD)
                    .center_x(Length::Fill)
                    .style(move |_theme: &iced::Theme| container::Style {
                        background: Some(iced::Background::Color(theme::surface_color(theme))),
                        border: iced::Border {
                            color: theme::border_color(theme),
                            width: 1.0,
                            radius: theme::RADIUS_MD.into(),
                        },
                        ..Default::default()
                    })
                } else {
                    // Expanded: full user info
                    container(
                        row![
                            icons::Icon::User.view(icons::IconSize::Medium, theme),
                            column![
                                text(user_name)
                                    .size(theme::TEXT_CAPTION)
                                    .style(move |_theme: &iced::Theme| text::Style {
                                        color: Some(theme::text_color(theme)),
                                    }),
                                text(role_text)
                                    .size(theme::TEXT_CAPTION - 2.0)
                                    .style(move |_theme: &iced::Theme| text::Style {
                                        color: Some(theme::text_secondary_color(theme)),
                                    }),
                            ]
                            .spacing(2),
                        ]
                        .spacing(theme::SPACING_SM)
                        .align_y(iced::Alignment::Center)
                    )
                    .width(Length::Fill)
                    .padding([theme::SPACING_SM, theme::SPACING_MD])
                    .style(move |_theme: &iced::Theme| container::Style {
                        background: Some(iced::Background::Color(theme::surface_color(theme))),
                        border: iced::Border {
                            color: theme::border_color(theme),
                            width: 1.0,
                            radius: theme::RADIUS_MD.into(),
                        },
                        ..Default::default()
                    })
                }
            };
            
            // Create sidebar-specific logout button
            let logout_btn_sidebar = if collapsed {
                // Collapsed: icon only
                button(
                    icons::Icon::Logout.view(icons::IconSize::Medium, theme)
                )
                .on_press(Message::Logout)
                .padding(theme::SPACING_MD)
                .width(Length::Fill)
                .style(move |_theme: &iced::Theme, status: button::Status| {
                    let bg_color = match status {
                        button::Status::Hovered => theme::danger_color(theme),
                        _ => Color::from_rgba(
                            theme::danger_color(theme).r,
                            theme::danger_color(theme).g,
                            theme::danger_color(theme).b,
                            0.8,
                        ),
                    };
                    
                    button::Style {
                        background: Some(iced::Background::Color(bg_color)),
                        text_color: iced::Color::WHITE,
                        border: iced::Border {
                            radius: theme::RADIUS_MD.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                })
            } else {
                // Expanded: icon + text
                button(
                    row![
                        icons::Icon::Logout.view(icons::IconSize::Small, theme),
                        text("Logout").size(theme::TEXT_CAPTION),
                    ]
                    .spacing(theme::SPACING_SM)
                    .align_y(iced::Alignment::Center)
                )
                .on_press(Message::Logout)
                .padding([theme::SPACING_SM, theme::SPACING_MD])
                .width(Length::Fill)
                .style(move |_theme: &iced::Theme, status: button::Status| {
                    let bg_color = match status {
                        button::Status::Hovered => theme::danger_color(theme),
                        _ => Color::from_rgba(
                            theme::danger_color(theme).r,
                            theme::danger_color(theme).g,
                            theme::danger_color(theme).b,
                            0.8,
                        ),
                    };
                    
                    button::Style {
                        background: Some(iced::Background::Color(bg_color)),
                        text_color: iced::Color::WHITE,
                        border: iced::Border {
                            radius: theme::RADIUS_MD.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                })
            };
            
            // Create sidebar user section
            let sidebar_user_section = column![
                user_pill_sidebar,
                logout_btn_sidebar,
            ]
            .spacing(theme::SPACING_SM)
            .width(Length::Fill);
            
            container(
                column![
                    // Top section: toggle and nav items
                    container(toggle_btn)
                        .width(Length::Fill)
                        .padding([theme::SPACING_MD, theme::SPACING_SM]),
                    container(
                        column(sidebar_items)
                            .spacing(theme::SPACING_XS)
                            .width(Length::Fill)
                    )
                    .padding([theme::SPACING_SM, theme::SPACING_SM])
                    .width(Length::Fill)
                    .height(Length::Shrink),
                    // Spacer to push user section to bottom
                    container(iced::widget::Space::new(Length::Shrink, Length::Fill)),
                    // Bottom section: user info and logout
                    container(sidebar_user_section)
                        .padding([theme::SPACING_MD, theme::SPACING_SM])
                        .width(Length::Fill),
                ]
                .spacing(0)
                .width(Length::Fill)
                .height(Length::Fill)
            )
            .width(if collapsed {
                Length::Fixed(60.0)
            } else {
                Length::Fixed(200.0)
            })
            .height(Length::Fill)
            .style(move |_theme: &iced::Theme| container::Style {
                background: Some(iced::Background::Color(theme::surface_color(theme))),
                border: iced::Border {
                    color: theme::border_color(theme),
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            })
        };

        // Modern header with tabs and user info (for header layout)
        let header = container(
            row![
                // Tabs - will wrap naturally if needed
                row(nav_tabs)
                    .spacing(theme::SPACING_XS)
                    .width(Length::Shrink),
                // Spacer to push user info to the right
                iced::widget::horizontal_space(),
                // User info - fixed width, always on the right
                row![user_pill, logout_btn]
                    .spacing(theme::SPACING_LG)
                    .align_y(iced::Alignment::Center)
                    .width(Length::Shrink),
            ]
            .align_y(iced::Alignment::Center)
            .spacing(theme::SPACING_MD)
            .width(Length::Fill)
        )
        .padding(theme::SPACING_LG)
        .width(Length::Fill)
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(theme::surface_color(theme))),
            border: iced::Border {
                color: theme::border_color(theme),
                width: 1.0,
                radius: 0.0.into(),
            },
            shadow: iced::Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                offset: iced::Vector::new(0.0, 2.0),
                blur_radius: 8.0,
            },
            ..Default::default()
        });

        let ctrl_or_cmd = if cfg!(target_os = "macos") {
            "Cmd"
        } else {
            "Ctrl"
        };
        let shortcuts_text = format!(
            "{} {} {}+1: Inventory • {}+2: Notes • {}+K: Calculator • {}+N: New Note • {}+S: Save",
            "", "", ctrl_or_cmd, ctrl_or_cmd, ctrl_or_cmd, ctrl_or_cmd, ctrl_or_cmd
        );
        let shortcuts_hint = container(
            row![
                icons::Icon::Lightbulb.view(icons::IconSize::Small, theme),
                text(shortcuts_text)
                    .size(theme::TEXT_CAPTION)
                    .style(move |_theme: &iced::Theme| text::Style {
                        color: Some(theme::text_tertiary_color(theme)),
                    }),
            ]
            .spacing(theme::SPACING_XS)
            .align_y(iced::Alignment::Center)
        )
        .padding([theme::SPACING_SM, theme::SPACING_LG])
        .width(Length::Fill)
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(theme::surface_elevated_color(theme))),
            ..Default::default()
        });

        let content = match self.current_view {
            View::Inventory => crate::views::inventory::view(
                &self.filtered_items,
                &self.items,
                &self.search_filter,
                self.show_search_panel,
                theme,
            ),
            View::Editor => crate::views::editor::view(
                &self.notes,
                self.selected_note_id.as_ref(),
                &self.editor_content,
                &self.note_title_input,
                self.delete_note_confirm.as_ref(),
                theme,
            ),
            View::Settings => crate::views::settings::view(
                &self.settings,
                &self.settings_interval_input,
                &self.settings_category_input,
                self.latest_version.as_ref(),
                self.import_error.as_deref(),
                theme,
            ),
            View::UserManagement => {
                let users: Vec<_> = self.auth_store.get_all_users();
                crate::views::user_management::view(
                    &users,
                    session.role,
                    &self.new_username_input,
                    &self.new_password_input,
                    self.new_role_input,
                    self.user_operation_error.as_deref(),
                    theme,
                )
            }
            View::AuditLog => {
                let entries = self.audit_log.get_recent(100);
                crate::views::audit_log::view(&entries, session.role, theme)
            }
            View::Alerts => crate::views::alerts::view(&self.alert_manager, session.role, theme),
        };

        let content_container = container(content)
            .padding(theme::SPACING_XL)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_theme: &iced::Theme| container::Style {
                background: Some(iced::Background::Color(theme::bg_color(theme))),
                ..Default::default()
            });

        // Conditionally render based on layout style
        let main_view: Element<Message> = match layout_style {
            LayoutStyle::Header => {
                // Header layout: header at top, content below
                container(
                    column![header, shortcuts_hint, content_container]
                        .spacing(0)
                        .width(Length::Fill)
                        .height(Length::Fill)
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
            }
            LayoutStyle::Sidebar => {
                // Sidebar layout: sidebar on left, content on right (no top bar)
                container(
                    row![
                        sidebar,
                        column![
                            shortcuts_hint,
                            content_container
                        ]
                        .spacing(0)
                        .width(Length::Fill)
                        .height(Length::Fill)
                    ]
                    .spacing(0)
                    .width(Length::Fill)
                    .height(Length::Fill)
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
            }
        };

        if self.calculator.visible {
            let calculator = crate::views::calculator::view(&self.calculator.display, theme);

            let positioned_calc = container(calculator)
                .width(350)
                .padding(theme::SPACING_XL)
                .style(move |_theme: &iced::Theme| container::Style {
                    background: Some(iced::Background::Color(
                        Color::from_rgba(
                            theme::surface_color(theme).r,
                            theme::surface_color(theme).g,
                            theme::surface_color(theme).b,
                            0.95,
                        )
                    )),
                    border: iced::Border {
                        color: theme::primary_color(theme),
                        width: 2.0,
                        radius: theme::RADIUS_LG.into(),
                    },
                    shadow: iced::Shadow {
                        color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                        offset: iced::Vector::new(0.0, 8.0),
                        blur_radius: 24.0,
                    },
                    ..Default::default()
                });

            // Stack calculator on top of main view
            container(iced::widget::stack![
                main_view,
                container(positioned_calc)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill),
            ])
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        } else {
            main_view
        }
    }
}

