use std::ops::{AddAssign, Deref, Mul, MulAssign, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn from_xy(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn len(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn norm(self) -> Self {
        let len = self.len();
        Self::from_xy(self.x / len, self.y / len)
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vector3 {
    pub vector: [f32; 3],
}

impl Vector3 {
    pub fn zero() -> Self {
        Self { vector: [0.0; 3] }
    }

    pub fn repeat(v: f32) -> Self {
        Self { vector: [v; 3] }
    }

    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Self { vector: [x, y, z] }
    }

    pub fn get(&self, i: usize) -> f32 {
        self.vector[i]
    }

    pub fn set(&mut self, i: usize, value: f32) {
        self.vector[i] = value
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

    pub fn cross(self, b: Self) -> Self {
        let a = self;
        Self::from_xyz(
            a.y() * b.z() - a.z() * b.y(),
            a.z() * b.x() - a.x() * b.z(),
            a.x() * b.y() - a.y() * b.x(),
        )
    }
}

impl Deref for Vector3 {
    type Target = [f32];

    fn deref(&self) -> &Self::Target {
        &self.vector
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_xyz(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl Mul<f32> for Vector3 {
    type Output = Self;

    fn mul(mut self, rhs: f32) -> Self {
        self *= rhs;
        self
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..3 {
            self.vector[i] += rhs.vector[i];
        }
    }
}

impl MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, rhs: f32) {
        for i in 0..3 {
            self.vector[i] *= rhs;
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
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
