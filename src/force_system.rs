use crate::quad::*;
use crate::structs::*;

pub struct ForceSystemOptions {
    gravitational_constant: i32,
    theta: f64,
}

pub struct ForceSystem {
    pub barnes_hut_tree: QuadTree,
    options: ForceSystemOptions,
}

impl ForceSystem {
    pub fn new(bounds: AABB) -> Self {
        ForceSystem {
            barnes_hut_tree: QuadTree::new(bounds, 0),
            options: ForceSystemOptions {
                gravitational_constant: -2000,
                theta: 0.7,
            },
        }
    }

    pub fn construct_tree(&mut self, bounds: AABB, positions: &Vec<Option<(ID, Position)>>) {
        self.barnes_hut_tree = QuadTree::new(bounds, 0);

        for position in positions.iter() {
            if let Some(pos) = position {
                let (id, p) = pos;
                let p = p.to_vec2d();
                self.barnes_hut_tree.insert(id, &p);
            }
        }
    }

    fn get_force_contributions(&self, node: &ID, pos: &Vector2d, branch: &QuadTree) -> Vector2d {
        let mut forces = Vector2d { x: 0.0, y: 0.0 };

        if branch.divided || branch.elements.len() > 0 {
            let dx = branch.center_of_mass.x - pos.x;
            let dy = branch.center_of_mass.y - pos.y;

            let distance = (dx * dx + dy * dy).sqrt();
            let s = if branch.bounds.size.x > branch.bounds.size.y {
                branch.bounds.size.x
            } else {
                branch.bounds.size.y
            };

            let sd = s / distance;

            if sd < self.options.theta {
                forces = forces.add_vec(&self.calc_force(distance, dx, dy, 1).to_vec2d());
            } else if branch.divided {
                if let Some(tl) = &branch.tl {
                    forces = forces.add_vec(&self.get_force_contributions(node, &pos, &tl));
                }
                if let Some(tr) = &branch.tr {
                    forces = forces.add_vec(&self.get_force_contributions(node, &pos, &tr));
                }
                if let Some(bl) = &branch.bl {
                    forces = forces.add_vec(&self.get_force_contributions(node, &pos, &bl));
                }
                if let Some(br) = &branch.br {
                    forces = forces.add_vec(&self.get_force_contributions(node, &pos, &br));
                }
            } else {
                if !branch.elements.contains_key(node) {
                    forces = forces.add_vec(&self.calc_force(distance, dx, dy, 1).to_vec2d());
                }
            }
        }

        forces
    }

    fn calc_force(&self, mut distance: f64, mut dx: f64, dy: f64, mass: i32) -> Force {
        if distance < 1.0 {
            distance = 1.0;
            dx = 1.0;
        }

        let gravity_force =
            (self.options.gravitational_constant * mass) as f64 / (distance * distance * distance);

        Force {
            x: dx * gravity_force,
            y: dy * gravity_force,
        }
    }

    pub fn update(
        &mut self,
        bounds: AABB,
        positions: &Vec<Option<(ID, Position)>>,
        forces: &mut Vec<Option<Force>>,
    ) -> () {
        self.construct_tree(bounds, positions);
        for (index, position) in positions.iter().enumerate() {
            if let Some((id, pos)) = position {
                let force =
                    self.get_force_contributions(id, &pos.to_vec2d(), &self.barnes_hut_tree);

                if let Some(node_force) = &mut forces[index] {
                    node_force.x += force.x;
                    node_force.y += force.y;
                }
            }
        }
    }
}
