use iced::{
    Alignment::Center,
    Border, Element,
    Length::Fill,
    Theme,
    widget::{container, row, text_input},
};
use iced_fonts::lucide;

pub fn search<'a, Message>(
    value: &str,
    on_input: impl Fn(String) -> Message + 'a,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    container(row![
        container(lucide::search())
            .align_x(Center)
            .align_y(Center)
            .padding(5)
            .style(|theme: &Theme| container::Style {
                border: Border {
                    width: 1.0,
                    color: theme.extended_palette().background.strongest.color,
                    radius: 0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }),
        text_input("Search", value).on_input(on_input)
    ])
    .width(Fill)
    .into()
}
