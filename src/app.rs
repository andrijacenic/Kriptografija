use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use iced::Alignment::Center;
use iced::alignment::{Horizontal, Vertical};
use iced::border::radius;
use iced::widget::{
    Column, button, column, combo_box, container, row, scrollable, text, text_input,
};
use iced::widget::{opaque, stack};
use iced::{Border, Element, Renderer, Task, Theme, font};
use iced::{Fill, Length};
use iced_aw::{Menu, menu_bar, menu_items};
use iced_fonts::LUCIDE_FONT_BYTES;
use iced_fonts::lucide::plus;

use crate::entry_component::entry;
use crate::menu_button_component::menu_button;
use crate::search_component::search;
use crate::theme;
use crate::utils::{AppData, DataEntry};
use crate::window_component::{WindowContent, WindowType, custom_window};
use crate::window_manager::WindowManager;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputType {
    Key,
    Description,
    Search,
}

impl std::fmt::Display for InputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    OpenWindow(WindowContent<AppMessage>),
    CloseWindow((Option<WindowContent<AppMessage>>, bool)),
    AddEntry((DataEntry, Option<WindowContent<AppMessage>>)),
    InputChange(InputType, String),
    SearchChange(InputType),
    SaveAppData,
    DeleteEntry((DataEntry, bool)),
    EditEntry(DataEntry),
    AddNewEntry,
    ExitApp(bool),
    None,
}

pub struct App {
    window_manager: WindowManager<AppMessage>,
    app_data: AppData<AppMessage>,
    theme: Theme,
    editing_id: Option<uuid::Uuid>,
    key_input_value: String,
    decription_input_value: String,
    search_input_value: String,
    entries_sorted: Vec<DataEntry>,
    search_inputs: combo_box::State<InputType>,
    searched_input: Option<InputType>,
}

impl App {
    pub fn new() -> (Self, Task<AppMessage>) {
        let window_manager = WindowManager::new();
        let mut app_data = AppData::new(AppMessage::None);

        let load_result = app_data.load_file("data.txt".to_string());

        let init_task = match load_result {
            Ok(_) => Task::none(),
            Err(error) => Task::done(AppMessage::OpenWindow(WindowContent::new(
                WindowType::Error,
                "Error Loading App Data.".to_string(),
                format!("{:?}", error),
                None,
                false,
                true,
                None,
            ))),
        };

        let combined_tasks = Task::batch(vec![
            font::load(LUCIDE_FONT_BYTES).map(|result| match result {
                Err(e) => AppMessage::OpenWindow(WindowContent::new(
                    WindowType::Error,
                    "Font Load Error".to_string(),
                    format!("Failed to load Lucide font: {:?}", e),
                    None,
                    true,
                    true,
                    None,
                )),
                Ok(_) => AppMessage::None,
            }),
            init_task,
        ]);
        let entries_sorted = app_data.entries.clone();
        (
            Self {
                app_data,
                window_manager,
                theme: theme::default_theme(),
                editing_id: None,
                key_input_value: String::new(),
                decription_input_value: String::new(),
                search_input_value: String::new(),
                entries_sorted,
                search_inputs: combo_box::State::new(vec![InputType::Key, InputType::Description]),
                searched_input: Some(InputType::Key),
            },
            combined_tasks,
        )
    }

    pub fn update(&mut self, _message: AppMessage) -> Task<AppMessage> {
        match _message {
            AppMessage::OpenWindow(content) => {
                self.window_manager.add_window(content);
                Task::none()
            }
            AppMessage::CloseWindow((value, is_okay)) => match value {
                Some(window_content) => {
                    self.window_manager.remove_window_by_id(window_content.id);
                    if let Some(on_okay) = window_content.on_okay
                        && is_okay
                    {
                        Task::done(on_okay.as_ref().clone())
                    } else {
                        Task::none()
                    }
                }
                None => {
                    self.window_manager.remove_window();
                    Task::none()
                }
            },
            AppMessage::AddNewEntry => {
                self.editing_id = None;
                self.key_input_value = String::new();
                self.decription_input_value = String::new();
                Task::done(AppMessage::OpenWindow(WindowContent::new(
                    WindowType::EntryEditor,
                    "Add Entry".to_string(),
                    String::new(),
                    Some(600),
                    true,
                    true,
                    None,
                )))
            }
            AppMessage::DeleteEntry((entry, checked)) => {
                if let Some(pos) = self.app_data.entries.iter().position(|x| x.id == entry.id) {
                    if checked {
                        self.window_manager.remove_window();
                        self.app_data.entries.remove(pos);
                        self.search_entries();
                        Task::none()
                    } else {
                        Task::done(AppMessage::OpenWindow(WindowContent::new(
                            WindowType::Warning,
                            "Delete entry?".to_string(),
                            "Warning deleting an entry is not reversable!".to_string(),
                            None,
                            true,
                            true,
                            Some(AppMessage::DeleteEntry((entry, true))),
                        )))
                    }
                } else {
                    Task::none()
                }
            }
            AppMessage::EditEntry(entry) => {
                self.editing_id = Some(entry.id);
                self.key_input_value = entry.key;
                self.decription_input_value = entry.description;
                Task::done(AppMessage::OpenWindow(WindowContent::new(
                    WindowType::EntryEditor,
                    "Edit Entry".to_string(),
                    String::new(),
                    Some(600),
                    true,
                    true,
                    None,
                )))
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
                self.search_entries();
                Task::done(AppMessage::CloseWindow((window_content, true)))
            }
            AppMessage::InputChange(input_type, value) => {
                match input_type {
                    InputType::Key => self.key_input_value = value,
                    InputType::Description => self.decription_input_value = value,
                    InputType::Search => {
                        self.search_input_value = value;
                        self.search_entries();
                    }
                }
                Task::none()
            }
            AppMessage::SearchChange(value) => {
                self.searched_input = Some(value);
                self.search_entries();
                Task::none()
            }
            AppMessage::SaveAppData => {
                match self.app_data.save_file("data.txt".to_string()) {
                    Ok(_) => self.window_manager.add_window(WindowContent::new(
                        WindowType::Info,
                        "Data saved!".to_string(),
                        "Data saved successfully.".to_string(),
                        None,
                        false,
                        true,
                        None,
                    )),
                    Err(e) => self.window_manager.add_window(WindowContent::new(
                        WindowType::Error,
                        "Saving data error!".to_string(),
                        format!("Error saving data: {}", e),
                        None,
                        false,
                        true,
                        None,
                    )),
                }
                Task::none()
            }
            AppMessage::ExitApp(value) => {
                if value {
                    iced::exit()
                } else {
                    Task::done(AppMessage::OpenWindow(WindowContent::new(
                        WindowType::Warning,
                        "Leaving soo soon?".to_string(),
                        "Exit the app?\nIf you didn't save the data it will be lost.".to_string(),
                        None,
                        true,
                        true,
                        Some(AppMessage::ExitApp(true)),
                    )))
                }
            }
            AppMessage::None => Task::none(),
        }
    }

    // The UI layout
    pub fn view(&self) -> Element<'_, AppMessage> {
        let mut layers: Vec<Element<AppMessage, Theme, Renderer>> = vec![
            container(self.get_main_view())
                .width(Fill)
                .height(Fill)
                .into(),
        ];

        if let Some(window) = self.get_window_view() {
            layers.push(opaque(window));
        }

        stack(layers).into()
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn get_main_view(&self) -> Element<'_, AppMessage> {
        let mut entries_column: Column<AppMessage, Theme, Renderer> = column![search(
            self.search_input_value.as_str(),
            |value| { AppMessage::InputChange(InputType::Search, value) },
            &self.search_inputs,
            |value: InputType| AppMessage::SearchChange(value),
            self.searched_input.as_ref()
        )]
        .spacing(10)
        .padding(20);

        for e in &self.entries_sorted {
            entries_column = entries_column.push(entry(
                e,
                AppMessage::DeleteEntry((e.clone(), false)),
                AppMessage::EditEntry(e.clone()),
            ));
        }
        let add_button = container(button(plus()).on_press(AppMessage::AddNewEntry))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Right)
            .align_y(Vertical::Bottom)
            .padding(15);
        let layers: Vec<Element<AppMessage, Theme, Renderer>> = vec![
            column![self.get_menus(), scrollable(entries_column)].into(),
            add_button.into(),
        ];
        stack(layers).width(Fill).height(Fill).into()
    }

    fn get_menus(&self) -> Element<'_, AppMessage> {
        let menu_tpl = |items| {
            Menu::new(items)
                .width(150.0)
                .offset(5.0)
                .spacing(5.0)
                .close_on_item_click(true)
                .close_on_background_click(true)
        };

        // Build the Menu Bar
        let mb = menu_bar!(
            (
                menu_button(text("File")).on_press(AppMessage::None),
                menu_tpl(menu_items!(
                    (menu_button(text("Save").width(Length::Fill))
                        .on_press(AppMessage::SaveAppData)),
                    (menu_button(text("Exit").width(Length::Fill))
                        .on_press(AppMessage::ExitApp(false))),
                ))
            ),
            (menu_button(text("Info")).on_press(AppMessage::OpenWindow(WindowContent::new(
                WindowType::Info,
                "Info".to_string(),
                "Developed by: Andrija Cenić (1910)\nProject: Cryptography Course.".to_string(),
                None,
                false,
                true,
                None
            ))))
        );

        container(mb)
            .style(|theme: &Theme| container::Style {
                border: Border {
                    radius: radius(0),
                    width: 1.0,
                    color: theme.extended_palette().background.strongest.color,
                    ..Border::default()
                },
                ..Default::default()
            })
            .padding(1)
            .width(Length::Fill)
            .into()
    }

    fn get_window_view(&self) -> Option<Element<'_, AppMessage>> {
        if let Some(window_content) = self.window_manager.get_window() {
            let (custom_body, on_okay): (Option<Element<'_, AppMessage>>, Option<AppMessage>) =
                if let WindowType::EntryEditor = window_content.window_type {
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
                                None,
                            )))
                        },
                    )
                } else {
                    (
                        None,
                        Some(match &window_content.on_okay {
                            Some(boxed_msg) => (**boxed_msg).clone(),
                            None => AppMessage::CloseWindow((Some(window_content.clone()), true)),
                        }),
                    )
                };
            Some(custom_window(
                window_content.clone(),
                AppMessage::CloseWindow((Some(window_content.clone()), false)),
                on_okay.unwrap_or(AppMessage::CloseWindow((
                    Some(window_content.clone()),
                    true,
                ))),
                AppMessage::CloseWindow((Some(window_content.clone()), false)),
                custom_body,
            ))
        } else {
            None
        }
    }

    fn create_entity_add_window_body(&self) -> Element<'_, AppMessage> {
        let label_width = Length::Fixed(85.0);

        column![
            text("Add an entry below").size(16),
            row![
                container(text("Key").size(16).width(label_width).align_y(Center)).padding(5),
                text_input("Key", self.key_input_value.as_str())
                    .style(|theme, status| {
                        let mut style = iced::widget::text_input::default(theme, status);
                        if !self.is_key_input_valid() {
                            style.border.color = theme.palette().danger;
                            style.border.width = 1.0;
                        }
                        style
                    })
                    .on_input(|value| { AppMessage::InputChange(InputType::Key, value) })
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
                text_input("Description", self.decription_input_value.as_str())
                    .style(|theme, status| {
                        let mut style = iced::widget::text_input::default(theme, status);
                        if !self.is_description_input_valid() {
                            style.border.color = theme.palette().danger;
                            style.border.width = 1.0;
                        }
                        style
                    })
                    .on_input(|value| { AppMessage::InputChange(InputType::Description, value) })
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

    fn search_entries(&mut self) {
        if self.search_input_value.is_empty() {
            self.entries_sorted = self.app_data.entries.clone();
        } else {
            let matcher = SkimMatcherV2::default().ignore_case();

            let mut matches: Vec<(i64, &DataEntry)> = self
                .app_data
                .entries
                .iter()
                .filter_map(|entry| {
                    let threshold = 50;

                    let score = match self.searched_input.unwrap() {
                        InputType::Description => {
                            matcher.fuzzy_match(&entry.description, &self.search_input_value)
                        }
                        _ => matcher.fuzzy_match(&entry.key, &self.search_input_value),
                    };

                    score
                        .filter(|&score| score > threshold)
                        .map(|score| (score, entry))
                })
                .collect();

            matches.sort_by(|a, b| b.0.cmp(&a.0));

            self.entries_sorted = matches
                .into_iter()
                .map(|(_, entry)| entry.clone())
                .collect();
        }
    }
}
