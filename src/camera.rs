use crate::{
    geometry::Matrix,
    geometry::{Transform, Vector3},
};

pub struct Camera {
    pub transform: Transform,
    pub aspect: f32,
}

impl Camera {
    pub fn new(transform: Transform) -> Self {
        Self {
            transform,
            aspect: 1.0,
        }
    }

    pub fn with_aspect(mut self, aspect: f32) -> Self {
        self.aspect = aspect;
        self
    }

    // pub fn move_h(&mut self, vec: Vector2) {
    //     let delta = self.transform.to_raw().direction();
    //     self.transform.translate_vec(delta);

    //     self.eval_matrix();
    // }

    // pub fn move_v(&mut self, dv: f32) {
    //     self.position += Vector3::from_xyz(0.0, dv, 0.0);
    //     self.eval_matrix();
    // }

    pub fn rotate_h(&mut self, angle: f32) {
        self.transform.rotate_h(angle);
    }

    pub fn rotate_v(&mut self, angle: f32) {
        self.transform.rotate_v(angle);
    }

    pub fn matrix(&self) -> Matrix {
        Matrix::perspective(1.5, self.aspect, 0.1, 2000.0) * self.transform.reverse_matrix()
    }

    pub fn direction(&self) -> Vector3 {
        self.transform.direction()
    }

    pub fn position(&self) -> Vector3 {
        self.transform.position()
    }
}
