use iced::{
    Alignment::Center,
    Border, Element,
    Length::{Fill, FillPortion},
    Theme,
    widget::{combo_box, container, row, text_input},
};
use iced_fonts::lucide;
use std::fmt::Display;

pub fn search<'a, Message, ComboBoxState>(
    value: &str,
    on_input: impl Fn(String) -> Message + 'a,
    state: &'a combo_box::State<ComboBoxState>,
    on_selected: impl Fn(ComboBoxState) -> Message + 'a + 'static,
    selected: Option<&ComboBoxState>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
    ComboBoxState: Display + Clone + 'a + 'static,
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
        text_input("Search", value)
            .on_input(on_input)
            .width(FillPortion(6)),
        combo_box(state, "Select", selected, on_selected).width(FillPortion(2))
    ])
    .width(Fill)
    .into()
}
