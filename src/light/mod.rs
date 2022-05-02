use web_sys::WebGl2RenderingContext as Gl;

use std::rc::Rc;

use crate::{objects::texture::Texture, geometry::Transform};

pub use self::{directional::Directional, point::Point};

pub mod directional;
pub mod point;

pub enum Light {
    Directional(Directional),
    Point(Point),
}

impl Light {
    pub fn depth(&self) -> &Rc<Texture> {
        match self {
            Light::Directional(d) => d.depth(),
            Light::Point(p) => p.depth(),
        }
    }

    pub fn texture_bounds(&self) -> (u32, u32) {
        match self {
            Light::Directional(d) => d.texture_bounds(),
            Light::Point(p) => p.texture_bounds(),
        }
    }

    pub fn diffuse(&self) -> f32 {
        match self {
            Light::Directional(d) => d.diffuse,
            Light::Point(p) => p.diffuse,
        }
    }

    pub fn specular(&self) -> f32 {
        match self {
            Light::Directional(d) => d.specular,
            Light::Point(p) => p.specular,
        }
    }

    pub fn new_directional(gl: &Gl, transform: Transform) -> Self {
        Self::Directional(Directional::new(gl, transform))
    }

    pub fn new_point(gl: &Gl, transform: Transform) -> Self {
        Self::Point(Point::new(gl, transform))
    }
}
