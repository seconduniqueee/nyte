use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::window::{Window, WindowId};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::platform::windows::{Color, WindowExtWindows};

#[derive(Default)]
struct App {
    window: Option<Window>,
}

#[derive(Clone, Copy, Debug)]
enum CustomEvent {
    Timer,
    Message,
}

impl ApplicationHandler<CustomEvent> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: CustomEvent) {
        match event {
            CustomEvent::Timer => {
                self.window.as_ref().unwrap().request_redraw();
            }
            CustomEvent::Message => {
                println!("Message received")
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },

            WindowEvent::Resized(size) => {
                println!("Resizing window: {:?}", size);
                self.window.as_ref().unwrap().request_redraw();
                self.window.as_ref().unwrap().set_border_color(Some(Color::from_rgb(255, 0, 0)));
            }

            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => ()
        }
    }
}

fn main() {
    let event_loop: EventLoop<CustomEvent> = EventLoop::<CustomEvent>::with_user_event()
        .build()
        .unwrap();

    let mut app = App::default();
    let proxy = event_loop.create_proxy();

    std::thread::spawn(move || loop{
        std::thread::sleep(std::time::Duration::from_millis(17));
        proxy.send_event(CustomEvent::Timer).ok();
    });

    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app).unwrap();

}
