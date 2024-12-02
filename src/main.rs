mod window;
mod application;

use winit::application::ApplicationHandler;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::windows::{WindowExtWindows};
use crate::window::builder::Window;

enum CustomEvent {
    Message(&'static str),
}

fn main() {
    let event_loop: EventLoop<CustomEvent> = EventLoop::<CustomEvent>::with_user_event()
        .build()
        .unwrap();

    let window = Window::<CustomEvent>::new(&event_loop, "title", 0, 0, true);

    event_loop.set_control_flow(ControlFlow::Poll);
}