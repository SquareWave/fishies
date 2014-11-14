extern crate native;
extern crate libc;
extern crate rsfml;
extern crate playground;

use playground::window;
use playground::state;
use playground::input;
use playground::display;
use libc::{c_void, size_t, malloc, free};
use std::mem;
use std::ptr;

use rsfml::system;

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}

fn main () -> () {
    unsafe {
    // Create the window of the application
        let (mut window, mut view) = window::create(1200,700);
        let mut input = input::InputManager::default();

        let base_state =
            malloc(mem::size_of::<Vec<state::GameObject>>() as size_t)
            as *mut Vec<state::GameObject>;
        assert!(!base_state.is_null());

        ptr::write(&mut *base_state, state::default());

        let mut states = vec![base_state];
        let mut display = display::DisplayManager{
            center: system::Vector2f{x: 600., y: 350.}
        };

        let mut clock = system::Clock::new();

        while window.is_open() {
            input.poll(&mut window);
            if input.rewind {
                let state = if states.len() == 1 {
                    states[0]
                } else {
                    match states.pop() {
                        Some(state) => state,
                        None => base_state
                    }
                };
                display.render(&*state, &input, &mut window, &mut view);
                clock.restart();
                ptr::read(state as *const Vec<state::GameObject>);
                // clean-up our allocation
                if states.len() != 1 {
                    free(state as *mut c_void)
                }
            } else if !input.pause {
                let state =
                    malloc(mem::size_of::<Vec<state::GameObject>>() as size_t)
                    as *mut Vec<state::GameObject>;
                assert!(!state.is_null());

                ptr::write(&mut *state,
                    state::simulate(&*states[states.len() - 1], input));
                states.push(state);
                display.render(&*states[states.len() - 1], &input, &mut window, &mut view);
            } else {
                display.render(&*states[states.len() - 1], &input, &mut window, &mut view);
            }
            clock.restart();
        }

        for &ptr in states.iter() {
            ptr::read(ptr as *const Vec<state::GameObject>);

            // clean-up our allocation
            free(ptr as *mut c_void)
        }
    }
}
