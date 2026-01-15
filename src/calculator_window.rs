use iced::window;
use iced::{Element, Task};

use crate::calculator::Calculator;
use crate::icon;
use crate::messages::{AppTheme, LoadError};
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
}

pub fn run() -> iced::Result {
    iced::application(
        "Calculator",
        CalculatorWindow::update,
        CalculatorWindow::view,
    )
    .theme(CalculatorWindow::theme)
    .window(window::Settings {
        size: iced::Size::new(360.0, 520.0),
        min_size: Some(iced::Size::new(320.0, 480.0)),
        max_size: Some(iced::Size::new(400.0, 600.0)),
        icon: icon::load_icon(),
        ..Default::default()
    })
    .run_with(CalculatorWindow::new)
}
