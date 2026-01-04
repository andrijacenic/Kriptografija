use iced::Fill;
use iced::widget::{button, center, column, container, text};
use iced::widget::{opaque, stack};
use iced::{Element, Renderer, Task, Theme, font};
use iced_fonts::LUCIDE_FONT_BYTES;

use crate::utils::AppData;
use crate::window_component::{WindowContent, WindowType, custom_window};
use crate::{theme, window_manager};

#[derive(Debug, Clone)]
pub enum AppMessage {
    OpenWindow(WindowContent),
    CloseWindow(Option<bool>),
    SaveAppData,
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
            Task::batch(vec![font::load(LUCIDE_FONT_BYTES).map(
                |result| match result {
                    Err(e) => AppMessage::OpenWindow(WindowContent {
                        window_type: WindowType::Error,
                        title: "Font Load Error".to_string(),
                        content: format!("Failed to load Lucide font: {:?}", e),
                        window_width: None,
                    }),
                    Ok(_) => AppMessage::None,
                },
            )]),
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
            AppMessage::SaveAppData => {
                match self.app_data.save_file("data.txt".to_string()) {
                    Ok(_) => window_manager::WindowManager::global()
                        .lock()
                        .unwrap()
                        .add_window(WindowContent {
                            window_type: WindowType::Info,
                            title: "Data saved!".to_string(),
                            content: "Data saved successfully.".to_string(),
                            window_width: None,
                        }),
                    Err(e) => window_manager::WindowManager::global()
                        .lock()
                        .unwrap()
                        .add_window(WindowContent {
                            window_type: WindowType::Error,
                            title: "Saving data error!".to_string(),
                            content: format!("Error saving data: {}", e),
                            window_width: None,
                        }),
                }
                Task::none()
            }
            AppMessage::None => Task::none(),
        }
    }

    // The UI layout
    pub fn view(&self) -> Element<'_, AppMessage> {
        let main_content = self.get_main_view();

        let mut layers: Vec<Element<AppMessage, Theme, Renderer>> =
            vec![container(main_content).width(Fill).height(Fill).into()];

        if let Some(window) = self.get_window_view() {
            layers.push(opaque(window));
        }

        stack(layers).into()
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn get_main_view(&self) -> Element<'_, AppMessage> {
        center(
            column![
                text("Hello, World!").size(50),
                button("Open Window").on_press(AppMessage::OpenWindow(WindowContent {
                    window_type: WindowType::Info,
                    title: "Info".to_string(),
                    content: "This is an informational window.".to_string(),
                    window_width: None,
                })),
                button("Save File").on_press(AppMessage::SaveAppData)
            ]
            .spacing(20),
        )
        .into()
    }

    fn get_window_view(&self) -> Option<Element<'_, AppMessage>> {
        let window_manager = window_manager::WindowManager::global().lock().unwrap();
        if let Some(window_content) = window_manager.get_window() {
            Some(custom_window(
                window_content.clone(),
                AppMessage::CloseWindow(None),
                AppMessage::CloseWindow(Some(true)),
                AppMessage::CloseWindow(Some(false)),
                None::<iced::Element<'_, AppMessage>>,
            ))
        } else {
            None
        }
    }
}
