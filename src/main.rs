use std::collections::HashMap;
mod force_system;
mod quad;
mod structs;

use force_system::*;
use quad::*;
use structs::*;

pub struct PhysicsEngine {
    force_system: ForceSystem,
}

fn main() {
    let physics_engine = PhysicsEngine {
        force_system: ForceSystem::new(AABB::new((0.0, 0.0), (1080.0, 1920.0))),
    };
}
