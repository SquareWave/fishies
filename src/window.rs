use sfml::window;
use sfml::window::{window_style};
use sfml::system;
use sfml::graphics::{RenderWindow, RenderTarget, View};

pub fn create(width: u32, height: u32) -> (RenderWindow, View) {
    let (mut w,v) = (
        create_window(width, height), 
        create_view(width as f32, height as f32)
    );
    w.set_view(&v);
    w.set_mouse_cursor_visible(false);
    (w,v)
}

fn create_window(width: u32, height: u32) -> RenderWindow {
    match RenderWindow::new(
        window::VideoMode::new_init(width, height, 32),
        "Fishies",
        window_style::CLOSE,
        &window::ContextSettings::default()) {
        Some(window) => window,
        None => panic!("Too foggy!")
    }
}

fn create_view(width: f32, height: f32) -> View {
    match View::new_init(&system::Vector2f {
        x: width / 2.,
        y: height / 2.
    }, &system::Vector2f {
        x: width,
        y: height
    }) {
        Some(view) => view,
        None => panic!("I can't see!")
    }
}