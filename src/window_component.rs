use iced::{
    Color, Element,
    Length::Fill,
    widget::{button, center, column, container, row, text},
};
use iced_aw::card;

use crate::window_manager::WindowContentBase;

pub fn window_component<'a, Message>(
    window_content: WindowContentBase,
    on_close: Message,
    on_okay: Message,
    on_cancel: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    // TODO: Add all of the different thing for each of the window types
    // match window_content.window_type {
    //     crate::window_manager::WindowType::Info => todo!(),
    //     crate::window_manager::WindowType::Warning => todo!(),
    //     crate::window_manager::WindowType::Error => todo!(),
    //     crate::window_manager::WindowType::AddElement => todo!(),
    // }

    container(center(
        card(
            text(window_content.title),
            column![
                text(window_content.content).size(24),
                row![
                    button("Okay").on_press(on_okay),
                    button("Cancel").on_press(on_cancel),
                ]
                .spacing(15)
                .padding(20)
                .width(Fill)
            ]
            .spacing(15)
            .padding(20),
        )
        .on_close(on_close)
        .width(300),
    ))
    .width(Fill)
    .height(Fill)
    .style(|_theme| container::Style {
        background: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.2).into()),
        ..Default::default()
    })
    .into()
}
