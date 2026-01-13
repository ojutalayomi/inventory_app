use iced::widget::{column, container, text};
use iced::{Element, Length};

use crate::messages::{AppTheme, Message};
use crate::theme;
use crate::icons;

pub fn view<'a>(app_theme: &'a AppTheme) -> Element<'a, Message> {
    container(
        column![
            icons::Icon::Box.view(icons::IconSize::XLarge, app_theme),
            text("Inventory Manager")
                .size(theme::TEXT_DISPLAY)
                .style(move |_theme: &iced::Theme| text::Style {
                    color: Some(theme::primary_color(app_theme)),
                }),
            text("Loading your workspace...")
                .size(theme::TEXT_H3)
                .style(move |_theme: &iced::Theme| text::Style {
                    color: Some(theme::text_secondary_color(app_theme)),
                }),
        ]
        .spacing(theme::SPACING_XL)
        .align_x(iced::Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .style(move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(theme::bg_color(app_theme))),
        ..Default::default()
    })
    .into()
}
