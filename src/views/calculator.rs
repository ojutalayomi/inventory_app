use iced::widget::{button, column, container, mouse_area, row, text};
use iced::{Element, Length};

use crate::messages::{CalculatorOp, Message};

pub fn view<'a>(display: &'a str) -> Element<'a, Message> {
    let title_bar = mouse_area(
        container(
            text("Calculator - Drag to Move")
                .size(14)
                .width(Length::Fill)
                .align_x(iced::alignment::Horizontal::Center),
        )
        .width(Length::Fill)
        .padding(8)
        .style(|_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgb(
                0.2, 0.25, 0.3,
            ))),
            ..Default::default()
        }),
    )
    .on_press(Message::CalculatorDragStart);

    let display_container = container(text(display).size(32).width(Length::Fill))
        .padding(15)
        .width(Length::Fill)
        .style(|_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgb(
                0.1, 0.1, 0.1,
            ))),
            border: iced::Border {
                color: iced::Color::from_rgb(0.3, 0.3, 0.3),
                width: 1.0,
                radius: 5.0.into(),
            },
            ..Default::default()
        });

    let button_style =
        |_theme: &iced::Theme, _status: iced::widget::button::Status| iced::widget::button::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgb(
                0.25, 0.25, 0.25,
            ))),
            border: iced::Border {
                color: iced::Color::from_rgb(0.4, 0.4, 0.4),
                width: 1.0,
                radius: 5.0.into(),
            },
            text_color: iced::Color::WHITE,
            ..Default::default()
        };

    let op_button_style =
        |_theme: &iced::Theme, _status: iced::widget::button::Status| iced::widget::button::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgb(
                0.3, 0.4, 0.5,
            ))),
            border: iced::Border {
                color: iced::Color::from_rgb(0.4, 0.5, 0.6),
                width: 1.0,
                radius: 5.0.into(),
            },
            text_color: iced::Color::WHITE,
            ..Default::default()
        };

    let row1 = row![
        button(text("7").size(20))
            .on_press(Message::CalculatorInput("7".to_string()))
            .width(Length::Fill)
            .height(50)
            .style(button_style),
        button(text("8").size(20))
            .on_press(Message::CalculatorInput("8".to_string()))
            .width(Length::Fill)
            .height(50)
            .style(button_style),
        button(text("9").size(20))
            .on_press(Message::CalculatorInput("9".to_string()))
            .width(Length::Fill)
            .height(50)
            .style(button_style),
        button(text("÷").size(20))
            .on_press(Message::CalculatorOperation(CalculatorOp::Divide))
            .width(Length::Fill)
            .height(50)
            .style(op_button_style),
    ]
    .spacing(5);

    let row2 = row![
        button(text("4").size(20))
            .on_press(Message::CalculatorInput("4".to_string()))
            .width(Length::Fill)
            .height(50)
            .style(button_style),
        button(text("5").size(20))
            .on_press(Message::CalculatorInput("5".to_string()))
            .width(Length::Fill)
            .height(50)
            .style(button_style),
        button(text("6").size(20))
            .on_press(Message::CalculatorInput("6".to_string()))
            .width(Length::Fill)
            .height(50)
            .style(button_style),
        button(text("×").size(20))
            .on_press(Message::CalculatorOperation(CalculatorOp::Multiply))
            .width(Length::Fill)
            .height(50)
            .style(op_button_style),
    ]
    .spacing(5);

    let row3 = row![
        button(text("1").size(20))
            .on_press(Message::CalculatorInput("1".to_string()))
            .width(Length::Fill)
            .height(50)
            .style(button_style),
        button(text("2").size(20))
            .on_press(Message::CalculatorInput("2".to_string()))
            .width(Length::Fill)
            .height(50)
            .style(button_style),
        button(text("3").size(20))
            .on_press(Message::CalculatorInput("3".to_string()))
            .width(Length::Fill)
            .height(50)
            .style(button_style),
        button(text("-").size(20))
            .on_press(Message::CalculatorOperation(CalculatorOp::Subtract))
            .width(Length::Fill)
            .height(50)
            .style(op_button_style),
    ]
    .spacing(5);

    let row4 = row![
        button(text("0").size(20))
            .on_press(Message::CalculatorInput("0".to_string()))
            .width(Length::Fill)
            .height(50)
            .style(button_style),
        button(text(".").size(20))
            .on_press(Message::CalculatorInput(".".to_string()))
            .width(Length::Fill)
            .height(50)
            .style(button_style),
        button(text("=").size(20))
            .on_press(Message::CalculatorEquals)
            .width(Length::Fill)
            .height(50)
            .style(op_button_style),
        button(text("+").size(20))
            .on_press(Message::CalculatorOperation(CalculatorOp::Add))
            .width(Length::Fill)
            .height(50)
            .style(op_button_style),
    ]
    .spacing(5);

    let clear_button = button(text("Clear").size(18))
        .on_press(Message::CalculatorClear)
        .width(Length::Fill)
        .height(40)
        .style(
            |_theme: &iced::Theme, _status: iced::widget::button::Status| {
                iced::widget::button::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgb(
                        0.6, 0.2, 0.2,
                    ))),
                    border: iced::Border {
                        color: iced::Color::from_rgb(0.7, 0.3, 0.3),
                        width: 1.0,
                        radius: 5.0.into(),
                    },
                    text_color: iced::Color::WHITE,
                    ..Default::default()
                }
            },
        );

    column![
        title_bar,
        display_container,
        row1,
        row2,
        row3,
        row4,
        clear_button,
    ]
    .spacing(8)
    .into()
}
