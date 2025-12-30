use crate::message::Message;
use crate::theme;
use iced::Length::Fill;
use iced::widget::{button, center, column, container, text};
use iced::widget::{opaque, stack};
use iced::{Color, Element, Length, Renderer, Task, Theme};
pub struct App {
    // Your application state goes here
    pub welcome_message: String,
    window_opened: bool,
    window_answer: Option<bool>,
}

impl App {
    // Initialize the application state
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                welcome_message: String::from("Welcome to your Iced App!"),
                window_opened: false,
                window_answer: None,
            },
            Task::none(),
        )
    }

    // Logic for handling messages
    pub fn update(&mut self, _message: Message) -> Task<Message> {
        match _message {
            Message::Kikiriki => {
                self.welcome_message = "Kikiriki".to_owned();
                Task::none()
            }
            Message::Pralina => {
                self.welcome_message = "Pralina".to_owned();
                Task::none()
            }
            Message::Prozor(opened) => {
                self.window_opened = opened;
                Task::none()
            }
        }
    }

    // The UI layout
    pub fn view(&self) -> Element<'_, Message> {
        let kikiriki = button("Kikiriki").on_press(Message::Kikiriki);
        let pralina = button("Pralina").on_press(Message::Pralina);
        let prozor = button("Prozor").on_press(Message::Prozor(true));

        let main_content = center(
            column![
                text(&self.welcome_message).size(40),
                kikiriki,
                pralina,
                prozor,
            ]
            .spacing(20),
        );

        let mut layers: Vec<Element<Message, Theme, Renderer>> = vec![main_content.into()];

        if (self.window_opened) {
            let overlay = container(center(
                // The actual popup box
                container(
                    column![
                        text("Are you sure?").size(24),
                        text("This action cannot be undone."),
                        button("Yes, do it -> KIKIRIKI").on_press(Message::Kikiriki),
                        button("Cancel -> PRALINA").on_press(Message::Pralina),
                        button("EXIT -> PRALINA").on_press(Message::Prozor(false)),
                    ]
                    .spacing(15)
                    .padding(20)
                    .align_x(iced::Alignment::Center),
                )
                .width(300),
            ))
            .width(Fill)
            .height(Fill)
            .style(|_| container::Style {
                // Dim the background (semi-transparent black)
                background: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.7).into()),
                ..Default::default()
            });

            layers.push(opaque(overlay).into());
        }

        stack(layers).into()
    }

    pub fn theme(&self) -> Theme {
        theme::default_theme()
    }
}
