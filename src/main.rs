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

#[derive(Default)]
struct AppState {
    score: i32,
}

impl Application for App {
    fn init(&mut self, window: &mut Window) {
        println!("App init call");
        window.set_maximized(true);
    }

    fn update(&mut self) {}

    fn render(&mut self, state: &mut State) {
        state.render((32, 100, 193)).unwrap()
    }

    fn handle_event(&mut self, event: WindowEvent) -> () {
        if let WindowEvent::CloseRequested = event {
            println!("Exiting with current score {}...", self.state.score);
        }
    }
}