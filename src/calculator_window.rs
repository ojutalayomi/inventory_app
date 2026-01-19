use iced::keyboard::{self, Key};
use iced::window;
use iced::{Element, Subscription, Task};

use crate::calculator::Calculator;
use crate::icon;
use crate::messages::{AppTheme, CalculatorOp, LoadError};
use crate::persistence;
use crate::views;
use crate::Message;

pub struct CalculatorWindow {
    calculator: Calculator,
    theme: AppTheme,
}

impl CalculatorWindow {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                calculator: Calculator::new(),
                theme: AppTheme::Dark,
            },
            Task::perform(persistence::load_state(), Message::Loaded),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Loaded(Ok(state)) => {
                self.theme = state.settings.theme;
                Task::none()
            }
            Message::Loaded(Err(LoadError::FileNotFound)) => Task::none(),
            Message::Loaded(Err(LoadError::FormatError)) => Task::none(),
            Message::CalculatorInput(digit) => {
                self.calculator.input_digit(digit);
                Task::none()
            }
            Message::CalculatorOperation(op) => {
                self.calculator.set_operation(op);
                Task::none()
            }
            Message::CalculatorEquals => {
                self.calculator.calculate_result();
                Task::none()
            }
            Message::CalculatorClear => {
                self.calculator.clear();
                Task::none()
            }
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        views::calculator::view(&self.calculator.display, &self.theme)
    }

    pub fn theme(&self) -> iced::Theme {
        match self.theme {
            AppTheme::Dark => iced::Theme::Dark,
            AppTheme::Light => iced::Theme::Light,
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        keyboard::on_key_press(|key, modifiers| {
            let allow_modifiers = modifiers.is_empty()
                || modifiers == keyboard::Modifiers::SHIFT;

            if !allow_modifiers {
                return None;
            }

            match key.as_ref() {
                Key::Character(character) => match character.as_ref() {
                    "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "." => {
                        Some(Message::CalculatorInput(character.to_string()))
                    }
                    "=" if modifiers == keyboard::Modifiers::SHIFT => {
                        Some(Message::CalculatorOperation(CalculatorOp::Add))
                    }
                    "8" if modifiers == keyboard::Modifiers::SHIFT => {
                        Some(Message::CalculatorOperation(CalculatorOp::Multiply))
                    }
                    "+" => Some(Message::CalculatorOperation(CalculatorOp::Add)),
                    "-" => Some(Message::CalculatorOperation(CalculatorOp::Subtract)),
                    "*" | "x" | "X" => Some(Message::CalculatorOperation(CalculatorOp::Multiply)),
                    "/" => Some(Message::CalculatorOperation(CalculatorOp::Divide)),
                    "=" => Some(Message::CalculatorEquals),
                    "c" | "C" => Some(Message::CalculatorClear),
                    _ => None,
                },
                Key::Named(keyboard::key::Named::Enter) => Some(Message::CalculatorEquals),
                Key::Named(keyboard::key::Named::Escape) => Some(Message::CalculatorClear),
                _ => None,
            }
        })
    }
}

pub fn run() -> iced::Result {
    iced::application(
        "Calculator",
        CalculatorWindow::update,
        CalculatorWindow::view,
    )
    .theme(CalculatorWindow::theme)
    .subscription(CalculatorWindow::subscription)
    .window(window::Settings {
        size: iced::Size::new(360.0, 520.0),
        min_size: Some(iced::Size::new(320.0, 480.0)),
        max_size: Some(iced::Size::new(400.0, 600.0)),
        icon: icon::load_icon(),
        ..Default::default()
    })
    .run_with(CalculatorWindow::new)
}
