use sfml::system;
use sfml::window::{Event,Key};
use sfml::graphics::{RenderWindow};

pub struct InputManager {
    pub add_fishy: bool,
    pub add_sharky: bool,
    pub special: bool,
    pub super_speed: bool,
    pub pause: bool,
    pub rewind: bool,
    pub mouse: system::Vector2i,
    pub direction: Direction,
    pub zoom: Zoom
}

pub enum Zoom {
    In,
    Out,
    NoZoom
}

pub struct Direction {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool
}

impl InputManager {
    pub fn default() -> InputManager {
        InputManager {
            mouse: system::Vector2i {x:0, y:0}, 
            add_fishy: false,
            add_sharky: false,
            special: false,
            super_speed: false,
            pause: false,
            rewind: false,
            direction: Direction {
                up: false,
                down: false,
                left: false,
                right: false
            },
            zoom: Zoom::NoZoom
        }
    }
    pub fn poll(&mut self, window: &mut RenderWindow) -> () {
        self.add_sharky = false;
        let mut close_window = false;
        for event in window.events() {
            match event {
                //Event::MouseMoved {x, y} => self.mouse = system::Vector2i {x:x, y:y},
                Event::KeyPressed {code:Key::Q, ..} => close_window = true,
                Event::KeyPressed {code:Key::P, ..} => self.pause = !self.pause,
                Event::KeyPressed {code:Key::R, ..} => self.rewind = true,
                Event::KeyReleased {code:Key::R, ..} => self.rewind = false,
                Event::KeyPressed {code:Key::F, ..} => self.add_fishy = true,
                Event::KeyReleased {code:Key::F, ..} => self.add_fishy = false,
                Event::KeyPressed {code:Key::S, ..} => self.add_sharky = true,
                Event::KeyPressed {code:Key::I, ..} => self.special = !self.special,
                Event::KeyPressed {code:Key::Z, ..} => self.zoom = Zoom::In,
                Event::KeyReleased {code:Key::Z, ..} => self.zoom = Zoom::NoZoom,
                Event::KeyPressed {code:Key::X, ..} => self.zoom = Zoom::Out,
                Event::KeyReleased {code:Key::X, ..} => self.zoom = Zoom::NoZoom,
                Event::KeyPressed {code:Key::W, ..} => self.super_speed = true,
                Event::KeyReleased {code:Key::W, ..} => self.super_speed = false,
                Event::KeyPressed {code:Key::Up, ..} => self.direction.up = true,
                Event::KeyPressed {code:Key::Down, ..} => self.direction.down = true,
                Event::KeyPressed {code:Key::Left, ..} => self.direction.left = true,
                Event::KeyPressed {code:Key::Right, ..} => self.direction.right = true,
                Event::KeyReleased {code:Key::Up, ..} => self.direction.up = false,
                Event::KeyReleased {code:Key::Down, ..} => self.direction.down = false,
                Event::KeyReleased {code:Key::Left, ..} => self.direction.left = false,
                Event::KeyReleased {code:Key::Right, ..} => self.direction.right = false,
                Event::KeyPressed {..} => println!("{}, {}", self.mouse.x, self.mouse.y),
                _             => {/* do nothing */}
            }
        }

        if close_window {
            window.close();
        }
    }
}