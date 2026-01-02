use std::sync::{Mutex, OnceLock};

use crate::window_component::WindowContent;

pub struct WindowManager {
    windows: Vec<WindowContent>,
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

    pub fn add_window(&mut self, window: WindowContent) {
        self.windows.push(window);
    }

    pub fn get_window(&self) -> Option<&WindowContent> {
        if self.windows.is_empty() {
            return None;
        }
        self.windows.last()
    }

    pub fn remove_window(&mut self) -> Option<WindowContent> {
        self.windows.pop()
    }

    pub fn window_count(&self) -> usize {
        self.windows.len()
    }
}
