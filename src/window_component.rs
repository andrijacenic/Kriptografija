use std::default;

use iced::{
    Color, Element,
    Length::Fill,
    Renderer, Theme,
    widget::{Text, button, center, container, row, text},
};
use iced_aw::{card, style};
use iced_fonts::lucide::{info, octagon_alert, triangle_alert};

#[derive(Debug, Clone, Copy)]
pub enum WindowType {
    Info,
    Warning,
    Error,
    AddEntry,
}

#[derive(Debug, Clone)]
pub struct WindowContent {
    pub window_type: WindowType,
    pub title: String,
    pub content: String,
    pub window_width: Option<u32>,
}

pub fn custom_window<'a, Message>(
    window_content: WindowContent,
    on_close: Message,
    on_okay: Message,
    on_cancel: Message,
    body: Option<impl Into<Element<'a, Message, Theme, Renderer>>>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let card_style = match window_content.window_type {
        WindowType::Warning => style::card::warning,
        WindowType::Error => style::card::danger,
        _default => style::card::primary,
    };

    let icon: Text<'_, Theme, Renderer> = match window_content.window_type {
        WindowType::Warning => triangle_alert(),
        WindowType::Error => octagon_alert(),
        _default => info(),
    };

    let window_width = if let Some(width) = window_content.window_width {
        width
    } else {
        400
    };

    let body_content = if let Some(body_elem) = body {
        body_elem.into()
    } else {
        text(window_content.content).size(20).into()
    };

    container(center(
        card(
            row![icon, text(window_content.title)].spacing(10),
            body_content,
        )
        .foot(
            row![
                button("Okay")
                    .on_press(on_okay)
                    .style(|theme: &Theme, _| button::Style {
                        background: Some(theme.palette().primary.into()),
                        text_color: Color::WHITE,
                        ..Default::default()
                    }),
                button("Cancel")
                    .on_press(on_cancel)
                    .style(|theme: &Theme, _| button::Style {
                        background: Some(theme.palette().danger.into()),
                        text_color: Color::WHITE,
                        ..Default::default()
                    }),
            ]
            .spacing(30),
        )
        .on_close(on_close)
        .style(card_style)
        .width(window_width),
    ))
    .width(Fill)
    .height(Fill)
    .style(|_theme| container::Style {
        background: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.2).into()),
        ..Default::default()
    })
    .into()
}
