use state;
use rsfml::graphics::{RenderWindow, RenderTarget, Color, Font, Text};

pub struct DisplayManager;

impl DisplayManager {
    pub fn render(&self, state: &Vec<state::GameObject>, window: &mut RenderWindow) -> () {
        window.clear(&Color::new_RGB(100, 144, 160));
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
                    text.set_position(&f.position);
                    text.set_rotation(f.orientation + 180.);
                    text.set_character_size(48);
                    text.set_color(&Color::new_RGB(240, 144, 64));

                    window.draw(&text);
                },
                state::BallObj(b) => {
                    let mut text = match Text::new() {
                        Some(text) => text,
                        None => panic!("text wasn't made ohnoes!")
                    };
                    text.set_font(&font);
                    text.set_string("o");
                    text.set_position(&b.position);
                    text.set_character_size(48);
                    text.set_color(&Color::new_RGB(144, 240, 64));

                    window.draw(&text);

                    // let mut circle = match CircleShape::new() {
                    //     Some(circle) => circle,
                    //     None       => panic!("Error, cannot create ball")
                    // };
                    // circle.set_radius(16.);
                    // circle.set_fill_color(&Color::new_RGB(144, 240, 64));
                    // circle.set_position(&p);
                    // window.draw(&circle);      
                }
            }
        }
        window.display();
    }
}