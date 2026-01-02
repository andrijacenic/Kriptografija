use std::fs::File;

use crate::utils::AppData;
use crate::window_component::{WindowContent, WindowType};
use crate::{theme, window_manager};
use iced::Length::Fill;
use iced::widget::{button, center, column, container, text};
use iced::widget::{opaque, stack};
use iced::{Element, Renderer, Task, Theme};

#[derive(Debug, Clone)]
pub enum AppMessage {
    OpenWindow(WindowContent),
    CloseWindow(Option<bool>),
    None,
}

pub struct App {
    app_data: AppData,
    theme: Theme,
}

impl App {
    pub fn new() -> (Self, Task<AppMessage>) {
        let (app_data, initial_error) = match AppData::load_file("data.txt".to_string()) {
            Ok(data) => (data, None),
            Err(e) => (AppData::new().unwrap(), Some(e.to_string())),
        };

        if let Some(error_msg) = initial_error {
            window_manager::WindowManager::global()
                .lock()
                .unwrap()
                .add_window(WindowContent {
                    window_type: WindowType::Error,
                    title: "Loading data error!".to_string(),
                    content: format!("Error loading data: {}", error_msg),
                    window_width: None,
                });
        }

        (
            Self {
                app_data,
                theme: theme::default_theme(),
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, _message: AppMessage) -> Task<AppMessage> {
        match _message {
            AppMessage::OpenWindow(content) => {
                window_manager::WindowManager::global()
                    .lock()
                    .unwrap()
                    .add_window(content);
                Task::none()
            }
            AppMessage::CloseWindow(value) => {
                print!("Closing window with value: {:?}\n", value);
                window_manager::WindowManager::global()
                    .lock()
                    .unwrap()
                    .remove_window();
                Task::none()
            }
            AppMessage::None => Task::none(),
        }
    }

    // The UI layout
    pub fn view(&self) -> Element<'_, AppMessage> {
        let main_content = center(
            column![
                text("Hello, ASDASDASDASDASDASDASDASDASDASDASDAS!").size(50),
                button("Click Me").on_press(AppMessage::OpenWindow(WindowContent {
                    window_type: WindowType::Info,
                    title: "Info".to_string(),
                    content: "This is an informational window.".to_string(),
                    window_width: None,
                })),
            ]
            .spacing(20),
        );

        let mut layers: Vec<Element<AppMessage, Theme, Renderer>> =
            vec![container(main_content).width(Fill).height(Fill).into()];

        let window_manager = window_manager::WindowManager::global().lock().unwrap();
        if let Some(window_content) = window_manager.get_window() {
            let window_element = crate::window_component::window_component(
                window_content.clone(),
                AppMessage::CloseWindow(None),
                AppMessage::OpenWindow(WindowContent {
                    window_type: WindowType::Warning,
                    title: "Warning".to_string(),
                    content: format!(
                        "You have {} windows open.",
                        window_manager.window_count() + 1
                    ),
                    window_width: None,
                }),
                AppMessage::CloseWindow(Some(false)),
                None::<iced::Element<'_, AppMessage>>,
            );
            layers.push(opaque(window_element));
        }

        stack(layers).into()
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
