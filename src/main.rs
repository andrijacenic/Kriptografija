mod app;
mod theme;
mod utils;
mod window_component;
mod window_manager;

use app::App;

use crate::theme::APP_TITLE;

pub fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .theme(App::theme)
        .resizable(true)
        .centered()
        .window_size((1200, 800))
        .title(APP_TITLE)
        .run()
}
