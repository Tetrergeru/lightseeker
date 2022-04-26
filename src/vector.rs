use std::ops::{Mul, AddAssign};

#[derive(Clone, Copy, Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn len(self) -> f32 {
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
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            coord: Vector2::new(x, y),
            size: Vector2::new(w, h),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vector4 {
    pub vector: [f32; 4],
}

impl Vector4 {
    pub fn zero() -> Self {
        Self { vector: [0.0; 4] }
    }

    pub fn from_xyzw(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            vector: [x, y, z, w],
        }
    }

    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Self {
            vector: [x, y, z, 1.0],
        }
    }

    pub fn x(&self) -> f32 {
        self.get(0)
    }

    pub fn y(&self) -> f32 {
        self.get(1)
    }

    pub fn z(&self) -> f32 {
        self.get(2)
    }

    pub fn w(&self) -> f32 {
        self.get(3)
    }

    pub fn get(&self, i: usize) -> f32 {
        self.vector[i]
    }

    pub fn set(&mut self, i: usize, value: f32) {
        self.vector[i] = value
    }
}

impl Mul<f32> for Vector4 {
    type Output = Vector4;

    fn mul(mut self, rhs: f32) -> Self::Output {
        for i in 0..3 {
            self.vector[i] *= rhs;
        }
        self
    }
}

impl AddAssign for Vector4 {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..4 {
            self.vector[i] += rhs.vector[i];
        }
    }
}