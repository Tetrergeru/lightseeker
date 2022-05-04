use std::{f32::consts::PI, rc::Rc};

use web_sys::WebGl2RenderingContext as Gl;

use crate::{
    geometry::{Matrix, Transform, Vector3},
    objects::texture::Texture,
};

pub struct Point {
    pub transform: Transform,
    pub diffuse: f32,
    pub specular: f32,
    pub color: Vector3,

    w: u32,
    h: u32,
    depth: Rc<Texture>,
    pub texture: Rc<Texture>,
}

impl Point {
    pub fn new(gl: &Gl, transform: Transform) -> Self {
        let w = 2 * 2048;
        let h = 2 * 2048;

        Self {
            transform,
            diffuse: 1.0,
            specular: 1.0,
            color: Vector3::from_xyz(1.0, 1.0, 1.0),
            w,
            h,
            depth: Rc::new(Texture::new_depth(gl, w, h)),
            texture: Rc::new(Texture::with_size(gl, w, h)),
        }
    }

    pub fn with_color(mut self, color: Vector3) -> Self {
        self.color = color;
        self
    }

    pub fn color(&self) -> Vector3 {
        self.color
    }

    pub fn depth(&self) -> &Rc<Texture> {
        &self.depth
    }

    pub fn position(&self) -> Vector3 {
        self.transform.position()
    }

    pub fn direction(&self) -> Vector3 {
        self.transform.direction()
    }

    pub fn texture_bounds(&self) -> (u32, u32) {
        (self.w, self.h)
    }

    fn fov() -> f32 {
        2.0 * 6.0_f32.sqrt().atan()
    }

    fn aspect() -> f32 {
        2.0 / 3.0_f32.sqrt()
    }

    fn v_angle_0() -> f32 {
        PI / 2.0 + Self::fov() * 2.65 / 2.0
    }

    fn v_angle_123() -> f32 {
        PI / 2.0 + Self::fov() / 2.0
    }

    fn perspective(near: f32, far: f32) -> Matrix {
        Matrix::perspective(Self::fov(), Self::aspect(), near, far)
    }

    pub fn matrices(&self) -> [Matrix; 4] {
        self.matrices_with_nf(1.0, 20.0)
    }

    pub fn matrices_with_nf(&self, near: f32, far: f32) -> [Matrix; 4] {
        let h_angles = [0.0, PI / 3.0, PI, -PI / 3.0];
        let v_angles = [
            Self::v_angle_0(),
            Self::v_angle_123(),
            Self::v_angle_123(),
            Self::v_angle_123(),
        ];

        let mut i = 0;
        [(); 4].map(|_| {
            i += 1;
            Self::perspective(near, far)
                * Matrix::rotation_x(v_angles[i - 1])
                * Matrix::rotation_y(h_angles[i - 1])
                * Matrix::translate(self.transform.position() * -1.0)
        })
    }
}
