use crate::message::Message;
use crate::theme;
use crate::utils::AppData;
use iced::Length::Fill;
use iced::widget::{button, center, column, container, text};
use iced::widget::{opaque, stack};
use iced::{Color, Element, Length, Renderer, Task, Theme};
use iced_aw::card;

pub struct App {
    app_data: AppData,
}

impl App {
    // Initialize the application state
    pub fn new() -> (Self, Task<Message>) {
        let app_data_result = AppData::load_file("data.txt".to_string());
        if app_data_result.is_err() {
            let app_data = AppData::new().unwrap();
            return (Self { app_data: app_data }, Task::none());
        }
        (
            Self {
                app_data: app_data_result.unwrap(),
            },
            Task::none(),
        )
    }

    // Logic for handling messages
    pub fn update(&mut self, _message: Message) -> Task<Message> {
        // match _message {
        //     Message::Kikiriki => {
        //         self.welcome_message = "Kikiriki".to_owned();
        //         Task::none()
        //     }
        //     Message::Pralina => {
        //         self.welcome_message = "Pralina".to_owned();
        //         Task::none()
        //     }
        //     Message::Prozor(opened) => {
        //         self.window_opened = opened;
        //         Task::none()
        //     }
        // }
        Task::none()
    }

    // The UI layout
    pub fn view(&self) -> Element<'_, Message> {
        let content = column![text("Hello, Iced!").size(50)]
            .spacing(20)
            .padding(40);

        container(content).width(Fill).height(Fill).into()
    }

    pub fn theme(&self) -> Theme {
        theme::default_theme()
    }
}
