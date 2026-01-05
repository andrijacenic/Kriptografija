use iced::{Element, widget::row};

pub fn search<'a, Message>() -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    // let a = row![];

    row![].into()
}
