use iced::Length;
use iced::widget::container;
use iced::widget::space::horizontal;
use iced::{Element, Theme};

pub fn divider<'a, Message>(lenght: impl Into<Length>) -> Element<'a, Message>
where
    Message: 'a,
{
    container(horizontal())
        .width(lenght)
        .height(Length::Fill)
        .style(|theme: &Theme| container::Style {
            background: Some(theme.extended_palette().background.strongest.color.into()),
            ..Default::default()
        })
        .into()
}
