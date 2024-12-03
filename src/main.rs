use winit::event::WindowEvent;
use nyte::application::{run_app, Application};
use nyte::window::Window;

fn main() {
    let mut app = App::default();
    run_app(&mut app);
}

#[derive(Default)]
struct App {
    state: AppState,
}

#[derive(Default)]
struct AppState {
    score: i32
}

impl Application for App {
    fn init(&mut self, window: &mut Window) {
        println!("App init call");
        window.set_maximized(true);
    }

    fn update(&mut self) {}

    fn render(&mut self) {}

    fn handle_event(&mut self, event: WindowEvent) -> () {
        println!("App handle event call {:?}", event);
    }
}