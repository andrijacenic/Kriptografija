mod app;
mod base_description_component;
mod custom_button_component;
mod divider_component;
mod entity_edit_component;
mod entry_component;
mod menu_button_component;
mod search_component;
mod theme;
mod utils;
mod window_component;
mod window_manager;

use app::App;
use iced::window::Settings;

use crate::{theme::APP_TITLE, utils::load_icon};
use iced_aw::ICED_AW_FONT_BYTES;

pub fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .theme(App::theme)
        .font(ICED_AW_FONT_BYTES)
        .window(Settings {
            min_size: Some((400, 200).into()),
            size: (700, 800).into(),
            icon: match load_icon() {
                Ok(icon) => Some(icon),
                Err(error) => {
                    println!("{:?}", error);
                    None
                }
            },
            resizable: true,
            ..Default::default()
        })
        .centered()
        .title(APP_TITLE)
        .run()
}
