use sfml::system;
use sfml::window::{event,Key};
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
                event::KeyPressed {code:Key::Q, ..} => close_window = true,
                event::KeyPressed {code:Key::P, ..} => self.pause = !self.pause,
                event::KeyPressed {code:Key::R, ..} => self.rewind = true,
                event::KeyReleased {code:Key::R, ..} => self.rewind = false,
                event::KeyPressed {code:Key::F, ..} => self.add_fishy = true,
                event::KeyReleased {code:Key::F, ..} => self.add_fishy = false,
                event::KeyPressed {code:Key::S, ..} => self.add_sharky = true,
                event::KeyPressed {code:Key::I, ..} => self.special = !self.special,
                event::KeyPressed {code:Key::Z, ..} => self.zoom = Zoom::In,
                event::KeyReleased {code:Key::Z, ..} => self.zoom = Zoom::NoZoom,
                event::KeyPressed {code:Key::X, ..} => self.zoom = Zoom::Out,
                event::KeyReleased {code:Key::X, ..} => self.zoom = Zoom::NoZoom,
                event::KeyPressed {code:Key::W, ..} => self.super_speed = true,
                event::KeyReleased {code:Key::W, ..} => self.super_speed = false,
                event::KeyPressed {code:Key::Up, ..} => self.direction.up = true,
                event::KeyPressed {code:Key::Down, ..} => self.direction.down = true,
                event::KeyPressed {code:Key::Left, ..} => self.direction.left = true,
                event::KeyPressed {code:Key::Right, ..} => self.direction.right = true,
                event::KeyReleased {code:Key::Up, ..} => self.direction.up = false,
                event::KeyReleased {code:Key::Down, ..} => self.direction.down = false,
                event::KeyReleased {code:Key::Left, ..} => self.direction.left = false,
                event::KeyReleased {code:Key::Right, ..} => self.direction.right = false,
                event::KeyPressed {..} => println!("{}, {}", self.mouse.x, self.mouse.y),
                _             => {/* do nothing */}
            }
        }

        if close_window {
            window.close();
        }
    }
}