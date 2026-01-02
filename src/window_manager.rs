use std::sync::{Mutex, OnceLock};

pub enum WindowMessage {
    Response(bool),
    Close,
}

pub enum WindowType {
    Info,
    Warning,
    Error,
    AddElement,
}

pub struct WindowContentBase {
    pub window_type: WindowType,
    pub title: String,
    pub content: String,
}

struct WindowManager {
    windows: Vec<WindowContentBase>,
}

impl WindowManager {
    // The "static method" to get the instance
    pub fn global() -> &'static Mutex<WindowManager> {
        static INSTANCE: OnceLock<Mutex<WindowManager>> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            Mutex::new(WindowManager {
                windows: Vec::new(),
            })
        })
    }

    pub fn add_window(&mut self, window: WindowContentBase) {
        self.windows.push(window);
    }

    pub fn get_window(&self) -> Option<&WindowContentBase> {
        if self.windows.is_empty() {
            return None;
        }
        self.windows.last()
    }

    pub fn remove_window(&mut self) -> Option<WindowContentBase> {
        self.windows.pop()
    }

    // TODO: Should we implement the renderer method here or in App?
    pub fn render_window(&self) {
        let window = self.get_window();
        if let Some(window) = window {
            match window.window_type {
                WindowType::Info => {
                    // Render info window
                }
                WindowType::Warning => {
                    // Render warning window
                }
                WindowType::Error => {
                    // Render error window
                }
                WindowType::AddElement => {
                    // Render add element window
                }
            }
        }
    }
}
// let window_manager = WindowManager::global().lock().unwrap();
