extern crate rand;
extern crate sfml;
extern crate fishies;

use fishies::window;
use fishies::state;
use fishies::input;
use fishies::display;

use sfml::system::{Vector2f, Clock,Time,sleep};

fn main () -> () {
    let (mut window, mut view) = window::create(1200, 700);
    let mut input = input::InputManager::default();

    let base_state = state::default();

    let mut states = Vec::with_capacity(1024);
    states.push(base_state.clone());
    let mut display = display::DisplayManager{
        center: Vector2f{x: 600., y: 350.}
    };

    let mut clock = Clock::new();

    let target_duration = Time::with_milliseconds(1000 / 30);

    while window.is_open() {
        let elapsed = clock.get_elapsed_time();
        let to_sleep = target_duration - elapsed;
        sleep(to_sleep);
        clock.restart();

        input.poll(&mut window);
        let mut state = match states.pop() {
            Some(state) => state,
            None => base_state.clone()
        };
        if !input.rewind {
            states.push(state.clone());
            if !input.pause {
                let next = state::simulate(&state, &input);
                states.push(next.clone());
            }
        }
        display.render(&mut state, &input, &mut window, &mut view);
    }
}
