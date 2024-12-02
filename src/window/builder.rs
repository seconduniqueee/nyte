use winit::dpi::LogicalSize;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window as WinitWindow};

pub struct Window {
    window: WinitWindow,
}

impl Window {
    pub fn new(event_loop: &ActiveEventLoop, title: &str, width: i32, height: i32, maximized: bool) -> Self {
        let attrs = WinitWindow::default_attributes()
            .with_title(title)
            .with_maximized(maximized)
            .with_inner_size(LogicalSize::new(width, height));

        let window = ActiveEventLoop::create_window(event_loop, attrs).unwrap();

        Self { window }
    }

    pub fn set_title(&mut self, title: &str) {
        self.window.set_title(title);
    }

    pub fn set_maximized(&mut self, maximized: bool) {
        self.window.set_maximized(maximized);
    }
}