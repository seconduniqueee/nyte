use winit::application::ApplicationHandler;
use winit::event::{StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::WindowId;
use crate::window::Window;

pub trait Application {
    fn init(&mut self, window: &mut Window) -> ();
    fn update(&mut self) -> ();
    fn render(&mut self) -> ();
    fn handle_event(&mut self, event: WindowEvent) -> () {}
}

struct Runner<'a> {
    window: Option<Window>,
    app: &'a mut dyn Application,
}

impl<'a> ApplicationHandler for Runner<'a> {
    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        match cause {
            StartCause::Init => {
                // init
            }
            StartCause::Poll => {
                self.app.update();
                self.app.render();
            }
            _ => ()
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(Window::new(event_loop));
        self.app.init(self.window.as_mut().unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        self.app.handle_event(event.clone());

        if let WindowEvent::CloseRequested = event {
            event_loop.exit();
        }
    }
}

pub fn run_app(app: &mut impl Application) {
    let mut event_loop = EventLoop::new().unwrap();
    let mut runner = Runner { window: None, app };

    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut runner).unwrap();
}