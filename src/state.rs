extern crate rand;

use input;
use std::cmp;
use std::f64::consts;
use sfml::system;

// static BOUNDS_X: f32 = 1000.;
// static BOUNDS_Y: f32 = 1000.;

pub type Vector = system::Vector2f;

#[derive(Copy, Clone)]
pub struct Ball {
    pub position: Vector,
    pub speed: f32,
    pub orientation: f32,
    pub animation: u8
}

#[derive(Copy, Clone)]
pub struct Fishy {
    pub position: Vector,
    pub velocity: Vector,
    pub tilt: Vector,
    pub orientation: f32,
    pub animation: u8,
    pub alive: bool,
    pub kind: u8,
}

#[derive(Copy, Clone)]
pub struct Drifter {
    pub position: Vector,
    pub velocity: Vector,
    pub size: f32,
    pub fill: f32
}

#[derive(Copy, Clone)]
pub struct Sharky {
    pub position: Vector,
    pub speed: f32,
    pub orientation: f32,
    pub active: bool,
    pub energy: i32
}

#[derive(Copy, Clone)]
pub enum GameObject {
    Ball (Ball),
    Fishy (Fishy),
    Sharky (Sharky),
    Drifter (Drifter),
}

pub fn default() -> Vec<GameObject> {
    let initial_fishy = Fishy {
        position: zero_vector(),
        velocity: zero_vector(),
        tilt: zero_vector(),
        orientation: 0.,
        alive: true,
        animation: 0,
        kind: 0
    };
    let initial_ball = Ball {
        position: zero_vector(),
        speed: 1.,
        orientation: 0.,
        animation: 0
    };
    let mut result = vec![
        GameObject::Fishy(initial_fishy),
        GameObject::Ball(initial_ball)
    ];

    for _ in 0..50 {
        let drifter = Drifter {
            position: Vector {
                x: rand::random::<i8>() as f32, 
                y: rand::random::<i8>() as f32
            },
            size: rand::random::<i8>() as f32,
            velocity: Vector {
                x: rand::random::<i8>() as f32 / 128., 
                y: rand::random::<i8>() as f32 / 128.
            },
            fill: rand::random::<u8>() as f32 / 255.
        };
        result.push(GameObject::Drifter(drifter))
    }

    result
}

// this function is a nightmare but I actually think it's the best option right now
// to clump the whole thing together
pub fn simulate(state: &Vec<GameObject>, 
    input: &input::InputManager) -> Vec<GameObject> {
    let mut ball_pos = zero_vector();
    let mut new_state: Vec<GameObject> = state.iter().map(|&obj| {
        match obj {

            // fishies want to get closer to other fishies but not too close,
            // avoid sharks based on how fast the sharks are going, and follow
            // the user ("Ball") if it's close
            GameObject::Fishy(mut fish) => {

                if !fish.alive {
                    fish.velocity = fish.velocity * 0.99;
                    fish.orientation = fish.orientation + 2.;
                    return GameObject::Fishy(fish);
                }

                const LEADERSHIP : f32 = 0.25;
                const DEATH_SMELL : f32 = 80.;
                const FEAR_OF_THE_DEAD : f32 = 20.;
                const PERSONAL_SPACE : f32 = 17.;
                const PRESENCE : f32 = 40.;
                const FEAR_OF_INTIMACY : f32 = 20.;
                const LOVEY_DOVEY : f32 = 0.002;
                const SIGHT_RANGE : f32 = 800.;
                const ATTRACTION_RANGE : f32 = 300.;
                const ALIGNMENT_RANGE : f32 = 200.;
                const DEATH_ZONE : f32 = 12.;
                const FISHY_SENSE : f32 = 25.;
                const FEAR_OF_DEATH : f32 = 200.;
                const TURN_RATE : f32 = 12.;
                const SPEED : f32 = 12.;
                const TILT_FACTOR : f32 = 0.05;

                let mut push = zero_vector();
                let tilt_decider = rand::random::<i32>() % 12000;
                if tilt_decider == 1 {
                    fish.tilt = get_unit_vector((rand::random::<u32>() % 360) as f32);
                } else if tilt_decider / 4 == 0 {
                    fish.tilt = zero_vector();
                }
                push = push + fish.tilt * TILT_FACTOR;
                for ref obj_inner in state.iter() {
                    match obj_inner {
                        &&GameObject::Ball(ref ball) => {
                            if input.special {continue;}
                            let target = fish.position - ball.position;
                            let mag = get_magnitude(target);
                            if mag > PRESENCE && mag < SIGHT_RANGE {
                                push = push - (target / mag) * LEADERSHIP;
                            }
                        },
                        &&GameObject::Fishy(ref other) => {
                            let target = fish.position - other.position;
                            let mag = get_magnitude(target);
                            if mag == 0. {continue;}
                            if !other.alive && mag < DEATH_SMELL {
                                push = push + (target / mag) * FEAR_OF_THE_DEAD;
                            } else if mag < PERSONAL_SPACE {
                                push = push + (target / mag) * FEAR_OF_INTIMACY;
                            } else if mag < ALIGNMENT_RANGE {
                                push = push + other.tilt * TILT_FACTOR;
                                push = push + get_unit_vector(other.orientation) * LOVEY_DOVEY;
                            } else if mag < ATTRACTION_RANGE {
                                push = push - (target / mag) * LOVEY_DOVEY;
                            }
                        },
                        &&GameObject::Sharky(ref shark) => {
                            let delta = fish.position - shark.position;
                            let mag = get_magnitude(delta);
                            if mag < DEATH_ZONE {
                                fish.alive = false;
                                continue;
                            }
                            if mag < shark.speed * FISHY_SENSE {
                                push = push + (delta / mag) * FEAR_OF_DEATH;
                            }
                        },
                        _ => {}

                    }
                }
                push = push / get_magnitude(push);
                //push = bound(fish.position, push);
                let target_r = to_360(get_rotation(push));
                let new_r = rotate_toward(fish.orientation, target_r, TURN_RATE);
                let unit_vector = get_unit_vector(new_r);

                fish.velocity = unit_vector * SPEED;
                fish.position = fish.position + fish.velocity;
                fish.orientation = new_r;
                if fish.animation == 4 {
                    fish.animation = 0;
                } else {
                    fish.animation += 1;
                }
                GameObject::Fishy(fish)
            },

            // sharkies want to keep their distance from the fishies but still stalk
            // them, until they decide to charge, at which point they accelerate
            // rapidly toward the fishies
            GameObject::Sharky(mut shark) => {
                let mut push = zero_vector();
                const STALKING_DISTANCE : f32 = 1024.;
                const SMELL_DISTANCE : f32 = 4096.;
                const MOB_MENTALITY : f32 = 512.;
                const BANDWAGON_APPEAL : f32 = 20.;
                const PERSONAL_SPACE : f32 = 200.;
                const TARGET_RANGE : f32 = 30.;
                const ACTIVE_TURN_RATE : f32 = 2.;
                const INACTIVE_TURN_RATE : f32 = 0.5;
                const ACCEL : f32 = 2.;
                const DECEL : f32 = ACCEL / 4.;
                const MINIMUM_SPEED : f32 = 7.;
                const MAXIMUM_SPEED : f32 = 45.;
                const ENERGY_DECAY : i32 = 4;
                const MAX_ENERGY : i32 = 1024;
                const HERES_JOHNNY : i32 = 256;

                let mut fish_in_range = false;

                for obj_inner in state.iter() {
                    match obj_inner {
                        &GameObject::Ball(..) => { },
                        &GameObject::Fishy(ref other) => {
                            let delta = shark.position - other.position;
                            let mag = get_magnitude(delta);
                            if shark.active || mag > STALKING_DISTANCE {
                                push = push - (delta / mag);
                            } else {
                                push = push + (delta / mag);
                            }

                            if mag < SMELL_DISTANCE {
                                fish_in_range = true;
                            }
                        },
                        &GameObject::Sharky(ref other) => {
                            let delta = shark.position - other.position;
                            let mag = get_magnitude(delta);
                            if mag == 0. {continue;}
                            if mag < PERSONAL_SPACE {
                                push = push + (delta / mag) * PERSONAL_SPACE;
                            } else if mag < MOB_MENTALITY {
                                if other.active && shark.energy > 0 {
                                    shark.active = true;
                                }
                                push = push - (delta / mag) * BANDWAGON_APPEAL;
                            }  
                        },
                        _ => {}

                    }
                }
                push = push / get_magnitude(push);
                //push = bound(shark.position, push);
                let target_r = to_360(get_rotation(push));
                let turn_rate = if shark.active {
                    ACTIVE_TURN_RATE
                } else {
                    INACTIVE_TURN_RATE
                };
                let new_r = rotate_toward(shark.orientation, target_r, turn_rate);
                let unit_vector = get_unit_vector(new_r);
                let on_target = (new_r - target_r).abs() < TARGET_RANGE;
                if shark.energy <= 0 && !on_target {
                    shark.active = false;
                }
                if fish_in_range && shark.active {
                    if on_target || shark.speed > MINIMUM_SPEED {
                        shark.energy -= ENERGY_DECAY;
                        shark.speed = f_min(shark.speed + ACCEL, MAXIMUM_SPEED);   
                    }
                } else {
                    shark.energy = cmp::min(shark.energy + 1, MAX_ENERGY);
                    shark.speed = f_max(shark.speed - DECEL, MINIMUM_SPEED);
                    if rand::random::<i32>() % HERES_JOHNNY == 0 {
                        shark.active = true;
                    }
                }
                shark.position = shark.position + unit_vector * shark.speed;
                shark.orientation = new_r;
                GameObject::Sharky(shark)
            },

            GameObject::Drifter(mut drifter) => {
                drifter.position = drifter.position + drifter.velocity;
                GameObject::Drifter(drifter)
            },

            // this is the user. it used to be more ball-like. probably needs a
            // name change
            GameObject::Ball(mut ball) => {
                ball_pos = ball.position;
                const ACCEL : f32 = 0.5;
                const SPEED_CAP : f32 = 15.;
                const TURN_RATE : f32 = 6.;
                if input.direction.left { ball.orientation -= TURN_RATE };
                if input.direction.right { ball.orientation += TURN_RATE };
                if input.direction.up { 
                    ball.speed = f_min(ball.speed + ACCEL, SPEED_CAP);
                };
                if input.direction.down {
                    ball.speed = f_max(ball.speed - ACCEL, 0.); 
                }
                if input.super_speed {
                    ball.speed = 2. * SPEED_CAP;
                }
                let velocity = get_unit_vector(ball.orientation) * ball.speed;
                ball.position = ball.position + velocity;
                if ball.animation == 4 {
                    ball.animation = 0;
                } else {
                    ball.animation += 1;
                }
                GameObject::Ball(ball)
            }
        }
    }).collect();
    if input.add_fishy {
        let fishy = Fishy {
            position: Vector {
                x: ball_pos.x + rand::random::<i8>() as f32, 
                y: ball_pos.y + rand::random::<i8>() as f32
            },
            velocity: zero_vector(),
            tilt: zero_vector(),
            orientation: 0.,
            alive: true,
            animation: 0,
            kind: rand::random::<u8>() % 3
        };
        new_state.push(GameObject::Fishy(fishy));       
    };
    if input.add_sharky {
        let sharky = Sharky {
            position: zero_vector(),
            orientation: 2.5,
            active: false,
            speed: 0.,
            energy: 0
        };
        new_state.push(GameObject::Sharky(sharky));       
    };
    new_state
}

fn get_rotation(vec: Vector) -> f32 {
    if vec.y > 0. {
        (vec.x / vec.y).atan() * (-360. / (2. * consts::PI as f32))
    } else if vec.y < 0. {
        ((vec.x / vec.y).atan() * (-360. / (2. * consts::PI as f32))) - 180.
    } else if vec.x < 0. {
        90.
    } else {
        270.
    }
}

fn zero_vector() -> Vector {
    Vector{x: 0., y: 0.}
}

fn get_magnitude(vec: Vector) -> f32 {
    (vec.x*vec.x + vec.y*vec.y).sqrt()
}

fn get_unit_vector(rotation: f32) -> Vector {
    let rad = (to_360(rotation) / 180.) * consts::PI as f32;
    let x = -rad.sin();
    let y = rad.cos();
    Vector { x:x, y:y }
}

fn f_max(a: f32, b:f32) -> f32 {
    if a > b {a} else {b}
}

fn f_min(a: f32, b:f32) -> f32 {
    if a < b {a} else {b}
}

fn rotate_toward(cur: f32, target: f32, speed: f32) -> f32 {
    if is_clockwise(cur, target) {
        if is_clockwise(cur + speed, target) {  cur + speed } else { target }
    } else {
        if !is_clockwise(cur - speed, target) {  cur - speed } else { target }
    }
}

fn to_360 (r: f32) -> f32 {
    if r < 0. {
        360. - ((-r) % 360.) 
    } else {
        r % 360.
    }
}

fn is_clockwise(cur: f32, target: f32) -> bool {
    let n_cur = to_360(cur);
    let n_target = to_360(target);
    ((n_target > n_cur) && (n_cur + 180. > n_target)) ||
    ((n_target < n_cur) && (n_cur - 180. > n_target))
}
