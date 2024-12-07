use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use pollster::block_on;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowAttributes, WindowId};
use crate::renderer::State;

pub trait Application {
    fn init(&mut self, window: &mut Window) -> ();
    fn update(&mut self) -> ();
    fn render(&mut self, state: &mut State) -> ();
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
                let state = self.state.as_mut().unwrap();

                self.app.update();
                self.app.render(state);
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
        let state = self.state.as_mut().unwrap();
        let id = self.window.as_mut().unwrap().id();

        if id != window_id || state.input(&event) {
            println!("Window does not match or previous event is not processed. Returning...");
            return;
        }

        match event {
            WindowEvent::CloseRequested => { event_loop.exit(); }
            WindowEvent::Resized(size) => { state.resize(size); }
            WindowEvent::KeyboardInput { event: evt, .. } => {
                if evt.physical_key == PhysicalKey::Code(KeyCode::Space) && evt.state == ElementState::Pressed {
                    state.set_next_index_offset();
                }
            }
            WindowEvent::RedrawRequested => {
                state.update();
                self.app.render(state);
            }
            _ => ()
        }
    }
}

pub fn run_app(app: &mut impl Application) {
    let mut event_loop = EventLoop::new().unwrap();
    let mut runner = Runner { window: None, state: None, app };

    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut runner).unwrap();
}