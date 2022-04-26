#[derive(Clone, Copy, Debug)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn len(self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn norm(self) -> Self {
        let len = self.len();
        Self::new(self.x / len, self.y / len)
    }
}

pub struct Rectangle {
    pub coord: Vector2,
    pub size: Vector2,
}

impl Rectangle {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Self {
            coord: Vector2::new(x, y),
            size: Vector2::new(w, h),
        }
    }
}
