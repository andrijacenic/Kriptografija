use iced::Alignment::Center;
use iced::widget::{button, center, column, container, row, text, text_input};
use iced::widget::{opaque, stack};
use iced::{Color, Element, Renderer, Task, Theme, font};
use iced::{Fill, Length};
use iced_fonts::LUCIDE_FONT_BYTES;

use crate::theme;
use crate::utils::{AppData, DataEntry};
use crate::window_component::{WindowContent, WindowType, custom_window};
use crate::window_manager::WindowManager;

#[derive(Debug, Clone)]
pub enum InputChangeType {
    Key,
    Description,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    OpenWindow(WindowContent),
    CloseWindow(Option<WindowContent>),
    AddEntry((DataEntry, Option<WindowContent>)),
    InputChange(InputChangeType, String),
    SaveAppData,
    None,
}

pub struct App {
    app_data: AppData,
    theme: Theme,
    editing_id: Option<uuid::Uuid>,
    key_input_value: String,
    decription_input_value: String,
}

impl App {
    pub fn new() -> (Self, Task<AppMessage>) {
        let (app_data, initial_error) = match AppData::load_file("data.txt".to_string()) {
            Ok(data) => (data, None),
            Err(e) => (AppData::new().unwrap(), Some(e.to_string())),
        };

        if let Some(error_msg) = initial_error {
            WindowManager::global()
                .lock()
                .unwrap()
                .add_window(WindowContent::new(
                    WindowType::Error,
                    "Loading data error!".to_string(),
                    format!("Error loading data: {}", error_msg),
                    None,
                    true,
                    true,
                ));
        }

        (
            Self {
                app_data,
                theme: theme::default_theme(),
                editing_id: None,
                key_input_value: String::new(),
                decription_input_value: String::new(),
            },
            Task::batch(vec![font::load(LUCIDE_FONT_BYTES).map(
                |result| match result {
                    Err(e) => AppMessage::OpenWindow(WindowContent::new(
                        WindowType::Error,
                        "Font Load Error".to_string(),
                        format!("Failed to load Lucide font: {:?}", e),
                        None,
                        true,
                        true,
                    )),
                    Ok(_) => AppMessage::None,
                },
            )]),
        )
    }

    pub fn update(&mut self, _message: AppMessage) -> Task<AppMessage> {
        match _message {
            AppMessage::OpenWindow(content) => {
                WindowManager::global().lock().unwrap().add_window(content);
                Task::none()
            }
            AppMessage::CloseWindow(value) => {
                match value {
                    Some(window_content) => {
                        WindowManager::global()
                            .lock()
                            .unwrap()
                            .remove_window_by_id(window_content.id);
                    }
                    None => {
                        WindowManager::global().lock().unwrap().remove_window();
                    }
                }
                Task::none()
            }
            AppMessage::AddEntry((entry, window_content)) => {
                if let Some(existing_entry) = self
                    .app_data
                    .entries
                    .iter_mut()
                    .find(|el| el.id == entry.id)
                {
                    existing_entry.key = entry.key;
                    existing_entry.description = entry.description;
                } else {
                    self.app_data.entries.push(entry);
                }
                Task::done(AppMessage::CloseWindow(window_content))
            }
            AppMessage::InputChange(input_type, value) => {
                match input_type {
                    InputChangeType::Key => {
                        self.key_input_value = value;
                    }
                    InputChangeType::Description => {
                        self.decription_input_value = value;
                    }
                }
                Task::none()
            }
            AppMessage::SaveAppData => {
                match self.app_data.save_file("data.txt".to_string()) {
                    Ok(_) => {
                        WindowManager::global()
                            .lock()
                            .unwrap()
                            .add_window(WindowContent::new(
                                WindowType::Info,
                                "Data saved!".to_string(),
                                "Data saved successfully.".to_string(),
                                None,
                                true,
                                true,
                            ))
                    }
                    Err(e) => {
                        WindowManager::global()
                            .lock()
                            .unwrap()
                            .add_window(WindowContent::new(
                                WindowType::Error,
                                "Saving data error!".to_string(),
                                format!("Error saving data: {}", e),
                                None,
                                true,
                                true,
                            ))
                    }
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
                button("Open Window").on_press(AppMessage::OpenWindow(WindowContent::new(
                    WindowType::AddEntry,
                    "Add Entry".to_string(),
                    "Add entry".to_string(),
                    Some(600),
                    true,
                    true,
                ))),
                button("Save File").on_press(AppMessage::SaveAppData)
            ]
            .spacing(20),
        )
        .into()
    }

    fn get_window_view(&self) -> Option<Element<'_, AppMessage>> {
        let window_manager = WindowManager::global().lock().unwrap();
        if let Some(window_content) = window_manager.get_window() {
            let (custom_body, on_okay): (Option<Element<'_, AppMessage>>, Option<AppMessage>) =
                if let WindowType::AddEntry = window_content.window_type {
                    (
                        Some(self.create_entity_add_window_body()),
                        if self.is_data_entry_valid() {
                            Some(AppMessage::AddEntry((
                                DataEntry {
                                    id: self.editing_id.unwrap_or(uuid::Uuid::new_v4()),
                                    key: self.key_input_value.clone(),
                                    description: self.decription_input_value.clone(),
                                },
                                Some(window_content.clone()),
                            )))
                        } else {
                            Some(AppMessage::OpenWindow(WindowContent::new(
                                WindowType::Warning,
                                "Invalid Input data".to_string(),
                                "Key and Description cannot be empty.".to_string(),
                                None,
                                false,
                                true,
                            )))
                        },
                    )
                } else {
                    (
                        None,
                        Some(AppMessage::CloseWindow(Some(window_content.clone()))),
                    )
                };
            Some(custom_window(
                window_content.clone(),
                AppMessage::CloseWindow(Some(window_content.clone())),
                on_okay.unwrap_or(AppMessage::CloseWindow(Some(window_content.clone()))),
                AppMessage::CloseWindow(Some(window_content.clone())),
                custom_body,
            ))
        } else {
            None
        }
    }

    fn create_entity_add_window_body(&self) -> Element<'_, AppMessage> {
        let label_width = Length::Fixed(85.0);

        column![
            text("Add an entry below").size(20),
            row![
                container(text("Key").width(label_width).align_y(Center)).padding(5),
                text_input("Key", self.key_input_value.as_str())
                    .style(|theme, status| {
                        let mut style = iced::widget::text_input::default(theme, status);
                        if !self.is_key_input_valid() {
                            style.border.color = Color::from_rgb(0.8, 0.0, 0.0); // Red border
                            style.border.width = 1.0;
                        }
                        style
                    })
                    .on_input(|value| { AppMessage::InputChange(InputChangeType::Key, value) })
            ]
            .spacing(10),
            row![
                container(text("Description").width(label_width).align_y(Center)).padding(5),
                text_input("Description", self.decription_input_value.as_str())
                    .style(|theme, status| {
                        let mut style = iced::widget::text_input::default(theme, status);
                        if !self.is_description_input_valid() {
                            style.border.color = Color::from_rgb(0.8, 0.0, 0.0); // Red border
                            style.border.width = 1.0;
                        }
                        style
                    })
                    .on_input(|value| {
                        AppMessage::InputChange(InputChangeType::Description, value)
                    })
            ]
            .spacing(10)
        ]
        .spacing(15)
        .into()
    }

    fn is_key_input_valid(&self) -> bool {
        !self.key_input_value.trim().is_empty()
    }

    fn is_description_input_valid(&self) -> bool {
        !self.decription_input_value.trim().is_empty()
    }

    fn is_data_entry_valid(&self) -> bool {
        self.is_key_input_valid() && self.is_description_input_valid()
    }
}
