use winit::event_loop::{ActiveEventLoop};
use winit::window::{Window as WinitWindow};

pub struct Window {
    window: WinitWindow,
}

impl Window {
    pub fn new(event_loop: &ActiveEventLoop) -> Self {
        let attrs = WinitWindow::default_attributes();
        let window = ActiveEventLoop::create_window(event_loop, attrs).unwrap();

        Self { window }
    }

    pub fn set_title(&mut self, title: &str) {
        self.window.set_title(title);
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.window.inner_size().width = width;
        self.window.inner_size().height = height;
    }

    pub fn set_maximized(&mut self, maximized: bool) {
        self.window.set_maximized(maximized);
    }
}