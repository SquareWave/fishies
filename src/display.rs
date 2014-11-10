use state;
use input;
use std::f64::consts;
use rsfml::system;
use rsfml::graphics::{RenderWindow, RenderTarget, Color, Font, Text, View};

pub struct DisplayManager {
    pub center: system::Vector2f
}

impl DisplayManager {
    pub fn render(
        &mut self, 
        state: &Vec<state::GameObject>,
        input: &input::InputManager,
        window: &mut RenderWindow, view: &mut View
        ) -> () {

        let color_0 = Color::new_RGB(0, 67, 88);
        let color_1 = Color::new_RGB(190, 219, 57);
        let color_2 = Color::new_RGB(255, 225, 26);
        let color_3 = Color::new_RGB(253, 116, 0);
        let color_4 = Color::new_RGB(31, 138, 112);

        match input.zoom {
            input::ZoomIn => view.zoom(1.1),
            input::ZoomOut => view.zoom(0.9),
            _ => {}
        }

        window.clear(&color_0);
        let font = match Font::new_from_file("./assets/UbuntuMono-R.ttf") {
            Some(font) => font,
            None => panic!("font wasn't loaded ohnoes!")
        };

        for &mut obj in state.iter() {
            match obj {
                state::FishyObj(f) => {
                    let mut text = match Text::new() {
                        Some(text) => text,
                        None => panic!("text wasn't made ohnoes!")
                    };
                    text.set_font(&font);
                    text.set_string(if f.animation<2 {"i"} else {"j"});
                    let character_size = 24;
                    let display_position = get_display_position(
                        f.position, f.orientation, character_size as f32);
                    text.set_position(&display_position);
                    text.set_rotation(f.orientation + 180.);

                    text.set_character_size(character_size);

                    let mut color = if f.kind == 0 {
                        color_1 
                    } else if f.kind == 1 {
                        color_2
                    } else {
                        color_3
                    };

                    if !f.alive {
                        color.alpha = 128;
                    }

                    text.set_color(&color);

                    window.draw(&text);
                },
                state::SharkyObj(shark) => {
                    let mut text = match Text::new() {
                        Some(text) => text,
                        None => panic!("text wasn't made ohnoes!")
                    };
                    text.set_font(&font);
                    text.set_string(" A \n/O\\\n X ");
                    let character_size = 24;
                    let display_position = get_display_position(
                        shark.position, shark.orientation, character_size as f32);
                    text.set_position(&display_position);
                    text.set_rotation(shark.orientation + 180.);
                    text.set_character_size(character_size);

                    text.set_color(&color_1);

                    window.draw(&text);
                },
                state::BallObj(b) => {
                    view.set_center(&b.position);
                    window.set_view(view);
                    self.center = b.position;
                    let mut text = match Text::new() {
                        Some(text) => text,
                        None => panic!("text wasn't made ohnoes!")
                    };
                    text.set_font(&font);
                    text.set_string(if b.animation<2 {"i"} else {"j"});
                    let character_size = 32;
                    let display_position = get_display_position(
                        b.position, b.orientation, character_size as f32);
                    text.set_position(&display_position);
                    text.set_rotation(b.orientation + 180.);
                    text.set_character_size(character_size);
                    text.set_color(&color_4);

                    window.draw(&text);

                    // let mut circle = match CircleShape::new() {
                    //     Some(circle) => circle,
                    //     None       => panic!("Error, cannot create ball")
                    // };
                    // circle.set_radius(1.);
                    // circle.set_fill_color(&Color::new_RGB(144, 240, 64));
                    // circle.set_position(&b.position);
                    // window.draw(&circle);
                }
            }
        }
        window.display();
    }
}

fn get_display_position(p: system::Vector2f, r : f32, font_size: f32) -> system::Vector2f {
    p + (get_unit_vector(r + 160.) * -0.792 * font_size)
}

fn to_360 (r: f32) -> f32 {
    (r + 360.) % 360.
}

fn get_unit_vector(rotation: f32) -> system::Vector2f {
    let rad = (to_360(rotation) / 180.) * consts::PI as f32;
    let x = -rad.sin();
    let y = rad.cos();
    system::Vector2f { x:x, y:y }
}