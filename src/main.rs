use iced::window;

mod alerts;
mod app_state;
mod audit;
mod auth;
mod calculator;
mod calculator_window;
mod errors;
mod handlers;
mod icon;
mod icons;
mod inventory;
mod messages;
mod note;
mod persistence;
mod search;
mod theme;
mod update_checker;
mod user;
mod views;

use app_state::{AppState, InventoryApp};
pub use messages::Message;

fn main() -> iced::Result {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|arg| arg == "--calculator") {
        return calculator_window::run();
    }

    iced::application(
        "Inventory Manager",
        InventoryApp::update,
        InventoryApp::view,
    )
    .theme(InventoryApp::theme)
    .subscription(InventoryApp::subscription)
    .window(window::Settings {
        size: iced::Size::new(1200.0, 800.0),
        min_size: Some(iced::Size::new(900.0, 600.0)),
        icon: icon::load_icon(),
        ..Default::default()
    })
    .run_with(InventoryApp::new)
}
