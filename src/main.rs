mod window;
mod application;

use std::thread::{sleep, spawn};
use std::time::Duration;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::platform::windows::{WindowExtWindows};
use winit::window::WindowId;
use crate::window::builder::Window;


fn main() {
    let event_loop: EventLoop<CustomEvent> = EventLoop::<CustomEvent>::with_user_event()
        .build()
        .unwrap();
    let proxy = event_loop.create_proxy();
    let mut app = App { window: None };

    spawn(move || loop {
        sleep(Duration::from_millis(1000));
        proxy.send_event(CustomEvent::Message("Message from thread...")).unwrap()
    });

    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app).unwrap();
}

#[derive(Debug)]
enum CustomEvent {
    Message(&'static str),
}

struct App {
    window: Option<Window>
}

impl ApplicationHandler<CustomEvent> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Window::new(event_loop, "title", 500, 500, true);
        self.window = Some(window);
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: CustomEvent) {
        match event {
            CustomEvent::Message(msg) => {
                println!("Message Received: {}", msg);
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Exiting...");
                event_loop.exit();
            }
            _ => ()
        }
    }
}

impl App {
    fn window(&mut self) -> Window {
        self.window.take().unwrap()
    }
}