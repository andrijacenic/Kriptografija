use crate::utils::AppData;
use crate::window_manager::WindowContentBase;
use crate::{theme, window_manager};
use iced::Length::Fill;
use iced::widget::{button, center, column, container, text};
use iced::widget::{opaque, stack};
use iced::{Color, Element, Length, Renderer, Task, Theme};
use iced_aw::card;

#[derive(Debug, Clone)]
pub enum AppMessage {
    OpenWindow(WindowContentBase),
    None,
}

pub struct App {
    app_data: AppData,
}

impl App {
    // Initialize the application state
    pub fn new() -> (Self, Task<AppMessage>) {
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
    pub fn update(&mut self, _message: AppMessage) -> Task<AppMessage> {
        match _message {
            AppMessage::OpenWindow(content) => {
                window_manager::WindowManager::global()
                    .lock()
                    .unwrap()
                    .add_window(content);
                Task::none()
            }
            AppMessage::None => {
                window_manager::WindowManager::global()
                    .lock()
                    .unwrap()
                    .remove_window();
                Task::none()
            }
        }
    }

    // The UI layout
    pub fn view(&self) -> Element<'_, AppMessage> {
        let main_content = center(
            column![
                text("Hello, ASDASDASDASDASDASDASDASDASDASDASDAS!").size(50),
                button("Click Me").on_press(AppMessage::OpenWindow(WindowContentBase {
                    window_type: crate::window_manager::WindowType::Info,
                    title: "Info".to_string(),
                    content: "This is an informational window.".to_string(),
                })),
            ]
            .spacing(20)
            .padding(30),
        );

        let mut layers: Vec<Element<AppMessage, Theme, Renderer>> =
            vec![container(main_content).width(Fill).height(Fill).into()];
        let window_manager = window_manager::WindowManager::global().lock().unwrap();
        if let Some(window_content) = window_manager.get_window() {
            let window_element = crate::window_component::window_component(
                window_content.clone(),
                AppMessage::None,
                AppMessage::OpenWindow(WindowContentBase {
                    window_type: crate::window_manager::WindowType::Info,
                    title: "Info".to_string(),
                    content: format!("You have {} windows open.", window_manager.window_count()),
                }),
                AppMessage::None,
            );
            layers.push(window_element);
        }

        stack(layers).into()
    }

    pub fn theme(&self) -> Theme {
        theme::default_theme()
    }
}
