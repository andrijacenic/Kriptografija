use std::ptr::null;

use crate::message::Message;
use crate::theme;
use iced::Alignment::Center;
use iced::widget::{button, center, column, container, text};
use iced::{Element, Length, Task, Theme, window};

pub struct App {
    // Your application state goes here
    pub welcome_message: String,
    sub_window: Option<window::Id>,
}

impl App {
    // Initialize the application state
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                welcome_message: String::from("Welcome to your Iced App!"),
                sub_window: None,
            },
            Task::none(),
        )
    }

    // Logic for handling messages
    pub fn update(&mut self, _message: Message) -> Task<Message> {
        match _message {
            // Handle variants from message.rs
            Message::Kikiriki => {
                self.welcome_message = "Kikiriki".to_owned();
                Task::none()
            }
            Message::Pralina => {
                self.welcome_message = "Pralina".to_owned();
                Task::none()
            }
            Message::Prozor => {
                let (id, task) = iced::window::open(iced::window::Settings {
                    size: (400.0, 200.0).into(),
                    ..Default::default()
                });

                self.sub_window = Some(id);

                task.map(|_| Message::Kikiriki)
            }
        }
    }

    // The UI layout
    pub fn view(&self) -> Element<'_, Message> {
        let kikiriki = button("Kikiriki").on_press(Message::Kikiriki);
        let pralina = button("Pralina").on_press(Message::Pralina);
        let prozor = button("Prozor").on_press(Message::Prozor);

        let layout = column![
            text(&self.welcome_message).size(40),
            kikiriki,
            pralina,
            prozor,
        ]
        .spacing(20)
        .align_x(iced::Alignment::Center);

        center(layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn theme(&self) -> Theme {
        theme::default_theme()
    }
}
