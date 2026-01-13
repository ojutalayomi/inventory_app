use iced::widget::{button, column, container, pick_list, row, scrollable, text, text_input, Column};
use iced::{Color, Element, Length};

use crate::inventory::InventoryItem;
use crate::messages::{AppTheme, Message};
use crate::search::{SearchFilter, SortField};
use crate::theme;
use crate::icons;

pub fn view<'a>(
    items: &'a [InventoryItem],
    all_items: &'a [InventoryItem],
    filter: &'a SearchFilter,
    show_search_panel: bool,
    app_theme: &'a AppTheme,
) -> Element<'a, Message> {
    // Calculate statistics
    let total_items = all_items.len();
    let filtered_count = items.len();
    let total_value: f64 = items.iter().map(|i| i.price * i.quantity as f64).sum();
    let low_stock_count = items.iter().filter(|i| i.quantity > 0 && i.quantity < 10).count();

    // Page title with gradient-style text
    let title = row![
        icons::Icon::Inventory.view(icons::IconSize::Large, app_theme),
        text("Inventory Management")
            .size(theme::TEXT_H1)
            .style(move |_theme: &iced::Theme| text::Style {
                color: Some(theme::primary_color(app_theme)),
            }),
    ]
    .spacing(theme::SPACING_SM)
    .align_y(iced::Alignment::Center);

    // Action buttons
    let add_button = button(
        text("+ Add Item").size(theme::TEXT_BODY)
    )
    .on_press(Message::OpenAddDialog)
    .padding([theme::SPACING_MD, theme::SPACING_XL])
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

    let search_toggle_text = if show_search_panel {
        "Hide Filters"
    } else {
        "Show Filters"
    };
    
    let search_button = button(
        row![
            icons::Icon::Search.view(icons::IconSize::Small, app_theme),
            text(search_toggle_text).size(theme::TEXT_BODY),
        ]
        .spacing(theme::SPACING_SM)
        .align_y(iced::Alignment::Center)
    )
        .on_press(Message::ToggleSearchPanel)
        .padding([theme::SPACING_MD, theme::SPACING_XL])
        .style(move |_theme: &iced::Theme, status: button::Status| {
            let is_active = filter.is_active();
            let base_color = if is_active {
                theme::accent_color(app_theme)
            } else {
                theme::surface_elevated_color(app_theme)
            };
            
            let bg_color = match status {
                button::Status::Hovered => if is_active {
                    Color::from_rgba(base_color.r, base_color.g, base_color.b, 0.9)
                } else {
                    theme::surface_color(app_theme)
                },
                _ => base_color,
            };
            
            button::Style {
                background: Some(iced::Background::Color(bg_color)),
                text_color: if is_active { Color::WHITE } else { theme::text_color(app_theme) },
                border: iced::Border {
                    color: if is_active { theme::accent_color(app_theme) } else { theme::border_color(app_theme) },
                    width: if is_active { 2.0 } else { 1.0 },
                    radius: theme::RADIUS_MD.into(),
                },
                ..Default::default()
            }
        });

    let header = row![
        title,
        iced::widget::horizontal_space(),
        row![search_button, add_button].spacing(theme::SPACING_MD)
    ]
    .spacing(theme::SPACING_XL)
    .align_y(iced::Alignment::Center)
    .padding(theme::SPACING_LG);

    let stats_row = row![
        make_stat_card(
            icons::Icon::Chart,
            "Total Items".to_string(),
            format!("{}", total_items),
            format!("Showing {}", filtered_count),
            app_theme,
        ),
        make_stat_card(
            icons::Icon::Dollar,
            "Total Value".to_string(),
            format!("${:.2}", total_value),
            "Filtered items".to_string(),
            app_theme,
        ),
        make_stat_card(
            icons::Icon::AlertTriangle,
            "Low Stock".to_string(),
            format!("{}", low_stock_count),
            "Items below 10".to_string(),
            app_theme,
        ),
    ]
    .spacing(theme::SPACING_LG)
    .padding([0.0, theme::SPACING_LG]);

    let mut content = column![header, stats_row].spacing(theme::SPACING_LG);

    // Search panel
    if show_search_panel {
        let search_panel = build_search_panel(filter, all_items, app_theme);
        content = content.push(search_panel);
    }

    // Items list
    if items.is_empty() {
        let (empty_icon, empty_title, empty_subtitle) = if filter.is_active() {
            (icons::Icon::Search, "No items match your filters", "Try adjusting your filter criteria")
        } else {
            (icons::Icon::Inventory, "No items in inventory", "Click 'Add Item' to get started!")
        };

        content = content.push(
            container(
                column![
                    empty_icon.view(icons::IconSize::XLarge, app_theme),
                    text(empty_title)
                        .size(theme::TEXT_H2)
                        .style(move |_theme: &iced::Theme| text::Style {
                            color: Some(theme::text_color(app_theme)),
                        }),
                    text(empty_subtitle)
                        .size(theme::TEXT_BODY)
                        .style(move |_theme: &iced::Theme| text::Style {
                            color: Some(theme::text_secondary_color(app_theme)),
                        }),
                ]
                .spacing(theme::SPACING_LG)
                .align_x(iced::Alignment::Center)
            )
            .padding(theme::SPACING_3XL)
            .width(Length::Fill)
            .center_x(Length::Fill)
        );
    } else {
        let mut items_list = Column::new().spacing(theme::SPACING_LG).padding([0.0, theme::SPACING_LG]);

        for item in items {
            let item_card = build_item_card(item, app_theme);
            items_list = items_list.push(item_card);
        }

        content = content.push(items_list);
    }

    // Make the entire page scrollable
    scrollable(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn make_stat_card(
    icon: icons::Icon,
    label: String,
    value: String,
    subtitle: String,
    app_theme: &AppTheme,
) -> Element<Message> {
    container(
        column![
            row![
                icon.view(icons::IconSize::Large, app_theme),
                column![
                    text(label)
                        .size(theme::TEXT_CAPTION)
                        .style(move |_theme: &iced::Theme| text::Style {
                            color: Some(theme::text_secondary_color(app_theme)),
                        }),
                    text(value)
                        .size(theme::TEXT_H2)
                        .style(move |_theme: &iced::Theme| text::Style {
                            color: Some(theme::text_color(app_theme)),
                        }),
                ]
                .spacing(theme::SPACING_XS),
            ]
            .spacing(theme::SPACING_MD)
            .align_y(iced::Alignment::Center),
            text(subtitle)
                .size(theme::TEXT_CAPTION)
                .style(move |_theme: &iced::Theme| text::Style {
                    color: Some(theme::text_tertiary_color(app_theme)),
                }),
        ]
        .spacing(theme::SPACING_SM)
    )
    .padding(theme::SPACING_XL)
    .width(Length::Fill)
    .style(move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(theme::surface_color(app_theme))),
        border: iced::Border {
            color: theme::border_color(app_theme),
            width: 1.0,
            radius: theme::RADIUS_LG.into(),
        },
        shadow: iced::Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.05),
            offset: iced::Vector::new(0.0, 2.0),
            blur_radius: 8.0,
        },
        ..Default::default()
    })
    .into()
}

fn build_search_panel<'a>(
    filter: &'a SearchFilter,
    all_items: &'a [InventoryItem],
    app_theme: &'a AppTheme,
) -> Element<'a, Message> {
    let categories = SearchFilter::get_unique_categories(all_items);
    let suppliers = SearchFilter::get_unique_suppliers(all_items);

    let mut category_options = vec!["All Categories".to_string()];
    category_options.extend(categories);

    let mut supplier_options = vec!["All Suppliers".to_string()];
    supplier_options.extend(suppliers);

    let search_input = text_input("Search by name, SKU, category, supplier...", &filter.query)
        .on_input(Message::SearchQueryChanged)
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
        });

    let category_picker = pick_list(
        category_options,
        filter
            .category_filter
            .clone()
            .or(Some("All Categories".to_string())),
        |val| {
            if val == "All Categories" {
                Message::CategoryFilterChanged(String::new())
            } else {
                Message::CategoryFilterChanged(val)
            }
        },
    )
    .padding(theme::SPACING_LG)
    .width(Length::Fill);

    let supplier_picker = pick_list(
        supplier_options,
        filter
            .supplier_filter
            .clone()
            .or(Some("All Suppliers".to_string())),
        |val| {
            if val == "All Suppliers" {
                Message::SupplierFilterChanged(String::new())
            } else {
                Message::SupplierFilterChanged(val)
            }
        },
    )
    .padding(theme::SPACING_LG)
    .width(Length::Fill);

    let min_qty_str = filter.min_quantity.map_or(String::new(), |v| v.to_string());
    let max_qty_str = filter.max_quantity.map_or(String::new(), |v| v.to_string());
    let min_price_str = filter.min_price.map_or(String::new(), |v| format!("{:.2}", v));
    let max_price_str = filter.max_price.map_or(String::new(), |v| format!("{:.2}", v));

    let min_qty_input = text_input("Min", &min_qty_str)
        .on_input(Message::MinQuantityChanged)
        .padding(theme::SPACING_MD)
        .size(theme::TEXT_BODY)
        .width(Length::Fixed(100.0));

    let max_qty_input = text_input("Max", &max_qty_str)
        .on_input(Message::MaxQuantityChanged)
        .padding(theme::SPACING_MD)
        .size(theme::TEXT_BODY)
        .width(Length::Fixed(100.0));

    let min_price_input = text_input("Min $", &min_price_str)
        .on_input(Message::MinPriceChanged)
        .padding(theme::SPACING_MD)
        .size(theme::TEXT_BODY)
        .width(Length::Fixed(100.0));

    let max_price_input = text_input("Max $", &max_price_str)
        .on_input(Message::MaxPriceChanged)
        .padding(theme::SPACING_MD)
        .size(theme::TEXT_BODY)
        .width(Length::Fixed(100.0));

    let sort_fields = vec![
        SortField::Name,
        SortField::Sku,
        SortField::Category,
        SortField::Supplier,
        SortField::Quantity,
        SortField::Price,
        SortField::CreatedAt,
        SortField::UpdatedAt,
    ];

    let sort_field_picker = pick_list(
        sort_fields,
        filter.sort_field.clone(),
        Message::SortFieldChanged,
    )
    .padding(theme::SPACING_MD)
    .width(Length::Fixed(150.0));

    let sort_dir_button = button(text(format!("{}", filter.sort_direction)))
        .on_press(Message::SortDirectionToggled)
        .padding(theme::SPACING_MD);

    let clear_button = button(
        row![
            icons::Icon::Delete.view(icons::IconSize::Small, app_theme),
            text("Clear Filters").size(theme::TEXT_BODY),
        ]
        .spacing(theme::SPACING_SM)
        .align_y(iced::Alignment::Center)
    )
        .on_press(Message::ClearFilters)
        .padding([theme::SPACING_MD, theme::SPACING_LG])
        .style(move |_theme: &iced::Theme, status: button::Status| {
            let bg_color = match status {
                button::Status::Hovered => theme::danger_color(app_theme),
                _ => Color::from_rgba(
                    theme::danger_color(app_theme).r,
                    theme::danger_color(app_theme).g,
                    theme::danger_color(app_theme).b,
                    0.8,
                ),
            };
            
            button::Style {
                background: Some(iced::Background::Color(bg_color)),
                text_color: Color::WHITE,
                border: iced::Border {
                    radius: theme::RADIUS_MD.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        });

    let panel = container(
        column![
            row![
                row![
                    icons::Icon::Search.view(icons::IconSize::Small, app_theme),
                    text("Search:")
                        .size(theme::TEXT_BODY)
                        .style(move |_theme: &iced::Theme| text::Style {
                            color: Some(theme::text_color(app_theme)),
                        }),
                ]
                .spacing(theme::SPACING_XS)
                .align_y(iced::Alignment::Center)
                .width(Length::Fixed(100.0)),
                search_input,
            ]
            .spacing(theme::SPACING_MD)
            .align_y(iced::Alignment::Center),
            row![
                column![
                    text("Category")
                        .size(theme::TEXT_CAPTION)
                        .style(move |_theme: &iced::Theme| text::Style {
                            color: Some(theme::text_secondary_color(app_theme)),
                        }),
                    category_picker,
                ]
                .spacing(theme::SPACING_XS)
                .width(Length::Fill),
                column![
                    text("Supplier")
                        .size(theme::TEXT_CAPTION)
                        .style(move |_theme: &iced::Theme| text::Style {
                            color: Some(theme::text_secondary_color(app_theme)),
                        }),
                    supplier_picker,
                ]
                .spacing(theme::SPACING_XS)
                .width(Length::Fill),
            ]
            .spacing(theme::SPACING_LG),
            row![
                column![
                    text("Quantity Range")
                        .size(theme::TEXT_CAPTION)
                        .style(move |_theme: &iced::Theme| text::Style {
                            color: Some(theme::text_secondary_color(app_theme)),
                        }),
                    row![min_qty_input, text("-").size(theme::TEXT_BODY), max_qty_input]
                        .spacing(theme::SPACING_SM)
                        .align_y(iced::Alignment::Center),
                ]
                .spacing(theme::SPACING_XS)
                .width(Length::Fill),
                column![
                    text("Price Range")
                        .size(theme::TEXT_CAPTION)
                        .style(move |_theme: &iced::Theme| text::Style {
                            color: Some(theme::text_secondary_color(app_theme)),
                        }),
                    row![min_price_input, text("-").size(theme::TEXT_BODY), max_price_input]
                        .spacing(theme::SPACING_SM)
                        .align_y(iced::Alignment::Center),
                ]
                .spacing(theme::SPACING_XS)
                .width(Length::Fill),
            ]
            .spacing(theme::SPACING_LG),
            row![
                text("Sort by:")
                    .size(theme::TEXT_BODY)
                    .style(move |_theme: &iced::Theme| text::Style {
                        color: Some(theme::text_color(app_theme)),
                    }),
                sort_field_picker,
                sort_dir_button,
                iced::widget::horizontal_space(),
                clear_button,
            ]
            .spacing(theme::SPACING_MD)
            .align_y(iced::Alignment::Center),
        ]
        .spacing(theme::SPACING_LG)
        .padding(theme::SPACING_XL),
    )
    .padding([0.0, theme::SPACING_LG])
    .width(Length::Fill)
    .style(move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(theme::surface_color(app_theme))),
        border: iced::Border {
            color: theme::border_color(app_theme),
            width: 1.0,
            radius: theme::RADIUS_LG.into(),
        },
        shadow: iced::Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.05),
            offset: iced::Vector::new(0.0, 2.0),
            blur_radius: 8.0,
        },
        ..Default::default()
    });

    panel.into()
}

fn build_item_card<'a>(item: &'a InventoryItem, app_theme: &'a AppTheme) -> Element<'a, Message> {
    let created = chrono::DateTime::from_timestamp(item.created_at, 0)
        .map(|dt| dt.format("%b %d, %Y").to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let updated = chrono::DateTime::from_timestamp(item.updated_at, 0)
        .map(|dt| dt.format("%b %d, %Y").to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let (stock_icon, stock_label, stock_color) = if item.quantity == 0 {
        (icons::Icon::XCircle, "OUT OF STOCK", theme::danger_color(app_theme))
    } else if item.quantity < 5 {
        (icons::Icon::AlertTriangle, "CRITICALLY LOW", theme::danger_color(app_theme))
    } else if item.quantity < 10 {
        (icons::Icon::AlertCircle, "LOW STOCK", theme::warning_color(app_theme))
    } else {
        (icons::Icon::CheckCircle, "IN STOCK", theme::success_color(app_theme))
    };

    let category_color = theme::category_color(&item.category, app_theme);

    let edit_button = button(
        row![
            icons::Icon::Edit.view(icons::IconSize::Small, app_theme),
            text("Edit").size(theme::TEXT_BODY),
        ]
        .spacing(theme::SPACING_SM)
        .align_y(iced::Alignment::Center)
    )
        .on_press(Message::OpenEditDialog(item.id.clone()))
        .padding([theme::SPACING_SM, theme::SPACING_LG])
        .style(move |_theme: &iced::Theme, status: button::Status| {
            let bg_color = match status {
                button::Status::Hovered => theme::primary_dark_color(app_theme),
                _ => theme::primary_color(app_theme),
            };
            
            button::Style {
                background: Some(iced::Background::Color(bg_color)),
                text_color: Color::WHITE,
                border: iced::Border {
                    radius: theme::RADIUS_MD.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        });

    let delete_button = button(
        row![
            icons::Icon::Delete.view(icons::IconSize::Small, app_theme),
            text("Delete").size(theme::TEXT_BODY),
        ]
        .spacing(theme::SPACING_SM)
        .align_y(iced::Alignment::Center)
    )
        .on_press(Message::DeleteItem(item.id.clone()))
        .padding([theme::SPACING_SM, theme::SPACING_LG])
        .style(move |_theme: &iced::Theme, status: button::Status| {
            let bg_color = match status {
                button::Status::Hovered => theme::danger_color(app_theme),
                _ => Color::from_rgba(
                    theme::danger_color(app_theme).r,
                    theme::danger_color(app_theme).g,
                    theme::danger_color(app_theme).b,
                    0.8,
                ),
            };
            
            button::Style {
                background: Some(iced::Background::Color(bg_color)),
                text_color: Color::WHITE,
                border: iced::Border {
                    radius: theme::RADIUS_MD.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        });

    container(
        row![
            // Category color indicator
            container(text(""))
                .width(4)
                .height(Length::Fill)
                .style(move |_theme: &iced::Theme| container::Style {
                    background: Some(iced::Background::Color(category_color)),
                    border: iced::Border {
                        radius: theme::RADIUS_LG.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            // Main content
            column![
                // Header row: Name, SKU, Stock status
                row![
                    column![
                        text(&item.name)
                            .size(theme::TEXT_H3)
                            .style(move |_theme: &iced::Theme| text::Style {
                                color: Some(theme::text_color(app_theme)),
                            }),
                        text(format!("SKU: {}", item.sku))
                            .size(theme::TEXT_CAPTION)
                            .style(move |_theme: &iced::Theme| text::Style {
                                color: Some(theme::text_tertiary_color(app_theme)),
                            }),
                    ]
                    .spacing(theme::SPACING_XS)
                    .width(Length::Fill),
                    container(
                        row![
                            stock_icon.view_with_color(icons::IconSize::Medium, Some(stock_color), app_theme),
                            text(stock_label).size(theme::TEXT_BODY),
                        ]
                        .spacing(theme::SPACING_SM)
                        .align_y(iced::Alignment::Center)
                    )
                    .padding([theme::SPACING_SM, theme::SPACING_LG])
                    .style(move |_theme: &iced::Theme| container::Style {
                        background: Some(iced::Background::Color(
                            Color::from_rgba(stock_color.r, stock_color.g, stock_color.b, 0.2)
                        )),
                        border: iced::Border {
                            color: stock_color,
                            width: 1.0,
                            radius: theme::RADIUS_FULL.into(),
                        },
                        ..Default::default()
                    }),
                ]
                .spacing(theme::SPACING_XL)
                .align_y(iced::Alignment::Center),
                // Details row: Category, Supplier, Quantity, Price
                row![
                    column![
                        text("Category")
                            .size(theme::TEXT_CAPTION)
                            .style(move |_theme: &iced::Theme| text::Style {
                                color: Some(theme::text_tertiary_color(app_theme)),
                            }),
                        text(&item.category)
                            .size(theme::TEXT_BODY)
                            .style(move |_theme: &iced::Theme| text::Style {
                                color: Some(theme::text_color(app_theme)),
                            }),
                    ]
                    .spacing(theme::SPACING_XS)
                    .width(Length::Fill),
                    column![
                        text("Supplier")
                            .size(theme::TEXT_CAPTION)
                            .style(move |_theme: &iced::Theme| text::Style {
                                color: Some(theme::text_tertiary_color(app_theme)),
                            }),
                        text(&item.supplier)
                            .size(theme::TEXT_BODY)
                            .style(move |_theme: &iced::Theme| text::Style {
                                color: Some(theme::text_color(app_theme)),
                            }),
                    ]
                    .spacing(theme::SPACING_XS)
                    .width(Length::Fill),
                    column![
                        text("Quantity")
                            .size(theme::TEXT_CAPTION)
                            .style(move |_theme: &iced::Theme| text::Style {
                                color: Some(theme::text_tertiary_color(app_theme)),
                            }),
                        text(format!("{}", item.quantity))
                            .size(theme::TEXT_H3)
                            .style(move |_theme: &iced::Theme| text::Style {
                                color: Some(stock_color),
                            }),
                    ]
                    .spacing(theme::SPACING_XS)
                    .align_x(iced::Alignment::End),
                    column![
                        text("Price")
                            .size(theme::TEXT_CAPTION)
                            .style(move |_theme: &iced::Theme| text::Style {
                                color: Some(theme::text_tertiary_color(app_theme)),
                            }),
                        text(format!("${:.2}", item.price))
                            .size(theme::TEXT_H3)
                            .style(move |_theme: &iced::Theme| text::Style {
                                color: Some(theme::primary_color(app_theme)),
                            }),
                    ]
                    .spacing(theme::SPACING_XS)
                    .align_x(iced::Alignment::End),
                ]
                .spacing(theme::SPACING_XL),
                // Description if present
                {
                    let mut desc_container = column![];
                    if !item.description.is_empty() {
                        desc_container = desc_container.push(
                            container(
                                text(&item.description)
                                    .size(theme::TEXT_BODY)
                                    .style(move |_theme: &iced::Theme| text::Style {
                                        color: Some(theme::text_secondary_color(app_theme)),
                                    })
                            )
                            .padding(theme::SPACING_LG)
                            .width(Length::Fill)
                            .style(move |_theme: &iced::Theme| container::Style {
                                background: Some(iced::Background::Color(theme::surface_elevated_color(app_theme))),
                                border: iced::Border {
                                    color: theme::border_color(app_theme),
                                    width: 1.0,
                                    radius: theme::RADIUS_MD.into(),
                                },
                                ..Default::default()
                            })
                        );
                    }
                    desc_container
                },
                // Footer: Timestamps and actions
                row![
                    column![
                        text(format!("Created: {}", created))
                            .size(theme::TEXT_CAPTION)
                            .style(move |_theme: &iced::Theme| text::Style {
                                color: Some(theme::text_tertiary_color(app_theme)),
                            }),
                        text(format!("Updated: {}", updated))
                            .size(theme::TEXT_CAPTION)
                            .style(move |_theme: &iced::Theme| text::Style {
                                color: Some(theme::text_tertiary_color(app_theme)),
                            }),
                    ]
                    .spacing(theme::SPACING_XS)
                    .width(Length::Fill),
                    row![edit_button, delete_button].spacing(theme::SPACING_MD),
                ]
                .spacing(theme::SPACING_XL)
                .align_y(iced::Alignment::Center),
            ]
            .spacing(theme::SPACING_LG)
            .padding(theme::SPACING_XL)
            .width(Length::Fill),
        ]
    )
    .width(Length::Fill)
    .style(move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(theme::surface_color(app_theme))),
        border: iced::Border {
            color: theme::border_color(app_theme),
            width: 1.0,
            radius: theme::RADIUS_LG.into(),
        },
        shadow: iced::Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.08),
            offset: iced::Vector::new(0.0, 2.0),
            blur_radius: 8.0,
        },
        ..Default::default()
    })
    .into()
}
