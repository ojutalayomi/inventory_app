use iced::widget::{button, column, container, pick_list, row, scrollable, text, text_input, Column};
use iced::{Element, Length};

use crate::inventory::InventoryItem;
use crate::messages::Message;
use crate::search::{SearchFilter, SortField};

pub fn view<'a>(
    items: &'a [InventoryItem],
    all_items: &'a [InventoryItem],
    filter: &'a SearchFilter,
    show_search_panel: bool,
) -> Element<'a, Message> {
    let title = text("Inventory Management").size(32);

    let add_button = button("+ Add Item")
        .on_press(Message::OpenAddDialog)
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
        );

    let search_toggle_text = if show_search_panel {
        "🔍 Hide Filters"
    } else {
        "🔍 Show Filters"
    };
    
    let search_button = button(search_toggle_text)
        .on_press(Message::ToggleSearchPanel)
        .padding(10)
        .style(move |_theme: &iced::Theme, _status: iced::widget::button::Status| {
            iced::widget::button::Style {
                background: Some(iced::Background::Color(if filter.is_active() {
                    iced::Color::from_rgb(0.3, 0.5, 0.7)
                } else {
                    iced::Color::from_rgb(0.3, 0.3, 0.3)
                })),
                border: iced::Border {
                    color: if filter.is_active() {
                        iced::Color::from_rgb(0.4, 0.6, 0.8)
                    } else {
                        iced::Color::from_rgb(0.4, 0.4, 0.4)
                    },
                    width: 1.0,
                    radius: 5.0.into(),
                },
                text_color: iced::Color::WHITE,
                ..Default::default()
            }
        });

    let header = row![title, add_button, search_button]
        .spacing(15)
        .padding(10);

    let mut content = column![header];

    // Search panel
    if show_search_panel {
        let search_panel = build_search_panel(filter, all_items);
        content = content.push(search_panel);
    }

    // Statistics bar
    let total_items = all_items.len();
    let filtered_count = items.len();
    let total_value: f64 = items.iter().map(|i| i.price * i.quantity as f64).sum();
    let total_quantity: u32 = items.iter().map(|i| i.quantity).sum();

    let stats_text = if filter.is_active() {
        format!(
            "Showing {} of {} items | Total Value: ${:.2} | Total Quantity: {}",
            filtered_count, total_items, total_value, total_quantity
        )
    } else {
        format!(
            "Total: {} items | Value: ${:.2} | Quantity: {}",
            total_items, total_value, total_quantity
        )
    };

    let stats = container(
        text(stats_text).size(14).style(|_theme: &iced::Theme| {
            iced::widget::text::Style {
                color: Some(iced::Color::from_rgb(0.7, 0.7, 0.7)),
            }
        }),
    )
    .padding(10)
    .style(|_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(iced::Color::from_rgb(
            0.12, 0.12, 0.12,
        ))),
        border: iced::Border {
            color: iced::Color::from_rgb(0.3, 0.3, 0.3),
            width: 1.0,
            radius: 5.0.into(),
        },
        ..Default::default()
    });

    content = content.push(stats);

    // Items list
    if items.is_empty() {
        let empty_msg = if filter.is_active() {
            "No items match your filters"
        } else {
            "No items in inventory. Click 'Add Item' to get started!"
        };

        content = content.push(
            container(
                text(empty_msg).size(16).style(|_theme: &iced::Theme| {
                    iced::widget::text::Style {
                        color: Some(iced::Color::from_rgb(0.5, 0.5, 0.5)),
                    }
                }),
            )
            .padding(40)
            .width(Length::Fill)
            .center_x(Length::Fill),
        );
    } else {
        let mut items_list = Column::new().spacing(10).padding(10);

        for item in items {
            let item_card = build_item_card(item);
            items_list = items_list.push(item_card);
        }

        content = content.push(scrollable(items_list).height(500));
    }

    content.into()
}

fn build_search_panel<'a>(
    filter: &'a SearchFilter,
    all_items: &'a [InventoryItem],
) -> Element<'a, Message> {
    let categories = SearchFilter::get_unique_categories(all_items);
    let suppliers = SearchFilter::get_unique_suppliers(all_items);

    let mut category_options = vec!["All Categories".to_string()];
    category_options.extend(categories);

    let mut supplier_options = vec!["All Suppliers".to_string()];
    supplier_options.extend(suppliers);

    let search_input = text_input("Search by name, SKU, category, supplier...", &filter.query)
        .on_input(Message::SearchQueryChanged)
        .padding(8)
        .width(Length::Fill);

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
    .padding(8)
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
    .padding(8)
    .width(Length::Fill);

    let min_qty_str = filter.min_quantity.map_or(String::new(), |v| v.to_string());
    let max_qty_str = filter.max_quantity.map_or(String::new(), |v| v.to_string());
    let min_price_str = filter.min_price.map_or(String::new(), |v| format!("{:.2}", v));
    let max_price_str = filter.max_price.map_or(String::new(), |v| format!("{:.2}", v));

    let min_qty_input = text_input("Min", &min_qty_str)
        .on_input(Message::MinQuantityChanged)
        .padding(8)
        .width(Length::Fixed(100.0));

    let max_qty_input = text_input("Max", &max_qty_str)
        .on_input(Message::MaxQuantityChanged)
        .padding(8)
        .width(Length::Fixed(100.0));

    let min_price_input = text_input("Min $", &min_price_str)
        .on_input(Message::MinPriceChanged)
        .padding(8)
        .width(Length::Fixed(100.0));

    let max_price_input = text_input("Max $", &max_price_str)
        .on_input(Message::MaxPriceChanged)
        .padding(8)
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
    .padding(8)
    .width(Length::Fixed(150.0));

    let sort_dir_button = button(text(format!("{}", filter.sort_direction)))
        .on_press(Message::SortDirectionToggled)
        .padding(8);

    let clear_button = button("Clear All Filters")
        .on_press(Message::ClearFilters)
        .padding(8)
        .style(|_theme: &iced::Theme, _status: iced::widget::button::Status| {
            iced::widget::button::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb(
                    0.6, 0.3, 0.3,
                ))),
                text_color: iced::Color::WHITE,
                border: iced::Border {
                    radius: 5.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        });

    let panel = container(
        column![
            row![
                text("Search:").size(14).width(Length::Fixed(80.0)),
                search_input,
            ]
            .spacing(10)
            .align_y(iced::Alignment::Center),
            row![
                text("Category:").size(14).width(Length::Fixed(80.0)),
                category_picker,
            ]
            .spacing(10)
            .align_y(iced::Alignment::Center),
            row![
                text("Supplier:").size(14).width(Length::Fixed(80.0)),
                supplier_picker,
            ]
            .spacing(10)
            .align_y(iced::Alignment::Center),
            row![
                text("Quantity:").size(14).width(Length::Fixed(80.0)),
                min_qty_input,
                text("-").size(14),
                max_qty_input,
            ]
            .spacing(10)
            .align_y(iced::Alignment::Center),
            row![
                text("Price:").size(14).width(Length::Fixed(80.0)),
                min_price_input,
                text("-").size(14),
                max_price_input,
            ]
            .spacing(10)
            .align_y(iced::Alignment::Center),
            row![
                text("Sort by:").size(14).width(Length::Fixed(80.0)),
                sort_field_picker,
                sort_dir_button,
                clear_button,
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

    panel.into()
}

fn build_item_card<'a>(item: &'a InventoryItem) -> Element<'a, Message> {
    let created = chrono::DateTime::from_timestamp(item.created_at, 0)
        .map(|dt| dt.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let updated = chrono::DateTime::from_timestamp(item.updated_at, 0)
        .map(|dt| dt.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let stock_status = if item.quantity == 0 {
        ("⚠️ OUT OF STOCK", iced::Color::from_rgb(0.9, 0.3, 0.3))
    } else if item.quantity < 10 {
        ("⚠️ LOW STOCK", iced::Color::from_rgb(0.9, 0.7, 0.3))
    } else {
        ("✓ In Stock", iced::Color::from_rgb(0.3, 0.8, 0.3))
    };

    container(
        column![
            row![
                column![
                    text(&item.name).size(20),
                    text(format!("SKU: {}", item.sku)).size(12).style(
                        |_theme: &iced::Theme| iced::widget::text::Style {
                            color: Some(iced::Color::from_rgb(0.7, 0.7, 0.7)),
                        }
                    ),
                ]
                .spacing(5)
                .width(Length::Fill),
                column![
                    text(stock_status.0)
                        .size(14)
                        .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                            color: Some(stock_status.1),
                        }),
                    text(format!("Qty: {}", item.quantity)).size(16),
                ]
                .spacing(3)
                .align_x(iced::Alignment::End),
            ]
            .spacing(10),
            text("").size(5),
            row![
                column![
                    text(format!("Category: {}", item.category)).size(13),
                    text(format!("Supplier: {}", item.supplier)).size(13),
                ]
                .spacing(5)
                .width(Length::Fill),
                column![
                    text(format!("Price: ${:.2}", item.price)).size(16),
                    text(format!("Total: ${:.2}", item.price * item.quantity as f64))
                        .size(14)
                        .style(|_theme: &iced::Theme| iced::widget::text::Style {
                            color: Some(iced::Color::from_rgb(0.3, 0.8, 0.3)),
                        }),
                ]
                .spacing(3)
                .align_x(iced::Alignment::End),
            ]
            .spacing(10),
            {
                let mut desc_col = Column::new();
                if !item.description.is_empty() {
                    desc_col = desc_col.push(text("").size(3));
                    desc_col = desc_col.push(
                        text(&item.description).size(12).style(
                            |_theme: &iced::Theme| iced::widget::text::Style {
                                color: Some(iced::Color::from_rgb(0.6, 0.6, 0.6)),
                            }
                        )
                    );
                }
                desc_col
            },
            text("").size(5),
            row![
                text(format!("Created: {}", created)).size(11).style(
                    |_theme: &iced::Theme| iced::widget::text::Style {
                        color: Some(iced::Color::from_rgb(0.5, 0.5, 0.5)),
                    }
                ),
                text(format!("Updated: {}", updated)).size(11).style(
                    |_theme: &iced::Theme| iced::widget::text::Style {
                        color: Some(iced::Color::from_rgb(0.5, 0.5, 0.5)),
                    }
                ),
                row![
                    button("Edit")
                        .on_press(Message::OpenEditDialog(item.id.clone()))
                        .padding(5),
                    button("Delete")
                        .on_press(Message::DeleteItem(item.id.clone()))
                        .padding(5)
                        .style(
                            |_theme: &iced::Theme, _status: iced::widget::button::Status| {
                                iced::widget::button::Style {
                                    background: Some(iced::Background::Color(
                                        iced::Color::from_rgb(0.7, 0.2, 0.2),
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
                .spacing(5),
            ]
            .spacing(15)
            .align_y(iced::Alignment::Center),
        ]
        .spacing(5)
        .padding(15),
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
