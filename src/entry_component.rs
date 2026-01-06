use iced::Alignment::Center;
use iced::Length::{FillPortion, Shrink};
use iced::widget::space::horizontal;
use iced::widget::{container, row, text};
use iced::{Element, Theme};
use iced_fonts::lucide::{delete, pen};

use crate::custom_button_component::custom_button;
use crate::divider_component::divider;
use crate::utils::DataEntry;

pub fn entry<'a, Message>(
    entry: &DataEntry,
    on_delete: Message,
    on_edit: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    container(
        row![
            container(text(entry.key.clone())).width(FillPortion(4)),
            divider(1),
            container(text(entry.description.clone())).width(FillPortion(8)),
            divider(1),
            container(
                row![
                    custom_button(pen(), on_edit, |theme: &Theme| theme
                        .extended_palette()
                        .secondary
                        .base
                        .color),
                    horizontal(),
                    custom_button(delete(), on_delete, |theme: &Theme| theme.palette().danger),
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
