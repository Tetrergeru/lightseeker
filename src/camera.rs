use crate::{
    matrix::Matrix,
    vector::{Vector2, Vector3, Vector4},
};

pub struct Camera {
    pub position: Vector3,
    pub angle_h: f32,
    pub angle_v: f32,
    pub aspect: f32,

    matrix: Matrix,
}

impl Camera {
    pub fn new(position: Vector3, angle_h: f32, angle_v: f32) -> Self {
        let mut camera = Self {
            position,
            angle_h,
            angle_v,
            matrix: Matrix::zero(),
            aspect: 1.0,
        };
        camera.eval_matrix();
        camera
    }

    pub fn with_aspect(mut self, aspect: f32) -> Self {
        self.aspect = aspect;
        self.eval_matrix();
        self
    }

    fn eval_matrix(&mut self) {
        self.matrix = Matrix::ident()
            * Matrix::perspective(1.5, self.aspect, 0.1, 2000.0)
            * Matrix::rotation_x(self.angle_v)
            * Matrix::rotation_y(self.angle_h)
            * Matrix::translate(self.position)
    }

    pub fn move_h(&mut self, vec: Vector2) {
        let delta = Matrix::rotation_y(-self.angle_h) * Vector4::from_xyz(vec.x, 0.0, vec.y);
        for i in 0..3 {
            self.position.set(i, self.position.get(i) + delta.get(i))
        }
        self.eval_matrix();
    }

    pub fn rotate_h(&mut self, angle: f32) {
        self.angle_h += angle;
        self.eval_matrix();
    }

    pub fn rotate_v(&mut self, angle: f32) {
        self.angle_v += angle;
        let angle_limit = 1.0;
        if self.angle_v > angle_limit {
            self.angle_v = angle_limit
        } else if self.angle_v < -angle_limit {
            self.angle_v = -angle_limit
        }
        self.eval_matrix();
    }

    pub fn matrix(&self) -> Matrix {
        self.matrix
    }
}
