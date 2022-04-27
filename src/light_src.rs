use crate::{matrix::Matrix, vector::Vector3};

pub struct LightSrc {
    pub position: Vector3,
    pub angle_h: f32,
    pub angle_v: f32,

    matrix: Matrix,
}

impl LightSrc {
    pub fn new(point: Vector3, angle_h: f32, angle_v: f32) -> Self {
        let mut light = Self {
            position: point,
            angle_h,
            angle_v,
            matrix: Matrix::zero(),
        };
        light.eval_matrix();
        light
    }

    fn eval_matrix(&mut self) {
        self.matrix = Matrix::ident()
            * Matrix::perspective(std::f32::consts::PI / 4.0, 1.0, 1.0, 10.0)
            * Matrix::rotation_x(self.angle_v)
            * Matrix::rotation_y(self.angle_h)
            * Matrix::translate(self.position)
    }

    pub fn matrix(&self) -> Matrix {
        self.matrix
    }
}
