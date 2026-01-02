mod app;
mod theme;
mod utils;
mod window_component;
mod window_manager;

use app::App;

use crate::theme::APP_TITLE;
use iced_aw::ICED_AW_FONT_BYTES;

pub fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .theme(App::theme)
        .font(ICED_AW_FONT_BYTES)
        .resizable(true)
        .centered()
        .window_size((1200, 800))
        .title(APP_TITLE)
        .run()
}
