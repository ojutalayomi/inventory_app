use iced::widget::{button, column, row, text};
use iced::{Color, Element, Length};

use crate::messages::{AppTheme, CalculatorOp, Message};
use crate::theme;
use crate::icons;

pub fn view<'a>(display: &'a str, app_theme: &'a AppTheme) -> Element<'a, Message> {
    // Modern display with gradient text
    let display_container = text(if display.is_empty() { "0" } else { display })
        .size(theme::TEXT_DISPLAY)
        .align_x(iced::alignment::Horizontal::Right)
        .width(Length::Fill)
        .style(move |_theme: &iced::Theme| text::Style {
            color: Some(theme::primary_color(app_theme)),
        });

    // Helper function for number buttons
    let make_num_button = |num: &'a str| {
        button(text(num).size(theme::TEXT_H3))
            .on_press(Message::CalculatorInput(num.to_string()))
            .width(Length::Fill)
            .height(60)
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
                    shadow: iced::Shadow {
                        color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                        offset: iced::Vector::new(0.0, 2.0),
                        blur_radius: 4.0,
                    },
                    ..Default::default()
                }
            })
    };

    // Helper function for operator buttons
    let make_op_button = |symbol: &'a str, op: CalculatorOp| {
        button(text(symbol).size(theme::TEXT_H3))
            .on_press(Message::CalculatorOperation(op))
            .width(Length::Fill)
            .height(60)
            .style(move |_theme: &iced::Theme, status: button::Status| {
                let bg_color = match status {
                    button::Status::Hovered => theme::primary_dark_color(app_theme),
                    _ => theme::primary_color(app_theme),
                };
                
                button::Style {
                    background: Some(iced::Background::Color(bg_color)),
                    text_color: Color::WHITE,
                    border: iced::Border {
                        color: theme::primary_color(app_theme),
                        width: 1.0,
                        radius: theme::RADIUS_MD.into(),
                    },
                    shadow: iced::Shadow {
                        color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                        offset: iced::Vector::new(0.0, 2.0),
                        blur_radius: 6.0,
                    },
                    ..Default::default()
                }
            })
    };

    let row1 = row![
        make_num_button("7"),
        make_num_button("8"),
        make_num_button("9"),
        make_op_button("÷", CalculatorOp::Divide),
    ]
    .spacing(theme::SPACING_SM);

    let row2 = row![
        make_num_button("4"),
        make_num_button("5"),
        make_num_button("6"),
        make_op_button("×", CalculatorOp::Multiply),
    ]
    .spacing(theme::SPACING_SM);

    let row3 = row![
        make_num_button("1"),
        make_num_button("2"),
        make_num_button("3"),
        make_op_button("-", CalculatorOp::Subtract),
    ]
    .spacing(theme::SPACING_SM);

    let row4 = row![
        make_num_button("0"),
        make_num_button("."),
        button(text("=").size(theme::TEXT_H3))
            .on_press(Message::CalculatorEquals)
            .width(Length::Fill)
            .height(60)
            .style(move |_theme: &iced::Theme, status: button::Status| {
                let bg_color = match status {
                    button::Status::Hovered => theme::accent_color(app_theme),
                    _ => theme::success_color(app_theme),
                };
                
                button::Style {
                    background: Some(iced::Background::Color(bg_color)),
                    text_color: Color::WHITE,
                    border: iced::Border {
                        color: theme::success_color(app_theme),
                        width: 1.0,
                        radius: theme::RADIUS_MD.into(),
                    },
                    shadow: iced::Shadow {
                        color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                        offset: iced::Vector::new(0.0, 2.0),
                        blur_radius: 6.0,
                    },
                    ..Default::default()
                }
            }),
        make_op_button("+", CalculatorOp::Add),
    ]
    .spacing(theme::SPACING_SM);

    let clear_button = button(
        row![
            icons::Icon::Delete.view(icons::IconSize::Small, app_theme),
            text("Clear").size(theme::TEXT_BODY_LARGE),
        ]
        .spacing(theme::SPACING_SM)
        .align_y(iced::Alignment::Center)
    )
        .on_press(Message::CalculatorClear)
        .width(Length::Fill)
        .height(50)
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
                    color: theme::danger_color(app_theme),
                    width: 1.0,
                    radius: theme::RADIUS_MD.into(),
                },
                ..Default::default()
            }
        });

    column![
        row![
            icons::Icon::Calculator.view(icons::IconSize::Medium, app_theme),
            text("Calculator")
                .size(theme::TEXT_H3)
                .style(move |_theme: &iced::Theme| text::Style {
                    color: Some(theme::text_color(app_theme)),
                }),
        ]
        .spacing(theme::SPACING_SM)
        .align_y(iced::Alignment::Center),
        text("").size(theme::SPACING_SM),
        display_container,
        text("").size(theme::SPACING_LG),
        row1,
        row2,
        row3,
        row4,
        text("").size(theme::SPACING_SM),
        clear_button,
    ]
    .padding(theme::SPACING_MD)
    .spacing(theme::SPACING_MD)
    .into()
}
