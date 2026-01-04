use iced::Alignment::Center;
use iced::Length::{FillPortion, Shrink};
use iced::widget::space::horizontal;
use iced::widget::{container, row, text};
use iced::{Element, Theme};
use iced_fonts::lucide::{delete, pen};

use crate::button_custom::button_custom;
use crate::divider_component::divider_component;
use crate::utils::DataEntry;

pub fn entry_component<'a, Message>(
    entry: &DataEntry,
    on_delete: Message,
    on_edit: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    container(
        row![
            container(text(entry.key.clone())).width(FillPortion(1)),
            divider_component(30),
            container(text(entry.description.clone())).width(FillPortion(8)),
            divider_component(30),
            container(
                row![
                    button_custom(pen(), on_edit, |theme: &Theme| theme.palette().primary),
                    horizontal(),
                    button_custom(delete(), on_delete, |theme: &Theme| theme.palette().danger),
                ]
                .spacing(10)
            )
            .width(Shrink)
        ]
        .spacing(10)
        .align_y(Center),
    )
    .style(|theme: &Theme| container::Style {
        background: Some(iced::Background::Color(theme.palette().background)),
        border: iced::Border {
            color: theme.extended_palette().background.strongest.color,
            width: 1.0,
            ..Default::default()
        },
        ..Default::default()
    })
    .padding(5)
    .into()
}
