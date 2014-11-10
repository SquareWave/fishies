use rsfml::window;
use rsfml::system;
use rsfml::graphics::{RenderWindow, RenderTarget, View};

pub fn create(width: uint, height: uint) -> (RenderWindow, View) {
    let (mut w,v) = (
        create_window(width, height), 
        create_view(width as f32, height as f32)
    );
    w.set_view(&v);
    w.set_mouse_cursor_visible(false);
    (w,v)
}

fn create_window(width: uint, height: uint) -> RenderWindow {
    match RenderWindow::new(
        window::VideoMode::new_init(width, height, 32),
        "SFML Example",
        window::NoStyle,
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