use iced::widget::button::{Status, Style};
use iced::widget::{button, container, text};
use iced::{Element, Theme};

pub struct DescriptionElement {
    pub text: String,
}

pub struct LinkElement {
    pub base: DescriptionElement,
    pub link: String,
}

impl LinkElement {
    pub fn new() -> Self {
        LinkElement {
            base: DescriptionElement {
                text: "aaaa".to_string(),
            },
            link: "linkkkk".to_string(),
        }
    }
}

pub fn base_description<'a, Message>(description: DescriptionElement) -> Element<'a, Message>
where
    Message: 'a,
{
    text(description.text).into()
}

pub fn link_description<'a, Message>(
    description: LinkElement,
    on_press: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    container(
        button(text(description.base.text))
            .style(|theme: &Theme, status: Status| Style {
                text_color: match status {
                    Status::Active => theme.extended_palette().primary.strong.color,
                    Status::Pressed => theme.extended_palette().primary.weak.color,
                    Status::Hovered => theme.extended_palette().primary.base.color,
                    Status::Disabled => theme.extended_palette().background.strong.color,
                },
                background: None,
                ..Default::default()
            })
            .on_press(on_press),
    )
    .into()
}
