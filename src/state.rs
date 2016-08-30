extern crate rand;

use input;
use std::cmp;
use std::f64::consts;
use sfml::system;

// static BOUNDS_X: f32 = 1000.;
// static BOUNDS_Y: f32 = 1000.;

pub type Vector = system::Vector2f;

#[derive(Clone)]
pub struct Ball {
    pub position: Vector,
    pub speed: f32,
    pub orientation: f32,
    pub animation: u8
}

#[derive(Clone)]
pub struct Fishy {
    pub position: Vector,
    pub velocity: Vector,
    pub tilt: Vector,
    pub orientation: f32,
    pub animation: u8,
    pub alive: bool,
    pub kind: u8,
}

#[derive(Clone)]
pub struct Drifter {
    pub position: Vector,
    pub velocity: Vector,
    pub size: f32,
    pub fill: f32
}

#[derive(Clone)]
pub struct Sharky {
    pub position: Vector,
    pub speed: f32,
    pub orientation: f32,
    pub active: bool,
    pub energy: i32
}

#[derive(Clone)]
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
    let mut new_state: Vec<GameObject> = state.iter().map(|ref obj| {
        match obj.clone().clone() {

            // fishies want to get closer to other fishies but not too close,
            // avoid sharks based on how fast the sharks are going, and follow
            // the user ("Ball") if it's close
            GameObject::Fishy(mut fish) => {

                if !fish.alive {
                    fish.velocity = fish.velocity * 0.99;
                    fish.orientation = fish.orientation + 2.;
                    return GameObject::Fishy(fish);
                }

                let leadership = 0.25;
                let death_smell = 80.;
                let fear_of_the_dead = 20.;
                let personal_space = 17.;
                let presence = 40.;
                let fear_of_intimacy = 20.;
                let lovey_dovey = 0.002;
                let sight_range = 800.;
                let attraction_range = 300.;
                let alignment_range = 200.;
                let death_zone = 12.;
                let fishy_sense = 25.;
                let fear_of_death = 200.;
                let turn_rate = 12.;
                let speed = 12.;
                let tilt_factor = 0.05;

                let mut push = zero_vector();
                let tilt_decider = rand::random::<i32>() % 12000;
                if tilt_decider == 1 {
                    fish.tilt = get_unit_vector((rand::random::<u32>() % 360) as f32);
                } else if tilt_decider / 4 == 0 {
                    fish.tilt = zero_vector();
                }
                push = push + fish.tilt * tilt_factor;
                for ref obj_inner in state.iter() {
                    match obj_inner {
                        &&GameObject::Ball(ref ball) => {
                            if input.special {continue;}
                            let target = fish.position - ball.position;
                            let mag = get_magnitude(target);
                            if mag > presence && mag < sight_range {
                                push = push - (target / mag) * leadership;
                            }
                        },
                        &&GameObject::Fishy(ref other) => {
                            let target = fish.position - other.position;
                            let mag = get_magnitude(target);
                            if mag == 0. {continue;}
                            if !other.alive && mag < death_smell {
                                push = push + (target / mag) * fear_of_the_dead;
                            } else if mag < personal_space {
                                push = push + (target / mag) * fear_of_intimacy;
                            } else if mag < alignment_range {
                                push = push + other.tilt * tilt_factor;
                                push = push + get_unit_vector(other.orientation) * lovey_dovey;
                            } else if mag < attraction_range {
                                push = push - (target / mag) * lovey_dovey;
                            }
                        },
                        &&GameObject::Sharky(ref shark) => {
                            let delta = fish.position - shark.position;
                            let mag = get_magnitude(delta);
                            if mag < death_zone {
                                fish.alive = false;
                                continue;
                            }
                            if mag < shark.speed * fishy_sense {
                                push = push + (delta / mag) * fear_of_death;
                            }
                        },
                        _ => {}

                    }
                }
                push = push / get_magnitude(push);
                //push = bound(fish.position, push);
                let target_r = to_360(get_rotation(push));
                let new_r = rotate_toward(fish.orientation, target_r, turn_rate);
                let unit_vector = get_unit_vector(new_r);

                fish.velocity = unit_vector * speed;
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
                let stalking_distance = 1024.;
                let smell_distance = 4096.;
                let mob_mentality = 512.;
                let bandwagon_appeal = 20.;
                let personal_space = 200.;
                let target_range = 30.;
                let active_turn_rate = 2.;
                let inactive_turn_rate = 0.5;
                let accel = 2.;
                let decel = accel / 4.;
                let minimum_speed = 7.;
                let maximum_speed = 45.;
                let energy_decay = 4;
                let max_energy = 1024;
                let heres_johnny = 256;

                let mut fish_in_range = false;

                for ref obj_inner in state.iter() {
                    match obj_inner {
                        &&GameObject::Ball(..) => { },
                        &&GameObject::Fishy(ref other) => {
                            let delta = shark.position - other.position;
                            let mag = get_magnitude(delta);
                            if shark.active || mag > stalking_distance {
                                push = push - (delta / mag);
                            } else {
                                push = push + (delta / mag);
                            }

                            if mag < smell_distance {
                                fish_in_range = true;
                            }
                        },
                        &&GameObject::Sharky(ref other) => {
                            let delta = shark.position - other.position;
                            let mag = get_magnitude(delta);
                            if mag == 0. {continue;}
                            if mag < personal_space {
                                push = push + (delta / mag) * personal_space;
                            } else if mag < mob_mentality {
                                if other.active && shark.energy > 0 {
                                    shark.active = true;
                                }
                                push = push - (delta / mag) * bandwagon_appeal;
                            }  
                        },
                        _ => {}

                    }
                }
                push = push / get_magnitude(push);
                //push = bound(shark.position, push);
                let target_r = to_360(get_rotation(push));
                let turn_rate = if shark.active {
                    active_turn_rate
                } else {
                    inactive_turn_rate
                };
                let new_r = rotate_toward(shark.orientation, target_r, turn_rate);
                let unit_vector = get_unit_vector(new_r);
                let on_target = (new_r - target_r).abs() < target_range;
                if shark.energy <= 0 && !on_target {
                    shark.active = false;
                }
                if fish_in_range && shark.active {
                    if on_target || shark.speed > minimum_speed {
                        shark.energy -= energy_decay;
                        shark.speed = f_min(shark.speed + accel, maximum_speed);   
                    }
                } else {
                    shark.energy = cmp::min(shark.energy + 1, max_energy);
                    shark.speed = f_max(shark.speed - decel, minimum_speed);
                    if rand::random::<i32>() % heres_johnny == 0 {
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
                let accel = 0.5;
                let speed_cap = 15.;
                let turn_rate = 6.;
                if input.direction.left { ball.orientation -= turn_rate };
                if input.direction.right { ball.orientation += turn_rate };
                if input.direction.up { 
                    ball.speed = f_min(ball.speed + accel, speed_cap);
                };
                if input.direction.down {
                    ball.speed = f_max(ball.speed - accel, 0.); 
                }
                if input.super_speed {
                    ball.speed = 2. * speed_cap;
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

// fn bound(pos: Vector, push: Vector) -> Vector {
//     let mut bounded = push;
//     bounded.x = if pos.x > BOUNDS_X {
//         -1.
//     } else if pos.x < -BOUNDS_X {
//         1.
//     } else {
//         push.x
//     };

//     bounded.y = if pos.y > BOUNDS_Y {
//         -1.
//     } else if pos.y < -BOUNDS_Y {
//         1.
//     } else {
//         push.y
//     };

//     bounded
//}

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
