use input;
use std::rand;
use std::cmp;
use std::f64::consts;
use rsfml::system;

#[deriving(Clone)]
pub struct Ball {
    pub position: system::Vector2f,
    pub speed: f32,
    pub orientation: f32,
    pub animation: u8
}

#[deriving(Clone)]
pub struct Fishy {
    pub position: system::Vector2f,
    pub orientation: f32,
    pub velocity: system::Vector2f,
    pub animation: u8,
    pub alive: bool,
    pub kind: u8
}

#[deriving(Clone)]
pub struct Sharky {
    pub position: system::Vector2f,
    pub speed: f32,
    pub orientation: f32,
    pub active: bool,
    pub energy: int
}

#[deriving(Clone)]
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
                    fish.velocity = fish.velocity * 0.99;
                    fish.orientation = fish.orientation + 2.;
                    return FishyObj(fish);
                }

                let leadership = 1.;
                let death_smell = 80.;
                let fear_of_the_dead = 20.;
                let personal_space = 17.;
                let presence = 40.;
                let fear_of_intimacy = 20.;
                let lovey_dovey = 0.002;
                let sight_range = 890.;
                let death_zone = 12.;
                let fishy_sense = 25.;
                let fear_of_death = 200.;
                let turn_rate = 6.;
                let speed = 12.;

                let mut push = system::Vector2f{x:0., y:0.};
                for &obj_inner in state.iter() {
                    match obj_inner {
                        BallObj(ball) => {
                            if input.special {continue;}
                            let target = fish.position - ball.position;
                            let mag = get_magnitude(target);
                            if mag > presence && mag < sight_range {
                                push = push - (target / mag) * leadership;
                            }
                        },
                        FishyObj(other) => {
                            let target = fish.position - other.position;
                            let mag = get_magnitude(target);
                            if mag == 0. {continue;}
                            if !other.alive && mag < death_smell {
                                push = push + (target / mag) * fear_of_the_dead;
                            } else if mag < personal_space {
                                push = push + (target / mag) * fear_of_intimacy;
                            } else if mag < sight_range {
                                push = push - (target / mag) * lovey_dovey;
                            }
                        },
                        SharkyObj(shark) => {
                            let delta = fish.position - shark.position;
                            let mag = get_magnitude(delta);
                            if mag < death_zone {
                                fish.alive = false;
                                continue;
                            }
                            if mag < shark.speed * fishy_sense {
                                push = push + (delta / mag) * fear_of_death;
                            }
                        }
                    }
                }
                push = push / get_magnitude(push);
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
                FishyObj(fish)
            },


            SharkyObj(mut shark) => {
                let mut push = system::Vector2f{x:0., y:0.};
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

                for &obj_inner in state.iter() {
                    match obj_inner {
                        BallObj(..) => { },
                        FishyObj(other) => {
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
                        SharkyObj(other) => {
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
                        }
                    }
                }
                push = push / get_magnitude(push);
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
                    if rand::random::<int>() % heres_johnny == 0 {
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