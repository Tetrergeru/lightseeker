use std::rc::Rc;

use crate::geometry::{Matrix, Transform};

use super::{shape::Shape, texture::Texture};

pub struct Object {
    pub shape: Rc<Shape>,
    pub texture: Rc<Texture>,
    pub transform: Transform,
    pub ignored_by_light: bool,
}

impl Object {
    pub fn new(shape: Rc<Shape>, texture: Rc<Texture>, transform: Transform) -> Self {
        Self {
            shape,
            texture,
            transform,
            ignored_by_light: false,
        }
    }

    pub fn ignored_by_light(mut self) -> Self {
        self.ignored_by_light = true;
        self
    }

    pub fn transform_matrix(&self) -> Matrix {
        self.transform.matrix()
    }

    pub fn normal_matrix(&self) -> Matrix {
        self.transform.normal_matrix()
    }
}
