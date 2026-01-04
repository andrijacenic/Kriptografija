use crate::window_component::WindowContent;

pub struct WindowManager<Message> {
    windows: Vec<WindowContent<Message>>,
}

impl<Message> WindowManager<Message> {
    pub fn new() -> WindowManager<Message> {
        return WindowManager {
            windows: Vec::new(),
        };
    }
    pub fn add_window(&mut self, window: WindowContent<Message>) {
        self.windows.push(window);
    }

    pub fn get_window(&self) -> Option<&WindowContent<Message>> {
        if self.windows.is_empty() {
            return None;
        }
        self.windows.last()
    }

    pub fn remove_window(&mut self) -> Option<WindowContent<Message>> {
        self.windows.pop()
    }

    pub fn remove_window_by_id(&mut self, id: uuid::Uuid) -> Option<WindowContent<Message>> {
        if let Some(pos) = self.windows.iter().position(|x| x.id == id) {
            Some(self.windows.remove(pos))
        } else {
            None
        }
    }
}
