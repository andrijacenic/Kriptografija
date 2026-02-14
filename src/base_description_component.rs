use iced::widget::{rich_text, span};
use iced::{Element, Theme};
use regex::Regex;

use crate::utils::shift_hue;

#[derive(Clone, Debug)]
pub struct Link {
    pub text: String,
    pub link: String,
}

#[derive(Clone, Debug)]
pub struct DescriptionImage {
    pub text: String,
    pub image: String,
}

#[derive(Clone, Debug)]
pub struct DescriptionSound {
    pub text: String,
    pub sound: String,
}

#[derive(Clone, Debug)]
pub enum DescriptionElement {
    Text(String),
    Link(Link),
    Image(DescriptionImage),
    Sound(DescriptionSound),
}

pub fn parse_description_elements(description: String) -> Vec<DescriptionElement> {
    let re = Regex::new(r#"<(link|image|sound)="([^"]+)" text="([^"]+)">"#).unwrap();

    let mut elements = Vec::new();
    let mut last_match = 0;

    for cap in re.captures_iter(description.as_str()) {
        let m = cap.get(0).unwrap();

        if m.start() > last_match {
            elements.push(DescriptionElement::Text(
                description[last_match..m.start()].to_string(),
            ));
        }

        // 2. Parse the specific tag type
        let tag_type = &cap[1];
        let val = cap[2].to_string();
        let text = cap[3].to_string();

        match tag_type {
            "link" => elements.push(DescriptionElement::Link(Link {
                text: text,
                link: val,
            })),
            "image" => elements.push(DescriptionElement::Image(DescriptionImage {
                image: val,
                text,
            })),
            "sound" => elements.push(DescriptionElement::Sound(DescriptionSound {
                sound: val,
                text,
            })),
            _ => elements.push(DescriptionElement::Text(m.as_str().to_string())),
        }

        last_match = m.end();
    }

    // 3. Push any remaining text after the last tag
    if last_match < description.len() {
        elements.push(DescriptionElement::Text(
            description[last_match..].to_string(),
        ));
    }

    elements
}

pub fn serialize_description_elements(elements: Vec<DescriptionElement>) -> String {
    elements
        .into_iter()
        .map(|element| match element {
            DescriptionElement::Text(text) => text,
            DescriptionElement::Link(ld) => format!("<link=\"{}\" text=\"{}\">", ld.link, ld.text),
            DescriptionElement::Image(image) => {
                format!("<image=\"{}\" text=\"{}\">", image.image, image.text)
            }
            DescriptionElement::Sound(sound) => {
                format!("<sound=\"{}\" text=\"{}\">", sound.sound, sound.text)
            }
        })
        .collect::<Vec<String>>()
        .join("")
}

pub fn description_component<'a, Message>(
    description_elements: Vec<DescriptionElement>,
    on_click: impl Fn(DescriptionElement) -> Message + 'a,
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
        .on_link_click(on_click)
        .into()
}
