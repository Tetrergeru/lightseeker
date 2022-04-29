use crate::{matrix::Matrix, vector::Vector3};

pub struct Transform {
    pub position: Vector3,
    pub scale: Vector3,
    pub angle_h: f32,
    pub angle_v: f32,

    matrix: Matrix,
}

impl Transform {
    pub fn new() -> Self {
        let mut transform = Self {
            position: Vector3::zero(),
            angle_h: 0.0,
            angle_v: 0.0,
            matrix: Matrix::zero(),
            scale: Vector3::repeat(1.0),
        };
        transform.eval_matrix();
        transform
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

    fn eval_matrix(&mut self) {
        self.matrix = Matrix::ident()
            * Matrix::translate(self.position)
            * Matrix::rotation_x(self.angle_v)
            * Matrix::rotation_y(self.angle_h)
            * Matrix::scale(self.scale.x())
    }

    pub fn matrix(&self) -> Matrix {
        self.matrix
    }

    pub fn normal_matrix(&self) -> Matrix {
        Matrix::ident() * Matrix::rotation_x(-self.angle_v) * Matrix::rotation_y(-self.angle_h)
    }

    pub fn translate(&mut self, dx: f32, dy: f32, dz: f32) {
        self.position += Vector3::from_xyz(dx, dy, dz);
        self.eval_matrix();
    }

    pub fn rotate_h(&mut self, dh: f32) {
        self.angle_h += dh;
        self.eval_matrix();
    }

    pub fn rotate_v(&mut self, dv: f32) {
        self.angle_v += dv;
        self.eval_matrix();
    }

    pub fn scale(&mut self, scale_factor: f32) {
        self.scale *= scale_factor;
        self.eval_matrix();
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::new()
    }
}
