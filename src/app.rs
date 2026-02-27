use std::path::PathBuf;

use fuse_rust::Fuse;
use iced::alignment::{Horizontal, Vertical};
use iced::border::radius;
use iced::widget::{Column, button, column, combo_box, container, image::viewer, scrollable, text};
use iced::widget::{opaque, stack};
use iced::{Border, Element, Renderer, Task, Theme, font};
use iced::{Fill, Length};
use iced_aw::{Menu, menu_bar, menu_items};
use iced_fonts::LUCIDE_FONT_BYTES;
use iced_fonts::lucide::plus;

use crate::base_description_component::{
    DescriptionElement, DescriptionImage, DescriptionSound, Link, parse_description_elements,
    serialize_description_elements,
};
use crate::entity_edit_component::{InputChange, entity_edit};
use crate::entry_component::entry;
use crate::menu_button_component::menu_button;
use crate::search_component::search;
use crate::theme;
use crate::utils::{AppData, DataEntry};
use crate::window_component::{WindowContent, WindowContentType, WindowType, custom_window};
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

#[derive(Clone)]

pub enum OpenType {
    OpenLink(Link),
    OpenImage(DescriptionImage),
    OpenSound(DescriptionSound),
}

#[derive(Clone)]
pub enum AppMessage {
    OpenWindow(WindowContent<AppMessage>),
    CloseWindow((Option<WindowContent<AppMessage>>, bool)),
    AddEntry((DataEntry, Option<WindowContent<AppMessage>>)),
    InputChange(InputType, String),
    SearchChange(InputType),
    SaveAppData(bool),
    SaveTo(String),
    DeleteEntry((DataEntry, bool)),
    EditEntry(DataEntry),
    AddNewEntry,
    ExitApp(bool),
    OpenFile(bool),
    FileSelected(PathBuf),
    OpenLink(OpenType),
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
    fuse: Fuse,
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
                WindowContentType::StringContent(format!("{:?}", error)),
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
                    WindowContentType::StringContent(format!(
                        "Failed to load Lucide font: {:?}",
                        e
                    )),
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
                fuse: Fuse {
                    max_pattern_length: 100,
                    ..Default::default()
                },
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
                    WindowContentType::StringContent(String::new()),
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
                            WindowContentType::StringContent(
                                "Warning deleting an entry is not reversable!".to_string(),
                            ),
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
                self.decription_input_value = serialize_description_elements(entry.description);
                Task::done(AppMessage::OpenWindow(WindowContent::new(
                    WindowType::EntryEditor,
                    "Edit Entry".to_string(),
                    WindowContentType::StringContent(String::new()),
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
            AppMessage::SaveAppData(save_as) => {
                if save_as {
                    Task::perform(
                        async {
                            rfd::AsyncFileDialog::new()
                                .set_title("Save Your File")
                                .add_filter("Text", &["txt"])
                                .set_file_name("untitled.rs")
                                .save_file()
                                .await
                                .map(|handle| handle.path().to_path_buf())
                        },
                        |path_buf: Option<PathBuf>| {
                            if let Some(path_buf) = path_buf {
                                return AppMessage::SaveTo(
                                    path_buf.to_str().unwrap_or("").to_string(),
                                );
                            }
                            AppMessage::OpenWindow(WindowContent::new(
                                WindowType::Error,
                                "Access Error".to_string(),
                                WindowContentType::StringContent(
                                    "There was an error accessing files!".to_string(),
                                ),
                                None,
                                false,
                                true,
                                None,
                            ))
                        },
                    )
                } else {
                    Task::done(AppMessage::SaveTo("data.txt".to_string()))
                }
            }
            AppMessage::SaveTo(path) => {
                match self.app_data.save_file(path) {
                    Ok(_) => self.window_manager.add_window(WindowContent::new(
                        WindowType::Info,
                        "Data saved!".to_string(),
                        WindowContentType::StringContent("Data saved successfully.".to_string()),
                        None,
                        false,
                        true,
                        None,
                    )),
                    Err(e) => self.window_manager.add_window(WindowContent::new(
                        WindowType::Error,
                        "Saving data error!".to_string(),
                        WindowContentType::StringContent(format!("Error saving data: {}", e)),
                        None,
                        false,
                        true,
                        None,
                    )),
                }
                Task::none()
            }
            AppMessage::OpenFile(checked) => {
                if checked {
                    Task::perform(
                        async {
                            rfd::AsyncFileDialog::new()
                                .set_title("Open a File")
                                .add_filter("Text Files", &["txt"])
                                .pick_file()
                                .await
                                .map(|handle| handle.path().to_path_buf())
                        },
                        |path_buf: Option<PathBuf>| {
                            if let Some(path_buf) = path_buf {
                                return AppMessage::FileSelected(path_buf);
                            }
                            AppMessage::OpenWindow(WindowContent::new(
                                WindowType::Error,
                                "Access Error".to_string(),
                                WindowContentType::StringContent(
                                    "There was an error accessing files!".to_string(),
                                ),
                                None,
                                false,
                                true,
                                None,
                            ))
                        },
                    )
                } else {
                    Task::done(AppMessage::OpenWindow(WindowContent::new(
                        WindowType::Warning,
                        "If data not saved it will be lost!".to_string(),
                        WindowContentType::StringContent(
                            "Warning if data was not saved it will be lost!".to_string(),
                        ),
                        None,
                        true,
                        true,
                        Some(AppMessage::OpenFile(true)),
                    )))
                }
            }
            AppMessage::FileSelected(path_buf) => {
                if let Some(path_str) = path_buf.to_str() {
                    let load_result = self.app_data.load_file(path_str.to_string());
                    let res = match load_result {
                        Err(e) => AppMessage::OpenWindow(WindowContent::new(
                            WindowType::Error,
                            "Error loading file!".to_string(),
                            WindowContentType::StringContent(format!(
                                "There was an error while loading file: {:?}",
                                e
                            )),
                            None,
                            false,
                            true,
                            None,
                        )),
                        Ok(_) => AppMessage::CloseWindow((None, false)),
                    };
                    self.search_entries();
                    Task::done(res)
                } else {
                    Task::done(AppMessage::OpenWindow(WindowContent::new(
                        WindowType::Error,
                        "Error loading file!".to_string(),
                        WindowContentType::StringContent(
                            "There was an error while loading file!".to_string(),
                        ),
                        None,
                        false,
                        true,
                        Some(AppMessage::CloseWindow((None, false))),
                    )))
                }
            }
            AppMessage::ExitApp(value) => {
                if value {
                    iced::exit()
                } else {
                    Task::done(AppMessage::OpenWindow(WindowContent::new(
                        WindowType::Warning,
                        "Leaving soo soon?".to_string(),
                        WindowContentType::StringContent(
                            "Exit the app?\nIf you didn't save the data it will be lost."
                                .to_string(),
                        ),
                        None,
                        true,
                        true,
                        Some(AppMessage::ExitApp(true)),
                    )))
                }
            }
            AppMessage::OpenLink(link) => match link {
                OpenType::OpenImage(image) => {
                    println!("{:?}", image);
                    Task::done(AppMessage::OpenWindow(WindowContent::new(
                        WindowType::Image,
                        "View Image".to_string(),
                        WindowContentType::ImageContant(image),
                        Some(600),
                        false,
                        true,
                        None,
                    )))
                }
                OpenType::OpenLink(link) => {
                    println!("{:?}", link);
                    if !webbrowser::open(&link.link).is_ok() {
                        Task::done(AppMessage::OpenWindow(WindowContent::new(
                            WindowType::Warning,
                            "Cannot Open Link?".to_string(),
                            WindowContentType::StringContent("Link cannot be opened.".to_string()),
                            None,
                            false,
                            true,
                            None,
                        )))
                    } else {
                        Task::none()
                    }
                }
                OpenType::OpenSound(sound) => {
                    println!("{:?}", sound);
                    Task::none()
                }
            },
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
                |value| match value {
                    DescriptionElement::Image(image) => {
                        AppMessage::OpenLink(OpenType::OpenImage(image))
                    }
                    DescriptionElement::Link(link) => {
                        AppMessage::OpenLink(OpenType::OpenLink(link))
                    }
                    DescriptionElement::Sound(sound) => {
                        AppMessage::OpenLink(OpenType::OpenSound(sound))
                    }
                    DescriptionElement::Text(_) => AppMessage::None,
                },
                &self.theme,
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
                    (menu_button(text("Open").width(Length::Fill))
                        .on_press(AppMessage::OpenFile(false))),
                    (menu_button(text("Save").width(Length::Fill))
                        .on_press(AppMessage::SaveAppData(false))),
                    (menu_button(text("Save As").width(Length::Fill))
                        .on_press(AppMessage::SaveAppData(true))),
                    (menu_button(text("Exit").width(Length::Fill))
                        .on_press(AppMessage::ExitApp(false))),
                ))
            ),
            (menu_button(text("Info")).on_press(AppMessage::OpenWindow(WindowContent::new(
                WindowType::Info,
                "Info".to_string(),
                WindowContentType::StringContent(
                    "Developed by: Andrija Cenić (1910)\nProject: Cryptography Course.".to_string()
                ),
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
                match window_content.window_type {
                    WindowType::EntryEditor => (
                        Some(self.create_entity_add_window_body()),
                        if self.is_data_entry_valid() {
                            Some(AppMessage::AddEntry((
                                DataEntry {
                                    id: self.editing_id.unwrap_or(uuid::Uuid::new_v4()),
                                    key: self.key_input_value.clone(),
                                    description: parse_description_elements(
                                        self.decription_input_value.clone(),
                                    ),
                                    description_raw: self.decription_input_value.clone(),
                                },
                                Some(window_content.clone()),
                            )))
                        } else {
                            Some(AppMessage::OpenWindow(WindowContent::new(
                                WindowType::Warning,
                                "Invalid Input data".to_string(),
                                WindowContentType::StringContent(
                                    "Key and Description cannot be empty.".to_string(),
                                ),
                                None,
                                false,
                                true,
                                None,
                            )))
                        },
                    ),
                    WindowType::Image => (
                        self.create_image_view_window_body(match window_content.content.clone() {
                            WindowContentType::ImageContant(image) => Some(image.image),
                            _ => None,
                        })
                        .into(),
                        Some(match &window_content.on_okay {
                            Some(boxed_msg) => (**boxed_msg).clone(),
                            None => AppMessage::CloseWindow((Some(window_content.clone()), true)),
                        }),
                    ),
                    _ => (
                        None,
                        Some(match &window_content.on_okay {
                            Some(boxed_msg) => (**boxed_msg).clone(),
                            None => AppMessage::CloseWindow((Some(window_content.clone()), true)),
                        }),
                    ),
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
        entity_edit(
            Length::Fixed(85.0),
            self.key_input_value.as_str(),
            !self.is_key_input_valid(),
            self.decription_input_value.as_str(),
            !self.is_description_input_valid(),
            |change: InputChange| match change {
                InputChange::Key(value) => AppMessage::InputChange(InputType::Key, value),
                InputChange::Description(value) => {
                    AppMessage::InputChange(InputType::Description, value)
                }
            },
        )
    }

    fn create_image_view_window_body(&self, image_path: Option<String>) -> Element<'_, AppMessage> {
        match image_path {
            Some(path) => iced::widget::image(path)
                .width(Length::Fill)
                .height(Length::Fill)
                .into(),
            None => text("No image found!").into(),
        }
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
            let search_query = self.search_input_value.as_str();
            let fuse = &self.fuse;

            let mut scored_entries: Vec<(DataEntry, f64)> = self
                .app_data
                .entries
                .iter()
                .filter_map(|entry| {
                    let search_text = match self.searched_input.unwrap() {
                        InputType::Key => entry.key.as_str(),
                        _ => entry.description_raw.as_str(),
                    };
                    let score_result = fuse.search_text_in_string(search_query, search_text);

                    match score_result {
                        Some(result) if result.score <= 0.5 => Some((entry.clone(), result.score)),
                        _ => None,
                    }
                })
                .collect();

            scored_entries
                .sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

            self.entries_sorted = scored_entries
                .into_iter()
                .map(|(entry, _score)| entry)
                .collect();
        }
    }
}
