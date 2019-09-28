// pub enum Component {
//     Position { x: i32, y: i32 },
//     Spring { from: i32, to: i32 },
//     Force { x: i32, y: i32 },
//     Velocity { x: i32, y: i32 },
//     Fixed { is_fixed: bool },
//     Size { x: i32, y: i32 },
// }

#[derive(Eq, Hash, PartialEq)]
pub enum ID {
    Num(i32),
    Str(String),
}

impl Clone for ID {
    fn clone(&self) -> ID {
        match self {
            ID::Num(num) => ID::Num(*num),
            ID::Str(s) => ID::Str(s.clone()),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn to_vec2d(&self) -> Vector2d {
        Vector2d {
            x: self.x,
            y: self.y,
        }
    }
}

pub struct Spring {
    pub from: ID,
    pub to: ID,
}

pub struct Force {
    pub x: f64,
    pub y: f64,
}

impl Force {
    pub fn to_vec2d(&self) -> Vector2d {
        Vector2d {
            x: self.x,
            y: self.y,
        }
    }
}

pub struct Velocity {
    pub x: f64,
    pub y: f64,
}

impl Velocity {
    pub fn to_vec2d(&self) -> Vector2d {
        Vector2d {
            x: self.x,
            y: self.y,
        }
    }
}

pub struct Fixed {
    pub is_fixed: bool,
}

#[derive(Clone, Copy)]
pub struct Size {
    pub x: f64,
    pub y: f64,
}

pub struct Vector2d {
    pub x: f64,
    pub y: f64,
}

impl Vector2d {
    pub fn add_vec(&self, vec: &Vector2d) -> Vector2d {
        Vector2d {
            x: self.x + vec.x,
            y: self.y + vec.y,
        }
    }

    pub fn sub_vec(&self, vec: &Vector2d) -> Vector2d {
        Vector2d {
            x: self.x - vec.x,
            y: self.y - vec.y,
        }
    }

    pub fn normalize(&self) -> Vector2d {
        self.divide_scalar(self.len())
    }

    pub fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn divide_scalar(&self, scalar: f64) -> Vector2d {
        if scalar != 0.0 {
            Vector2d {
                x: self.x / scalar,
                y: self.y / scalar,
            }
        } else {
            Vector2d {
                x: self.x,
                y: self.y,
            }
        }
    }

    pub fn distance_to_vec(&self, vec: &Vector2d) -> f64 {
        let dx = vec.x - self.x;
        let dy = vec.y - self.y;

        (dx * dx + dy * dy).sqrt()
    }
}

impl Clone for Vector2d {
    fn clone(&self) -> Vector2d {
        *self
    }
}

impl Copy for Vector2d {}
