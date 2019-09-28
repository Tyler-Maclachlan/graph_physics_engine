use crate::structs::*;

struct CentralGravityOptions {
    central_gravity: f64,
    center: Position
}

pub fn update(center: &Vector2d, positions: &Vec<Option<Position>>, forces: &mut Vec<Option<Force>>) {
    for i in 0..positions.len() {
        if let Some(pos) = positions[i] {
            if let Some(force) = &mut forces[i] {
                let dx = center.x - pos.x;
                let dy = center.y - pos.y;

                let distance = (dx * dx + dy * dy).sqrt();
                let f = calc_force(distance, dx, dy, 1);

                force.x += f.x;
                force.y += f.y;
            }
        }
    }
}

fn calc_force(distance: f64, dx: f64, dy: f64, mass: i32) -> Force {
    let mut distance = distance;
    let mut dx = dx;

    if distance < 0.1 {
        distance = 0.1;
        dx = 0.1;
    }

    let force = 0.2 * mass as f64 / distance;

    Force {
        x: force * dx,
        y: force * dy
    }
}