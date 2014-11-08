extern crate native;
extern crate rsfml;

use rsfml::window::keyboard;
use rsfml::system;
use rsfml::window;
use rsfml::window::event;
use rsfml::graphics::{RenderWindow, RenderTarget, CircleShape, Color};

use std::fmt;
use std::num;
use std::cmp;

struct Circle {
    radius: f32,
    color: i64,
    position: Position
}
struct Position {
    x: f32,
    y: f32
}

impl Position {
    fn distance(&self, target: Position) -> f32 {
        let distX = self.x - target.x;
        let distY = self.y - target.y;
        return ((distX*distX) + (distY*distY)).sqrt();
    }
}

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}

fn drawCircle(window: &mut RenderWindow, circle: &Circle) -> () {
    // Create a CircleShape
    let mut shape = match CircleShape::new() {
        Some(shape) => shape,
        None       => panic!("Error, cannot create ball")
    };
    shape.set_radius(circle.radius);
    let color = Color {
        red: ((circle.color >> 16) & 0xff) as u8,
        green: ((circle.color >> 8) & 0xff) as u8,
        blue: ((circle.color) & 0xff) as u8,
        alpha: (0xff) as u8
    };
    shape.set_fill_color(&color);
    shape.set_position(&system::Vector2f::new(circle.position.x, circle.position.y));

    window.draw(&shape);
}

fn main () -> () {
    let window_size = 400;
    let fwindow_size = window_size as f32;
    // Create the window of the application
    let mut window = match RenderWindow::new(
        window::VideoMode::new_init(window_size, window_size, 32),
        "SFML Example",
        window::NoStyle,
        &window::ContextSettings::default()) {
        Some(window) => window,
        None => panic!("Cannot create a new Render Window.")
    };

    let mut clock = system::Clock::new();
    let mut position = Position {x:200., y:200.};
    let mut yVelocity = 0.;
    let mut xVelocity = 0.;

    while window.is_open() {
        // Handle events
        for event in window.events() {
            match event {
                event::KeyPressed {code:keyboard::Q, ..} => window.close(),
                event::KeyPressed {code:keyboard::Up, ..} => yVelocity+=1.,
                event::KeyPressed {code:keyboard::Down, ..} => yVelocity-=1.,
                event::KeyPressed {code:keyboard::Left, ..} => xVelocity+=1.,
                event::KeyPressed {code:keyboard::Right, ..} => xVelocity-=1.,
                event::KeyPressed {code:code, ..} => 
                    println!("position is ({},{})", position.x, position.y),
                _             => {/* do nothing */}
            }
        }
        let time = clock.get_elapsed_time().as_milliseconds() as f32;
        if (time < 20.) {
            continue;
        }
        let dy = time * yVelocity;
        let dx = time * xVelocity;
        position.y -= dy / 100.;
        position.x -= dx / 100.;
        position.y = (position.y + fwindow_size) % fwindow_size;
        position.x = (position.x + fwindow_size) % fwindow_size;
        // Clear the window
        window.clear(&Color::new_RGB(0, 0, 0));
        let circle = Circle {
            radius: 10.,
            color: 0xaaaaff,
            position: position
        };
        drawCircle(&mut window, &circle);
        // Display things on screen
        window.display();
        clock.restart();
    }
}
