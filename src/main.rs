mod app;
mod views;

use app::App;

fn main() -> iced::Result {
    iced::run(App::update, App::view)
}
