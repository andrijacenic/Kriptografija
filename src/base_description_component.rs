use iced::widget::{rich_text, span};
use iced::{Element, Theme};

use crate::utils::shift_hue;

#[derive(Clone)]
pub struct Link {
    pub text: String,
    pub link: String,
}

#[derive(Clone)]
pub struct DescriptionImage {
    pub text: String,
    pub image: String,
}

#[derive(Clone)]
pub struct DescriptionSound {
    pub text: String,
    pub sound: String,
}

#[derive(Clone)]
pub enum DescriptionElement {
    Text(String),
    Link(Link),
    Image(DescriptionImage),
    Sound(DescriptionSound),
}

#[derive(Clone)]
pub enum DescriptionLinkAction {
    Link(Link),
    Image(DescriptionImage),
    Sound(),
}

pub fn description_component<'a, Message>(
    description_elements: Vec<DescriptionElement>,
    on_link: impl Fn(DescriptionElement) -> Message + 'a,
    theme: &Theme,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let spans: Vec<_> = description_elements
        .into_iter()
        .map(|value| {
            let span_element = match value.clone() {
                DescriptionElement::Text(content) => span(content),
                DescriptionElement::Link(ld) => span(ld.text.clone())
                    .color(theme.extended_palette().primary.strong.color)
                    .underline(true),
                DescriptionElement::Image(image) => span(image.text.clone())
                    .color(shift_hue(
                        theme.extended_palette().primary.strong.color,
                        100.0,
                    ))
                    .underline(true),
                DescriptionElement::Sound(sound) => span(sound.text.clone())
                    .color(shift_hue(
                        theme.extended_palette().primary.strong.color,
                        200.0,
                    ))
                    .underline(true),
            };
            span_element.link(value)
        })
        .collect();

    rich_text(spans)
        .wrapping(iced::widget::text::Wrapping::WordOrGlyph)
        .on_link_click(on_link)
        .into()
}
