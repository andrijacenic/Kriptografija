use iced::{
    Border, Color, Element,
    Length::Fill,
    Renderer, Theme,
    widget::{Row, Text, button, center, container, row, space::horizontal, text},
};
use iced_aw::{card, style};
use iced_fonts::lucide::{info, octagon_alert, triangle_alert};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowType {
    Info,
    Warning,
    Error,
    AddEntry,
}

#[derive(Debug, Clone)]
pub struct WindowContent {
    pub id: Uuid,
    pub window_type: WindowType,
    pub title: String,
    pub content: String,
    pub window_width: Option<u32>,
    pub show_cancel: bool,
    pub show_okay: bool,
}

impl WindowContent {
    pub fn new(
        window_type: WindowType,
        title: String,
        content: String,
        window_width: Option<u32>,
        show_cancel: bool,
        show_okay: bool,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            window_type,
            title,
            content,
            window_width,
            show_cancel,
            show_okay,
        }
    }
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

    let mut footer: Row<'_, Message, Theme, Renderer> = row![horizontal()].spacing(10).width(Fill);

    if window_content.show_okay {
        footer = footer.push(create_button("Okay", on_okay, |theme| {
            theme.palette().primary
        }));
    }

    if window_content.show_cancel {
        footer = footer.push(create_button("Cancel", on_cancel, |theme| {
            theme.palette().danger
        }));
    }

    container(center(
        card(
            row![icon, text(window_content.title)].spacing(10),
            body_content,
        )
        .foot(footer)
        .on_close(on_close)
        .style(move |theme, status| card::Style {
            border_radius: 2.0,
            border_color: theme.palette().background,
            ..card_style(theme, status)
        })
        .width(window_width),
    ))
    .width(Fill)
    .height(Fill)
    .style(|_theme| container::Style {
        background: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.6).into()),
        ..Default::default()
    })
    .into()
}

fn create_button<'a, Message>(
    label: &'a str,
    on_press: Message,
    // Change this to a closure that picks a color from the theme
    color_selector: impl Fn(&Theme) -> Color + Copy + 'a,
) -> iced::widget::Button<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
{
    button(label)
        .on_press(on_press)
        .style(move |theme: &Theme, status| {
            let base_color = color_selector(theme);

            let final_color = match status {
                iced::widget::button::Status::Hovered => Color {
                    a: 0.8,
                    ..base_color
                },
                _ => base_color,
            };

            button::Style {
                background: Some(final_color.into()),
                text_color: Color::WHITE,
                border: Border {
                    radius: 2.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        })
}
