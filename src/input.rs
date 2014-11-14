use rsfml::system;
use rsfml::window::event;
use rsfml::window::keyboard;
use rsfml::graphics::{RenderWindow};

pub struct InputManager {
    pub add_fishy: bool,
    pub add_sharky: bool,
    pub special: bool,
    pub pause: bool,
    pub rewind: bool,
    pub mouse: system::Vector2i,
    pub direction: Direction,
    pub zoom: Zoom
}

pub enum Zoom {
    ZoomIn,
    ZoomOut,
    DontZoom
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
            pause: false,
            rewind: false,
            direction: Direction {
                up: false,
                down: false,
                left: false,
                right: false
            },
            zoom: DontZoom
        }
    }
    pub fn poll(&mut self, window: &mut RenderWindow) -> () {
        self.add_sharky = false;
        for event in window.events() {
            match event {
                //event::MouseMoved {x, y} => self.mouse = system::Vector2i {x:x, y:y},
                event::KeyPressed {code:keyboard::Q, ..} => window.close(),
                event::KeyPressed {code:keyboard::P, ..} => self.pause = !self.pause,
                event::KeyPressed {code:keyboard::R, ..} => self.rewind = true,
                event::KeyReleased {code:keyboard::R, ..} => self.rewind = false,
                event::KeyPressed {code:keyboard::F, ..} => self.add_fishy = true,
                event::KeyReleased {code:keyboard::F, ..} => self.add_fishy = false,
                event::KeyPressed {code:keyboard::S, ..} => self.add_sharky = true,
                event::KeyPressed {code:keyboard::I, ..} => self.special = !self.special,
                event::KeyPressed {code:keyboard::Z, ..} => self.zoom = ZoomIn,
                event::KeyReleased {code:keyboard::Z, ..} => self.zoom = DontZoom,
                event::KeyPressed {code:keyboard::X, ..} => self.zoom = ZoomOut,
                event::KeyReleased {code:keyboard::X, ..} => self.zoom = DontZoom,
                event::KeyPressed {code:keyboard::Up, ..} => self.direction.up = true,
                event::KeyPressed {code:keyboard::Down, ..} => self.direction.down = true,
                event::KeyPressed {code:keyboard::Left, ..} => self.direction.left = true,
                event::KeyPressed {code:keyboard::Right, ..} => self.direction.right = true,
                event::KeyReleased {code:keyboard::Up, ..} => self.direction.up = false,
                event::KeyReleased {code:keyboard::Down, ..} => self.direction.down = false,
                event::KeyReleased {code:keyboard::Left, ..} => self.direction.left = false,
                event::KeyReleased {code:keyboard::Right, ..} => self.direction.right = false,
                event::KeyPressed {..} => println!("{}, {}", self.mouse.x, self.mouse.y),
                _             => {/* do nothing */}
            }
        }
    }
}