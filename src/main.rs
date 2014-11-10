extern crate native;
extern crate alloc;
extern crate rsfml;
extern crate playground;

use playground::window;
use playground::state;
use playground::input;
use playground::display;

use rsfml::system;

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}

fn main () -> () {
    // Create the window of the application
    let (mut window, mut view) = window::create(1200,700);
    let mut input = input::InputManager::default();
    let mut state = state::default();
    let mut display = display::DisplayManager{
        center: system::Vector2f{x: 600., y: 350.}
    };

    let mut clock = system::Clock::new();

    while window.is_open() {
        // if clock.get_elapsed_time().as_milliseconds() < 20 {
        //     continue;
        // }
        input.poll(&mut window);
        if !input.pause {
            state = state::simulate(&state, input);
        }
        display.render(&state, &input, &mut window, &mut view);
        clock.restart();
    }
}
