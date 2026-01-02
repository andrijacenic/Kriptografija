use std::sync::{Mutex, OnceLock};

pub enum WindowMessage {
    Response(bool),
    Close,
}

#[derive(Debug, Clone, Copy)]

pub enum WindowType {
    Info,
    Warning,
    Error,
    AddElement,
}

#[derive(Debug, Clone)]
pub struct WindowContentBase {
    pub window_type: WindowType,
    pub title: String,
    pub content: String,
}

pub struct WindowManager {
    windows: Vec<WindowContentBase>,
}

impl WindowManager {
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

    pub fn window_count(&self) -> usize {
        self.windows.len()
    }
}
