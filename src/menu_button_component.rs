use iced::widget::{Button, button};
use iced::{Border, Color, Element, Theme};

pub fn menu_button<'a, Message>(content: impl Into<Element<'a, Message>>) -> Button<'a, Message> {
    button(content)
        .padding([4, 8])
        .style(|theme: &Theme, status: button::Status| {
            let palette = theme.extended_palette();

            let mut style = button::Style {
                text_color: palette.background.base.text,
                border: Border {
                    radius: 0.into(),
                    ..Border::default()
                },
                ..button::Style::default()
            };

            match status {
                button::Status::Active => {
                    style.background = Some(Color::TRANSPARENT.into());
                }
                button::Status::Hovered => {
                    style.background = Some(palette.primary.weak.color.scale_alpha(0.5).into());
                }
                button::Status::Pressed => {
                    style.background = Some(palette.primary.strong.color.into());
                }
                button::Status::Disabled => {
                    style.text_color = Color::from_rgb(0.5, 0.5, 0.5);
                }
            }
            style
        })
}
