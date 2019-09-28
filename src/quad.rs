use crate::structs::*;
use std::collections::HashMap;

pub struct AABB {
    pub position: Position,
    pub size: Size,
}

impl AABB {
    pub fn new((x, y): (f64, f64), (height, width): (f64, f64)) -> Self {
        AABB {
            position: Position { x, y },
            size: Size {
                x: width,
                y: height,
            },
        }
    }

    pub fn half_width(&self) -> f64 {
        self.size.x / 2.0
    }

    pub fn half_height(&self) -> f64 {
        self.size.y / 2.0
    }

    pub fn horizontal_midpoint(&self) -> f64 {
        self.position.x + self.half_width()
    }

    pub fn vertical_midpoint(&self) -> f64 {
        self.position.y + self.half_height()
    }

    pub fn overlaps_vec2d(&self, vec: &Vector2d) -> bool {
        self.position.x - self.size.x <= vec.x
            && vec.x <= self.position.x + self.size.x
            && self.position.y - self.size.y <= vec.y
            && vec.y <= self.position.y + self.size.y
    }

    pub fn overlaps_aabb(&self, aabb: &AABB) -> bool {
        !(aabb.position.x - aabb.size.x > self.position.x + self.size.x
            || aabb.position.x + aabb.size.x < self.position.x - self.size.x
            || aabb.position.y - aabb.size.y > self.position.y + self.size.y
            || aabb.position.y + aabb.size.y < self.position.y - self.size.y)
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn size(&self) -> Size {
        self.size
    }
}

pub struct QuadTree {
    max_elements: i32,
    max_depth: i32,
    pub bounds: AABB,
    pub depth: i32,
    pub divided: bool,
    pub mass: i32,
    pub center_of_mass: Vector2d,
    pub tl: Option<Box<QuadTree>>,
    pub tr: Option<Box<QuadTree>>,
    pub bl: Option<Box<QuadTree>>,
    pub br: Option<Box<QuadTree>>,
    pub elements: HashMap<ID, Vector2d>,
}

impl QuadTree {
    pub fn new(bounds: AABB, depth: i32) -> Self {
        QuadTree {
            max_elements: 4,
            max_depth: 16,
            bounds,
            divided: false,
            depth,
            mass: 0,
            center_of_mass: Vector2d { x: 0.0, y: 0.0 },
            tl: None,
            tr: None,
            bl: None,
            br: None,
            elements: HashMap::new(),
        }
    }

    pub fn insert(&mut self, node: &ID, position: &Vector2d) -> bool {
        if !self.bounds.overlaps_vec2d(&position) {
            return false;
        } else if self.depth == self.max_depth || (!self.divided && self.elements.len() == 0) {
            let id = match node {
                ID::Num(num) => ID::Num(*num),
                ID::Str(s) => ID::Str(s.clone()),
            };
            let pos = Vector2d {
                x: position.x,
                y: position.y,
            };
            self.elements.insert(id, pos);
            self.update_mass(&pos, 1);
            return true;
        } else {
            if !self.divided {
                self.divide();
            }
            let mut inserted = false;
            if let Some(tl) = &mut self.tl {
                inserted = tl.insert(node, &position);
            } else if let Some(tr) = &mut self.tr {
                inserted = tr.insert(node, &position);
            } else if let Some(bl) = &mut self.bl {
                inserted = bl.insert(node, &position);
            } else if let Some(br) = &mut self.br {
                inserted = br.insert(node, &position);
            }

            inserted
        }
    }

    fn update_mass(&mut self, pos: &Vector2d, mass: i32) {
        let com = &mut self.center_of_mass;
        let total_mass = self.mass + mass;
        let total_mass_inverse = 1 / total_mass;

        com.x = com.x * self.mass as f64 + pos.x * mass as f64;
        com.x *= total_mass_inverse as f64;

        com.y = com.y * self.mass as f64 + pos.y * mass as f64;
        com.y *= total_mass_inverse as f64;

        self.mass = total_mass;
    }

    fn divide(&mut self) {
        let hw = self.bounds.half_width();
        let hh = self.bounds.half_height();
        let new_depth = self.depth + 1;
        let x = self.bounds.position.x;
        let y = self.bounds.position.y;

        self.tl = Some(Box::new(QuadTree::new(
            AABB::new((x, y), (hh, hw)),
            new_depth,
        )));
        self.tr = Some(Box::new(QuadTree::new(
            AABB::new((x + hw, y), (hh, hw)),
            new_depth,
        )));
        self.bl = Some(Box::new(QuadTree::new(
            AABB::new((x, y + hh), (hh, hw)),
            new_depth,
        )));
        self.br = Some(Box::new(QuadTree::new(
            AABB::new((x + hw, y + hh), (hh, hw)),
            new_depth,
        )));
        self.divided = true;

        for elem in self.elements.iter() {
            let (id, pos) = &elem;
            let mut inserted = false;
            if let Some(tl) = &mut self.tl {
                inserted = tl.insert(id, pos);
            }
            if let Some(tr) = &mut self.tr {
                inserted = tr.insert(id, pos);
            }
            if let Some(bl) = &mut self.bl {
                inserted = bl.insert(id, pos);
            }
            if let Some(br) = &mut self.br {
                inserted = br.insert(id, pos);
            }
        }
    }

    pub fn query(&self, area: &AABB) -> Vec<ID> {
        let mut out: Vec<ID> = Vec::new();

        if self.bounds.overlaps_aabb(&area) {
            if (!self.divided) {
                for elem in self.elements.iter() {
                    let (id, pos) = elem;

                    if (area.overlaps_vec2d(pos)) {
                        let new_id = id.clone();
                        out.push(new_id);
                    }
                }
            } else {
                if let Some(tl) = &self.tl {
                    out.append(&mut tl.query(&area));
                }
                if let Some(tr) = &self.tr {
                    out.append(&mut tr.query(&area));
                }
                if let Some(bl) = &self.bl {
                    out.append(&mut bl.query(&area));
                }
                if let Some(br) = &self.br {
                    out.append(&mut br.query(&area));
                }
            }
        }

        out
    }
}
