use std::rc::Rc;

use web_sys::WebGl2RenderingContext as Gl;

use crate::{
    geometry::{Matrix, Transform, Vector3},
    objects::texture::Texture,
};

pub struct Directional {
    pub transform: Transform,
    pub fov: f32,
    pub inner_fov: f32,
    pub diffuse: f32,
    pub specular: f32,

    w: u32,
    h: u32,
    texture: Rc<Texture>,
}

impl Directional {
    pub fn new(gl: &Gl, transform: Transform) -> Self {
        let w = 2048;
        let h = 2048;

        Self {
            transform,
            texture: Rc::new(Texture::new_depth(gl, w, h)),
            fov: std::f32::consts::PI / 4.0,
            inner_fov: std::f32::consts::PI / 3.0,
            diffuse: 2.0,
            specular: 2.0,
            w,
            h,
        }
    }

    pub fn matrix(&self) -> Matrix {
        Matrix::perspective(self.fov, 1.0, 1.0, 20.0) * self.transform.reverse_matrix()
    }

    pub fn depth(&self) -> &Rc<Texture> {
        &self.texture
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
}
