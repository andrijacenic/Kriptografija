use iced::Length;
use iced::widget::{container, row};
use iced::{Element, Theme};

pub fn divider<'a, Message>(lenght: impl Into<Length>) -> Element<'a, Message>
where
    Message: 'a,
{
    container(row![])
        .style(|theme: &Theme| container::Style {
            background: Some(theme.extended_palette().background.strongest.color.into()),
            ..Default::default()
        })
        .height(lenght)
        .width(1)
        .into()
}
