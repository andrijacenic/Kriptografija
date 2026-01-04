use iced::{Border, Color, Element, Renderer, Theme, widget::button};

pub fn button_custom<'a, Message>(
    label: impl Into<Element<'a, Message, Theme, Renderer>>,
    on_press: Message,
    color_selector: impl Fn(&Theme) -> Color + Copy + 'a,
) -> iced::widget::Button<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
{
    button(label)
        .on_press(on_press)
        .style(move |theme: &Theme, status| {
            let base_color = color_selector(theme);

            let final_color = match status {
                iced::widget::button::Status::Hovered => Color {
                    a: 0.8,
                    ..base_color
                },
                _ => base_color,
            };

            button::Style {
                background: Some(final_color.into()),
                text_color: Color::WHITE,
                border: Border {
                    radius: 2.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        })
}
