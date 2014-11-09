use rsfml::system;
use rsfml::window::event;
use rsfml::window::keyboard;
use rsfml::graphics::{RenderWindow};

pub struct InputManager {
    pub add_fishy: bool,
    pub inspect: bool,
    pub mouse: system::Vector2i
}

impl InputManager {
    pub fn default() -> InputManager {
        InputManager { mouse: system::Vector2i {x:0, y:0}, add_fishy: false, inspect: false}
    }
    pub fn poll(&mut self, window: &mut RenderWindow) -> () {
        self.add_fishy = false;
        self.inspect = false;
        for event in window.events() {
            match event {
                event::MouseMoved {x, y} => self.mouse = system::Vector2i {x:x, y:y},
                event::KeyPressed {code:keyboard::Q, ..} => window.close(),
                event::KeyPressed {code:keyboard::F, ..} => self.add_fishy = true,
                event::KeyPressed {code:keyboard::I, ..} => self.inspect = true,
                event::KeyPressed {..} => println!("{}, {}", self.mouse.x, self.mouse.y),
                _             => {/* do nothing */}
            }
        }
    }
}