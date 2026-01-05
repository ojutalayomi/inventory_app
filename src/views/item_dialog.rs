use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Element, Length};

use crate::messages::{ItemDialogMode, Message};

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
) -> Element<'a, Message> {
    let title = match mode {
        ItemDialogMode::Add => "Add Inventory Item",
        ItemDialogMode::Edit(_) => "Edit Inventory Item",
    };

    let mut form = column![
        text(title).size(28),
        text("").size(8),
    ];

    // Show validation error if present
    if let Some(error) = validation_error {
        form = form.push(
            container(
                text(error)
                    .size(14)
                    .style(|_theme: &iced::Theme| {
                        iced::widget::text::Style {
                            color: Some(iced::Color::from_rgb(0.9, 0.3, 0.3)),
                        }
                    })
            )
            .padding(10)
            .width(Length::Fill)
            .style(|_theme: &iced::Theme| container::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgba(0.9, 0.3, 0.3, 0.2))),
                border: iced::Border {
                    color: iced::Color::from_rgb(0.9, 0.3, 0.3),
                    width: 1.0,
                    radius: 5.0.into(),
                },
                ..Default::default()
            })
        );
        form = form.push(text("").size(8));
    }

    // Show similar items warning
    if !similar_items.is_empty() {
        let mut warning_content = column![
            text("⚠️ Similar items found:")
                .size(14)
                .style(|_theme: &iced::Theme| {
                    iced::widget::text::Style {
                        color: Some(iced::Color::from_rgb(0.9, 0.7, 0.3)),
                    }
                }),
        ]
        .spacing(5);
        
        for item in similar_items.iter().take(3) {
            warning_content = warning_content.push(
                text(format!("  • {}", item))
                    .size(12)
                    .style(|_theme: &iced::Theme| {
                        iced::widget::text::Style {
                            color: Some(iced::Color::from_rgb(0.8, 0.8, 0.8)),
                        }
                    })
            );
        }
        
        form = form.push(
            container(warning_content)
                .padding(10)
                .width(Length::Fill)
                .style(|_theme: &iced::Theme| container::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgba(0.9, 0.7, 0.3, 0.2))),
                    border: iced::Border {
                        color: iced::Color::from_rgb(0.9, 0.7, 0.3),
                        width: 1.0,
                        radius: 5.0.into(),
                    },
                    ..Default::default()
                })
        );
        form = form.push(text("").size(8));
    }

    form = form.push(column![
        text("Item Name:").size(14),
        text_input("Enter item name", name)
            .on_input(Message::NameChanged)
            .padding(10),
        text("SKU:").size(14),
        text_input("Enter SKU", sku)
            .on_input(Message::SkuChanged)
            .padding(10),
        text("Category:").size(14),
        text_input("Enter category (e.g., Electronics, Food)", category)
            .on_input(Message::CategoryChanged)
            .padding(10),
        text("Supplier:").size(14),
        text_input("Enter supplier name", supplier)
            .on_input(Message::SupplierChanged)
            .padding(10),
        text("Description:").size(14),
        text_input("Enter item description", description)
            .on_input(Message::DescriptionChanged)
            .padding(10),
        text("Quantity:").size(14),
        text_input("Enter quantity", quantity)
            .on_input(Message::QuantityChanged)
            .padding(10),
        text("Price ($):").size(14),
        text_input("Enter price", price)
            .on_input(Message::PriceChanged)
            .padding(10),
        text("").size(8),
        row![
            button("Submit")
                .on_press(Message::SubmitItem)
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
                    }
                ),
            button("Cancel")
                .on_press(Message::CloseItemDialog)
                .padding(10)
                .style(
                    |_theme: &iced::Theme, _status: iced::widget::button::Status| {
                        iced::widget::button::Style {
                            background: Some(iced::Background::Color(iced::Color::from_rgb(
                                0.3, 0.3, 0.3,
                            ))),
                            border: iced::Border {
                                color: iced::Color::from_rgb(0.4, 0.4, 0.4),
                                width: 1.0,
                                radius: 5.0.into(),
                            },
                            text_color: iced::Color::WHITE,
                            ..Default::default()
                        }
                    }
                ),
        ]
        .spacing(10),
    ])
    .spacing(8)
    .padding(30);

    container(
        container(
            container(scrollable(form).height(500).width(Length::Fill))
                .max_width(600)
                .max_height(600)
                .padding(20)
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
        .center_x(Length::Fill)
        .center_y(Length::Fill),
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
    })
    .into()
}
