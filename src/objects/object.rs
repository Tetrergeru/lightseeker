use std::rc::Rc;

use crate::matrix::Matrix;

use super::shape::Shape;

pub struct Object {
    pub shape: Rc<Shape>,
    pub transform: Matrix,
}

impl Object {
    pub fn new(shape: Rc<Shape>, transform: Matrix) -> Self {
        Self { shape, transform }
    }
}
