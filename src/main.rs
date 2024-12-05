use env_logger;
use winit::event::WindowEvent;
use winit::window::Window;
use nyte::application::{run_app, Application};
use nyte::renderer::State;

fn main() {
    env_logger::init();

    let mut app = App::default();
    run_app(&mut app);
}

#[derive(Default)]
struct App {
    state: AppState,
}

struct AppState {
    score: i32,
    color: (i16, i16, i16),
    increment: i16,
}

impl Default for AppState {
    fn default() -> Self {
        Self { score: 0, color: (100, 100, 100), increment: 5 }
    }
}

impl Application for App {
    fn init(&mut self, window: &mut Window) {
        println!("App init call");
        window.set_maximized(true);
    }

    fn update(&mut self) {}

    fn render(&mut self, state: &mut State) {
        let color = self.state.color;
        state.render(color).unwrap()
    }

    fn handle_event(&mut self, event: WindowEvent) -> () {
        match event {
            WindowEvent::CursorMoved {..} => {
                let color = self.state.color;
                let increment = self.state.increment;
                let next_color = (color.0 + increment, color.1 + increment, color.2 + increment);

                if next_color.0 == 200 {
                    self.state.increment = -5;
                } else if next_color.0 == 100 {
                    self.state.increment = 5;
                }

                self.state.color = next_color;
            }
            _ => ()
        }
    }
}