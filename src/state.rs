use input;
use std::f64::consts;
use rsfml::system;
use rsfml::system::{ToVec};

pub struct Ball {
    pub position: system::Vector2f
}

pub struct Fishy {
    pub display_position: system::Vector2f,
    pub position: system::Vector2f,
    pub orientation: f32,
    pub animation: u8,
}

pub enum GameObject {
    BallObj (Ball),
    FishyObj (Fishy)
}

pub fn default() -> Vec<GameObject> {
    let initial_fishy = Fishy {
        display_position: system::Vector2f {x: 200., y:200.},
        position: system::Vector2f {x: 200., y:200.},
        orientation: 2.5,
        animation: 0,
    };
    let initial_ball = Ball {
        position: system::Vector2f {x: 200., y:200.}
    };
    vec![FishyObj(initial_fishy), BallObj(initial_ball)]
}

pub fn simulate(state: &Vec<GameObject>, input: input::InputManager) -> Vec<GameObject> {
    let mut new_state: Vec<GameObject> = state.iter().map(|&obj| {
        match obj {
            FishyObj(mut fish) => {
                let target_r = to_360(get_rotation(input.mouse.to_vector2f() - fish.position));
                let new_r = rotate(fish.orientation, target_r);
                let unit_vector = get_unit_vector(new_r);
                if input.inspect {println!("{} : ({},{})",new_r,unit_vector.x, unit_vector.y)}

                let mut push = unit_vector;
                for obj_inner in state.iter() {
                    match obj_inner {
                        &FishyObj(ref other) => {
                            let dist_inner = fish.position - other.position;
                            let mag_inner = get_magnitude(dist_inner);
                            if mag_inner == 0. {continue;}
                            if mag_inner < 34. {
                                let new_unit = unit_vector + (dist_inner / mag_inner) * 0.3;
                                push = new_unit / get_magnitude(new_unit);
                            }
                        },
                        _ => {}
                    }
                }
                fish.position = fish.position + push * 10.;
                fish.display_position = fish.position + (unit_vector * 28.);
                fish.orientation = new_r;
                if fish.animation == 4 {
                    fish.animation = 0;
                } else {
                    fish.animation += 1;
                }
                FishyObj(fish)
            },
            BallObj(_) => BallObj(Ball{position:input.mouse.to_vector2f()})
        }
    }).collect();
    if input.add_fishy {
        let fishy = Fishy {
            display_position: system::Vector2f {x: 200., y:200.},
            position: system::Vector2f {x: 200., y:200.},
            orientation: 2.5,
            animation: 0,
        };
        new_state.push(FishyObj(fishy));       
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

fn rotate(cur: f32, target: f32) -> f32 {
    if is_clockwise(cur, target) {
        if is_clockwise(cur + 6., target) {  cur + 6. } else { target }
    } else {
        if !is_clockwise(cur - 6., target) {  cur - 6. } else { target }
    }
}

fn to_360 (r: f32) -> f32 {
    (r + 360.) % 360.
}

fn is_clockwise(cur: f32, target: f32) -> bool {
    let n_cur = to_360(cur);
    let n_target = to_360(target);
    ((n_target > n_cur) && (n_cur + 180. > n_target)) ||
    ((n_target < n_cur) && (n_cur - 180. > n_target))
}