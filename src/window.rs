use rsfml::window;
use rsfml::graphics::{RenderWindow};

pub fn create(width: uint, height: uint) -> RenderWindow {
    match RenderWindow::new(
        window::VideoMode::new_init(width, height, 32),
        "SFML Example",
        window::NoStyle,
        &window::ContextSettings::default()) {
        Some(window) => window,
        None => panic!("Cannot create a new Render Window.")
    }
}
