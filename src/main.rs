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
    let mut window = window::create(1200,700);
    window.set_mouse_cursor_visible(false);
    let mut input = input::InputManager::default();
    let mut state = state::default();
    let display = display::DisplayManager;

    let mut clock = system::Clock::new();

    while window.is_open() {
        if clock.get_elapsed_time().as_milliseconds() < 20 {
            continue;
        }
        input.poll(&mut window);
        state = state::simulate(&state, input);
        display.render(&state, &mut window);
        clock.restart();
    }
}
