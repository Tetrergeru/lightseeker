use crate::geometry::{Matrix, Vector3};

pub struct Transform {
    position: Vector3,
    scale: Vector3,
    angle_h: f32,
    angle_v: f32,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            position: Vector3::zero(),
            angle_h: 0.0,
            angle_v: 0.0,
            scale: Vector3::repeat(1.0),
        }
    }

    pub fn from_xyz_hv(x: f32, y: f32, z: f32, h: f32, v: f32) -> Self {
        let mut t = Self::new();
        t.translate(x, y, z);
        t.rotate_h(h);
        t.rotate_v(v);
        t
    }

    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        let mut t = Self::new();
        t.translate(x, y, z);
        t
    }

    pub fn matrix(&self) -> Matrix {
        Matrix::translate(self.position)
            * Matrix::rotation_x(self.angle_v)
            * Matrix::rotation_y(self.angle_h)
            * Matrix::scale(self.scale.x())
    }

    pub fn reverse_matrix(&self) -> Matrix {
        // Matrix::scale(-self.scale.x())
        Matrix::rotation_x(-self.angle_v)
            * Matrix::rotation_y(-self.angle_h)
            * Matrix::translate(self.position * -1.0)
    }

    pub fn normal_matrix(&self) -> Matrix {
        Matrix::rotation_x(-self.angle_v) * Matrix::rotation_y(-self.angle_h)
    }

    pub fn direction(&self) -> Vector3 {
        Matrix::rotation_y(self.angle_h)
            * (Matrix::rotation_x(self.angle_v) * Vector3::from_xyz(0.0, 0.0, -1.0))
    }

    pub fn position(&self) -> Vector3 {
        self.position
    }

    pub fn translate(&mut self, dx: f32, dy: f32, dz: f32) {
        self.position += Vector3::from_xyz(dx, dy, dz);
    }

    pub fn rotate_h(&mut self, dh: f32) {
        self.angle_h += dh;
    }

    pub fn rotate_v(&mut self, dv: f32) {
        self.angle_v += dv;
    }

    pub fn scale(&mut self, scale_factor: f32) {
        self.scale *= scale_factor;
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::new()
    }
}
