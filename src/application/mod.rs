use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use pollster::block_on;
use winit::application::ApplicationHandler;
use winit::event::{StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};
use crate::renderer::State;

pub trait Application {
    fn init(&mut self, window: &mut Window) -> ();
    fn update(&mut self) -> ();
    fn render(&mut self) -> ();
    fn handle_event(&mut self, event: WindowEvent) -> () {}
}

struct Runner<'a> {
    window: Option<Arc<Window>>,
    app: &'a mut dyn Application,
    state: Option<State<'a>>
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

    fn resumed(&mut self, event_loop: &ActiveEventLoop)  {
        let attrs = WindowAttributes::default();
        let window = event_loop.create_window(attrs).unwrap();
        let window = Arc::new(window);
        let state = block_on(State::new(window.clone()));

        self.window = Some(window.clone());
        self.state = Some(state);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        self.app.handle_event(event.clone());

        if let WindowEvent::CloseRequested = event {
            event_loop.exit();
        }

        if let WindowEvent::Resized(size) = event {
            self.state.as_mut().unwrap().resize(size);
        }
    }
}

pub fn run_app(app: &mut impl Application) {
    let mut event_loop = EventLoop::new().unwrap();
    let mut runner = Runner { window: None, state: None, app };

    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut runner).unwrap();
}