use std::rc::Rc;

use crate::matrix::Matrix;

use super::{shape::Shape, texture::Texture};

pub struct Object {
    pub shape: Rc<Shape>,
    pub texture: Rc<Texture>,
    pub transform: Matrix,
}

impl Object {
    pub fn new(shape: Rc<Shape>, texture: Rc<Texture>, transform: Matrix) -> Self {
        Self {
            shape,
            texture,
            transform,
        }
    }
}
