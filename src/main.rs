mod window;

use winit::application::ApplicationHandler;
use winit::event::{ElementState, WindowEvent};
use winit::window::{Window, WindowId};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode};
use winit::keyboard::PhysicalKey::Code;
use winit::platform::windows::{Color, WindowExtWindows};

#[derive(Default)]
struct App {
    window: Option<Window>,
    game_state: GameState,
}

#[derive(Clone, Copy, Debug)]
enum CustomEvent {
    Timer,
    Message(&'static str),
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    Other
}

struct GameState {
    screen_color: (u8, u8, u8),
}

impl GameState {
    fn update_screen_color(&mut self, screen_color: (u8, u8, u8)) -> &GameState {
        self.screen_color =  screen_color;
        self
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState { screen_color: (0, 0, 255) }
    }
}

impl ApplicationHandler<CustomEvent> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attrs = Window::default_attributes()
            .with_title("Nyte Game Engine")
            .with_maximized(true);

        self.window = Some(event_loop.create_window(attrs).unwrap());
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: CustomEvent) {
        match event {
            CustomEvent::Timer => {
                self.window.as_ref().unwrap().request_redraw();
            }
            CustomEvent::Message(message) => {
                println!("Message received: {message}")
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
            }

            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            }

            WindowEvent::MouseInput {device_id, state, button} => {
                println!("Device ID: {:?}", device_id);
                println!("State: {:?}", state);
                println!("Button: {:?}", button);
            }

            WindowEvent::KeyboardInput { event, .. } => {
                if let ElementState::Released = event.state { return; }

                let direction = match event.physical_key {
                    Code(KeyCode::KeyS) |
                    Code(KeyCode::ArrowDown) => Direction::Down,

                    Code(KeyCode::KeyW) |
                    Code(KeyCode::ArrowUp) => Direction::Up,

                    Code(KeyCode::KeyD) |
                    Code(KeyCode::ArrowLeft) => Direction::Left,

                    Code(KeyCode::KeyA) |
                    Code(KeyCode::ArrowRight) => Direction::Right,

                    _ => Direction::Other,
                };

                match direction {
                    Direction::Down => {
                        self.game_state.update_screen_color((255, 0, 0));
                    }
                    Direction::Up => {
                        self.game_state.update_screen_color((0, 255, 0));
                    }
                    Direction::Left => {
                        self.game_state.update_screen_color((0, 0, 255));
                    }
                    Direction::Right => {
                        self.game_state.update_screen_color((0, 0, 0));
                    }
                    _ => {
                        self.game_state.update_screen_color((255, 255, 255));
                    }
                }

                let screen_color = self.game_state.screen_color;
                let color = Color::from_rgb(screen_color.0, screen_color.1, screen_color.2);

                self.window
                    .as_ref()
                    .unwrap()
                    .set_border_color(Some(color));

                self.window
                    .as_ref()
                    .unwrap()
                    .request_redraw();

                println!("Result: {:?}", color)
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
