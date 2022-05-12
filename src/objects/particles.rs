use std::rc::Rc;

use rand::Rng;
use web_sys::WebGl2RenderingContext as Gl;

use crate::geometry::{transform::RawTransform, Transform, Vector3};

use super::{shape::Shape, texture::Texture};

pub struct Particles {
    pub buffer: Texture,
    pub texture: Rc<Texture>,
    pub transform: Transform,
    pub particles: Vec<RawTransform>,
    pub verts: i32,
    pub shape: Rc<Shape>,
}

impl Particles {
    pub fn new(gl: &Gl, shape: Rc<Shape>, texture: Rc<Texture>) -> Self {
        Self {
            buffer: Texture::from_shape(gl, &shape),
            texture,
            transform: Transform::new(),
            particles: Self::random_particles(),
            verts: shape.buffer_length(),
            shape,
        }
    }

    pub fn vertices(&self) -> i32 {
        self.verts
    }

    fn random_particles() -> Vec<RawTransform> {
        use std::f32::consts::PI;
        let mut rand = rand::thread_rng();
        let mut vec = vec![];
        for _ in 0..10 {
            let mut t = RawTransform::new();
            t.scale(rand.gen_range(0.1, 0.5));
            t.position = Vector3::from_xyz(
                rand.gen_range(-1.0, 1.0),
                rand.gen_range(-1.0, 1.0),
                rand.gen_range(-1.0, 1.0),
            );
            t.rotation = Vector3::from_xyz(
                rand.gen_range(-PI, PI),
                rand.gen_range(-PI, PI),
                rand.gen_range(-PI, PI),
            );
            vec.push(t);
        }
        vec
    }
}
