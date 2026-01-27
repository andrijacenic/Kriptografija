use std::rc::Rc;

use iced::Alignment::Center;
use iced::Length;
use iced::widget::{column, container, row, text, text_input};
use iced::{Border, Element, Theme};

pub enum InputChange {
    Key(String),
    Description(String),
}

pub fn entity_edit<'a, Message>(
    label_width: Length,
    key_input: &str,
    is_key_invalid: bool,
    description_input: &str,
    is_description_invalid: bool,
    on_input: impl Fn(InputChange) -> Message + 'a,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let on_input = Rc::new(on_input);
    let on_input_key = on_input.clone();
    let on_input_description = on_input.clone();

    column![
        text("Add an entry below").size(16),
        row![
            container(text("Key").size(16).width(label_width).align_y(Center)).padding(5),
            text_input("Key", key_input)
                .style(move |theme: &Theme, status| {
                    let mut style = text_input::default(theme, status);
                    if is_key_invalid {
                        style.border = Border {
                            color: theme.palette().danger,
                            width: 1.0,
                            ..Default::default()
                        };
                    }
                    style
                })
                .on_input(move |value| on_input_key(InputChange::Key(value)))
        ]
        .spacing(10),
        row![
            container(
                text("Description")
                    .size(16)
                    .width(label_width)
                    .align_y(Center)
            )
            .padding(5),
            text_input("Description", description_input)
                .style(move |theme: &Theme, status| {
                    let mut style = text_input::default(theme, status);
                    if is_description_invalid {
                        style.border = Border {
                            color: theme.palette().danger,
                            width: 1.0,
                            ..Default::default()
                        };
                    }
                    style
                })
                .on_input(move |value| on_input_description(InputChange::Description(value)))
        ]
        .spacing(10)
    ]
    .spacing(15)
    .into()
}
