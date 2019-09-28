use crate::structs::*;

pub fn update(
    dt: f64,
    positions: &mut Vec<Option<Position>>,
    velocities: &mut Vec<Option<Velocity>>,
    forces: &mut Vec<Option<Force>>,
    fixed: &Vec<Option<Fixed>>,
) {
    for i in 0..positions.len() {
        if let None = fixed[i] {
            if let Some(pos) = &mut positions[i] {
                if let Some(vel) = &mut velocities[i] {
                    if let Some(force) = &mut forces[i] {
                        vel.x = calc_velocity(vel.x, force.x, 1);
                        vel.y = calc_velocity(vel.y, force.y, 1);

                        pos.x += vel.x * dt;
                        pos.y += vel.y * dt;

                        force.x = 0.0;
                        force.y = 0.0;
                    }
                }
            }
        }
    }
}

fn calc_velocity(velocity: f64, force: f64, mass: i32) -> f64 {
    let damping_force = 0.9 * velocity;
    let a = (force - damping_force) / mass as f64;
    let mut velocity = velocity;
    velocity += a;

    if velocity > 100.0 {
        velocity = 100.0;
    }
    if velocity < -100.0 {
        velocity = -100.0;
    }

    velocity
}
