use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Color, Element, Length};

use crate::messages::{AppTheme, ItemDialogMode, Message};
use crate::theme;
use crate::icons;

pub fn view<'a>(
    mode: &ItemDialogMode,
    name: &'a str,
    sku: &'a str,
    category: &'a str,
    supplier: &'a str,
    description: &'a str,
    quantity: &'a str,
    price: &'a str,
    validation_error: Option<&'a str>,
    similar_items: &'a [String],
    app_theme: &'a AppTheme,
) -> Element<'a, Message> {
    let title = match mode {
        ItemDialogMode::Add => row![
            icons::Icon::Add.view(icons::IconSize::Medium, app_theme),
            text("Add Inventory Item").size(theme::TEXT_H1),
        ]
        .spacing(theme::SPACING_SM)
        .align_y(iced::Alignment::Center),
        ItemDialogMode::Edit(_) => row![
            icons::Icon::Edit.view(icons::IconSize::Medium, app_theme),
            text("Edit Inventory Item").size(theme::TEXT_H1),
        ]
        .spacing(theme::SPACING_SM)
        .align_y(iced::Alignment::Center),
    };

    // Helper function to create labeled input fields
    let make_input = |label: &'a str, placeholder: &'a str, value: &'a str, on_input: fn(String) -> Message| {
        column![
            text(label)
                .size(theme::TEXT_BODY)
                .style(move |_theme: &iced::Theme| text::Style {
                    color: Some(theme::text_color(app_theme)),
                }),
            text_input(placeholder, value)
                .on_input(on_input)
                .padding(theme::SPACING_LG)
                .size(theme::TEXT_BODY)
                .width(Length::Fill)
                .style(move |theme_iced: &iced::Theme, status: text_input::Status| {
                    let default_style = text_input::default(theme_iced, status);
                    text_input::Style {
                        background: iced::Background::Color(theme::surface_elevated_color(app_theme)),
                        border: iced::Border {
                            color: match status {
                                text_input::Status::Focused => theme::primary_color(app_theme),
                                _ => theme::border_color(app_theme),
                            },
                            width: if matches!(status, text_input::Status::Focused) { 2.0 } else { 1.0 },
                            radius: theme::RADIUS_MD.into(),
                        },
                        icon: default_style.icon,
                        placeholder: theme::text_tertiary_color(app_theme),
                        value: theme::text_color(app_theme),
                        selection: theme::primary_color(app_theme),
                    }
                }),
        ]
        .spacing(theme::SPACING_SM)
    };

    let mut form_content = column![
        title,
    ]
    .spacing(theme::SPACING_LG);

    // Show validation error if present
    if let Some(error) = validation_error {
        form_content = form_content.push(
            container(
                row![
                    icons::Icon::AlertTriangle.view(icons::IconSize::Medium, app_theme),
                    text(error).size(theme::TEXT_BODY)
                ]
                .spacing(theme::SPACING_MD)
                .align_y(iced::Alignment::Center)
            )
            .padding(theme::SPACING_LG)
            .width(Length::Fill)
            .style(move |_theme: &iced::Theme| container::Style {
                background: Some(iced::Background::Color(
                    Color::from_rgba(
                        theme::danger_color(app_theme).r,
                        theme::danger_color(app_theme).g,
                        theme::danger_color(app_theme).b,
                        0.2,
                    )
                )),
                border: iced::Border {
                    color: theme::danger_color(app_theme),
                    width: 1.0,
                    radius: theme::RADIUS_MD.into(),
                },
                ..Default::default()
            })
        );
    }

    // Show similar items warning
    if !similar_items.is_empty() {
        let mut warning_col = column![
            row![
                icons::Icon::AlertTriangle.view(icons::IconSize::Medium, app_theme),
                text("Similar items found:")
                    .size(theme::TEXT_BODY)
                    .style(move |_theme: &iced::Theme| text::Style {
                        color: Some(theme::warning_color(app_theme)),
                    }),
            ]
            .spacing(theme::SPACING_SM)
            .align_y(iced::Alignment::Center),
        ]
        .spacing(theme::SPACING_SM);
        
        for item in similar_items.iter().take(3) {
            warning_col = warning_col.push(
                text(format!("  • {}", item))
                    .size(theme::TEXT_CAPTION)
                    .style(move |_theme: &iced::Theme| text::Style {
                        color: Some(theme::text_secondary_color(app_theme)),
                    })
            );
        }
        
        form_content = form_content.push(
            container(warning_col)
                .padding(theme::SPACING_LG)
                .width(Length::Fill)
                .style(move |_theme: &iced::Theme| container::Style {
                    background: Some(iced::Background::Color(
                        Color::from_rgba(
                            theme::warning_color(app_theme).r,
                            theme::warning_color(app_theme).g,
                            theme::warning_color(app_theme).b,
                            0.2,
                        )
                    )),
                    border: iced::Border {
                        color: theme::warning_color(app_theme),
                        width: 1.0,
                        radius: theme::RADIUS_MD.into(),
                    },
                    ..Default::default()
                })
        );
    }

    // Two-column form layout
    let left_column = column![
        make_input("Item Name *", "Enter item name", name, Message::NameChanged),
        make_input("SKU *", "Enter SKU", sku, Message::SkuChanged),
        make_input("Category *", "e.g., Electronics, Food", category, Message::CategoryChanged),
        make_input("Supplier", "Enter supplier name", supplier, Message::SupplierChanged),
    ]
    .spacing(theme::SPACING_LG)
    .width(Length::Fill);

    let right_column = column![
        make_input("Quantity *", "Enter quantity", quantity, Message::QuantityChanged),
        make_input("Price ($) *", "Enter price", price, Message::PriceChanged),
        column![
            text("Description")
                .size(theme::TEXT_BODY)
                .style(move |_theme: &iced::Theme| text::Style {
                    color: Some(theme::text_color(app_theme)),
                }),
            text_input("Enter item description (optional)", description)
                .on_input(Message::DescriptionChanged)
                .padding(theme::SPACING_LG)
                .size(theme::TEXT_BODY)
                .width(Length::Fill)
                .style(move |theme_iced: &iced::Theme, status: text_input::Status| {
                    let default_style = text_input::default(theme_iced, status);
                    text_input::Style {
                        background: iced::Background::Color(theme::surface_elevated_color(app_theme)),
                        border: iced::Border {
                            color: match status {
                                text_input::Status::Focused => theme::primary_color(app_theme),
                                _ => theme::border_color(app_theme),
                            },
                            width: if matches!(status, text_input::Status::Focused) { 2.0 } else { 1.0 },
                            radius: theme::RADIUS_MD.into(),
                        },
                        icon: default_style.icon,
                        placeholder: theme::text_tertiary_color(app_theme),
                        value: theme::text_color(app_theme),
                        selection: theme::primary_color(app_theme),
                    }
                }),
        ]
        .spacing(theme::SPACING_SM),
    ]
    .spacing(theme::SPACING_LG)
    .width(Length::Fill);

    let two_col_row = row![left_column, right_column]
        .spacing(theme::SPACING_2XL);

    form_content = form_content.push(two_col_row);

    // Action buttons
    let submit_btn = button(
        row![
            icons::Icon::Check.view(icons::IconSize::Small, app_theme),
            text(match mode {
                ItemDialogMode::Add => "Add Item",
                ItemDialogMode::Edit(_) => "Save Changes",
            })
            .size(theme::TEXT_BODY_LARGE),
        ]
        .spacing(theme::SPACING_SM)
        .align_y(iced::Alignment::Center)
    )
    .on_press(Message::SubmitItem)
    .padding([theme::SPACING_MD, theme::SPACING_2XL])
    .style(move |_theme: &iced::Theme, status: button::Status| {
        let bg_color = match status {
            button::Status::Hovered => Color::from_rgba(
                theme::success_color(app_theme).r,
                theme::success_color(app_theme).g,
                theme::success_color(app_theme).b,
                0.9,
            ),
            _ => theme::success_color(app_theme),
        };
        
        button::Style {
            background: Some(iced::Background::Color(bg_color)),
            text_color: Color::WHITE,
            border: iced::Border {
                radius: theme::RADIUS_MD.into(),
                ..Default::default()
            },
            shadow: iced::Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                offset: iced::Vector::new(0.0, 2.0),
                blur_radius: 6.0,
            },
            ..Default::default()
        }
    });

    let cancel_btn = button(
        text("✕ Cancel")
            .size(theme::TEXT_BODY_LARGE)
    )
    .on_press(Message::CloseItemDialog)
    .padding([theme::SPACING_MD, theme::SPACING_2XL])
    .style(move |_theme: &iced::Theme, status: button::Status| {
        let bg_color = match status {
            button::Status::Hovered => theme::surface_elevated_color(app_theme),
            _ => theme::surface_color(app_theme),
        };
        
        button::Style {
            background: Some(iced::Background::Color(bg_color)),
            text_color: theme::text_color(app_theme),
            border: iced::Border {
                color: theme::border_color(app_theme),
                width: 1.0,
                radius: theme::RADIUS_MD.into(),
            },
            ..Default::default()
        }
    });

    form_content = form_content.push(
        row![
            cancel_btn,
            iced::widget::horizontal_space(),
            submit_btn,
        ]
        .spacing(theme::SPACING_LG)
        .align_y(iced::Alignment::Center)
    );

    form_content = form_content.push(
        container(
            text("* Required fields")
                .size(theme::TEXT_CAPTION)
                .style(move |_theme: &iced::Theme| text::Style {
                    color: Some(theme::text_tertiary_color(app_theme)),
                })
        )
        .width(Length::Fill)
        .center_x(Length::Fill)
    );

    // Modal dialog
    container(
        container(
            scrollable(form_content)
                .width(Length::Fill)
                .height(Length::Fixed(650.0))
        )
        .padding(theme::SPACING_3XL)
        .max_width(900)
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(theme::surface_color(app_theme))),
            border: iced::Border {
                color: theme::primary_color(app_theme),
                width: 2.0,
                radius: theme::RADIUS_LG.into(),
            },
            shadow: iced::Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.4),
                offset: iced::Vector::new(0.0, 12.0),
                blur_radius: 48.0,
            },
            ..Default::default()
        })
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .style(move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.7))),
        ..Default::default()
    })
    .into()
}
