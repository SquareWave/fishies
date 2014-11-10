use input;
use std::rand;
use std::cmp;
use std::f64::consts;
use rsfml::system;

pub struct Ball {
    pub position: system::Vector2f,
    pub speed: f32,
    pub orientation: f32,
    pub animation: u8
}

pub struct Fishy {
    pub position: system::Vector2f,
    pub orientation: f32,
    pub velocity: system::Vector2f,
    pub animation: u8,
    pub alive: bool,
    pub kind: u8
}

pub struct Sharky {
    pub position: system::Vector2f,
    pub speed: f32,
    pub orientation: f32,
    pub active: bool,
    pub energy: int
}

pub enum GameObject {
    BallObj (Ball),
    FishyObj (Fishy),
    SharkyObj (Sharky),
}

pub fn default() -> Vec<GameObject> {
    let initial_fishy = Fishy {
        position: system::Vector2f {x: 0., y:0.},
        velocity: system::Vector2f {x: 0., y:0.},
        orientation: 0.,
        alive: true,
        animation: 0,
        kind: 0
    };
    let initial_ball = Ball {
        position: system::Vector2f {x: 0., y:0.},
        speed: 1.,
        orientation: 0.,
        animation: 0
    };
    vec![FishyObj(initial_fishy), BallObj(initial_ball)]
}

pub fn simulate(state: &Vec<GameObject>, input: input::InputManager) -> Vec<GameObject> {
    let mut ball_pos = system::Vector2f {x: 0., y: 0.};
    let mut new_state: Vec<GameObject> = state.iter().map(|&obj| {
        match obj {
            FishyObj(mut fish) => {
                if !fish.alive {
                    fish.velocity = fish.velocity * 0.9;
                    fish.orientation = fish.orientation + 2.;
                    return FishyObj(fish);
                }

                let mut push = system::Vector2f{x:0., y:0.};
                for &obj_inner in state.iter() {
                    match obj_inner {
                        BallObj(ball) => {
                            if input.special {continue;}
                            let target = ball.position - fish.position;
                            let mag = get_magnitude(target);
                            push = push + target / mag * 1. ;
                        },
                        FishyObj(other) => {
                            let dist_inner = fish.position - other.position;
                            let mag_inner = get_magnitude(dist_inner);
                            if mag_inner == 0. {continue;}
                            if !other.alive && mag_inner < 80. {
                                push = push + (dist_inner / mag_inner) * 20.;
                            } else if mag_inner < 17. {
                                push = push + (dist_inner / mag_inner) * 20.;
                            } else if mag_inner < 890. {
                                push = push - (dist_inner / mag_inner) * 0.002;
                            }
                        },
                        SharkyObj(shark) => {
                            let delta = fish.position - shark.position;
                            let mag = get_magnitude(delta);
                            if mag < 12. {
                                fish.alive = false;
                                continue;
                            }
                            if mag < shark.speed * 25. {
                                push = push + (delta / mag) * 200.;
                            }
                        }
                    }
                }
                push = push / get_magnitude(push);
                let target_r = to_360(get_rotation(push));
                let new_r = rotate_toward(fish.orientation, target_r, 6.);
                let unit_vector = get_unit_vector(new_r);

                fish.velocity = unit_vector * 12.;
                fish.position = fish.position + fish.velocity;
                fish.orientation = new_r;
                if fish.animation == 4 {
                    fish.animation = 0;
                } else {
                    fish.animation += 1;
                }
                FishyObj(fish)
            },
            SharkyObj(mut shark) => {
                let mut push = system::Vector2f{x:0., y:0.};
                for &obj_inner in state.iter() {
                    match obj_inner {
                        BallObj(..) => { },
                        FishyObj(other) => {
                            let delta = shark.position - other.position;
                            let mag = get_magnitude(delta);
                            if shark.active || mag > 1048. {
                                push = push - (delta / mag);
                            } else {
                                push = push + (delta / mag);
                            }
                        },
                        SharkyObj(other) => {
                            let delta = shark.position - other.position;
                            let mag = get_magnitude(delta);
                            if mag == 0. {continue;}
                            if mag < 34. {
                                push = push + (delta / mag) * 20.;
                            } else if mag < 84. {
                                push = push - (delta / mag) * 0.25;
                            }  
                        }
                    }
                }
                push = push / get_magnitude(push);
                let target_r = to_360(get_rotation(push));
                let turn_rate = if shark.active {2.} else {0.5};
                let new_r = rotate_toward(shark.orientation, target_r, turn_rate);
                let unit_vector = get_unit_vector(new_r);
                let accel = 2.;
                let on_target = (new_r - target_r).abs() < 30.;
                if shark.energy <= 0 && !on_target {
                    shark.active = false;
                }
                if shark.active {
                    if on_target || shark.speed > 7. {
                        shark.energy -= 4;
                        shark.speed = f_min(shark.speed + accel, 45.);   
                    }
                } else {
                    shark.energy = cmp::min(shark.energy + 1, 1048);
                    shark.speed = f_max(shark.speed - accel / 4., 7.);
                    if rand::random::<int>() % 256 == 0 {
                        shark.active = true;
                    }
                }
                shark.position = shark.position + unit_vector * shark.speed;
                shark.orientation = new_r;
                SharkyObj(shark)
            },
            BallObj(mut ball) => {
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
                let velocity = get_unit_vector(ball.orientation) * ball.speed;
                ball.position = ball.position + velocity;
                if ball.animation == 4 {
                    ball.animation = 0;
                } else {
                    ball.animation += 1;
                }
                BallObj(ball)
                // let push = 25.;
                // let diag = 1. / consts::SQRT2 as f32;
                // let mut vec = system::Vector2f{x:0.,y:0.};
                // if input.direction.up { vec.y -= push };
                // if input.direction.down { vec.y += push };
                // if input.direction.right { vec.x += push };
                // if input.direction.left { vec.x -= push };
                // if vec.x != 0. { vec.y *= diag };
                // if vec.y != 0. { vec.x *= diag };
                // ball.position = ball.position + vec;
                // BallObj(ball)
            }
        }
    }).collect();
    if input.add_fishy {
        let fishy = Fishy {
            position: ball_pos,
            orientation: 0.,
            velocity: system::Vector2f {x: 0., y:0.},
            alive: true,
            animation: 0,
            kind: rand::random::<u8>() % 3
        };
        new_state.push(FishyObj(fishy));       
    };
    if input.add_sharky {
        let sharky = Sharky {
            position: system::Vector2f {x: 200., y:200.},
            orientation: 2.5,
            active: false,
            speed: 0.,
            energy: 0
        };
        new_state.push(SharkyObj(sharky));       
    };
    new_state
}

fn get_rotation(vec: system::Vector2f) -> f32 {
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

fn get_magnitude(vec: system::Vector2f) -> f32 {
    (vec.x*vec.x + vec.y*vec.y).sqrt()
}

fn get_unit_vector(rotation: f32) -> system::Vector2f {
    let rad = (to_360(rotation) / 180.) * consts::PI as f32;
    let x = -rad.sin();
    let y = rad.cos();
    system::Vector2f { x:x, y:y }
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
    let mut new_r = r;
    while new_r < 0. {
        new_r += 360.;
    }
    new_r % 360.
}

fn is_clockwise(cur: f32, target: f32) -> bool {
    let n_cur = to_360(cur);
    let n_target = to_360(target);
    ((n_target > n_cur) && (n_cur + 180. > n_target)) ||
    ((n_target < n_cur) && (n_cur - 180. > n_target))
}