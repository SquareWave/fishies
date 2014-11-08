extern crate native;
extern crate rsfml;

use rsfml::system::{Vector2f, Clock};
use rsfml::window::{ContextSettings, VideoMode, event, NoStyle};
use rsfml::graphics::{RenderWindow, RenderTarget, CircleShape, Color};

use std::fmt;
use std::num;

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
    shape.set_position(&Vector2f::new(circle.position.x, circle.position.y));

    window.draw(&shape);
}

fn main () -> () {
    // Create the window of the application
    let mut window = match RenderWindow::new(VideoMode::new_init(1368, 740, 32),
        "SFML Example",
        NoStyle,
        &ContextSettings::default()) {
        Some(window) => window,
        None => panic!("Cannot create a new Render Window.")
    };

    let clock = Clock::new();
    let mut mouse = Position {x:0.,y:0.};

    while window.is_open() {
        // Handle events
        for event in window.events() {
            match event {
                event::MouseMoved { x: x, y: y} => mouse = Position {
                    x:x as f32, 
                    y:y as f32
                },
                event::TextEntered { code: c } => window.close(),
                event::Closed => window.close(),
                _             => {/* do nothing */}
            }
        }
        let time = clock.get_elapsed_time().as_milliseconds() as i64;
        // Clear the window
        window.clear(&Color::new_RGB(0, 0, 0));
        // Draw the shape
        for i in range(0i, 90i) {
            for j in range(0i, 50i) {
                let position = Position {
                    x: (5i + (i*15i)) as f32,
                    y: (5i + (j*15i)) as f32
                };
                let dist = mouse.distance(position) as i64;
                if (i==500 && j==500) {
                    println!("{}",dist);                
                }
                let circle = Circle {
                    radius: 5.,
                    color: (time << 256) * num::pow(dist,1u) as i64,
                    position: position
                };
                drawCircle(&mut window, &circle);
            }
        }
        // Display things on screen
        window.display();
    }
}
