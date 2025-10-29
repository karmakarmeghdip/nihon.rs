mod app;
mod components;
mod constants;
mod error;
mod models;
mod services;
mod ui;
mod views;

use app::App;

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title(App::title)
        .theme(App::theme)
        .run()
}
