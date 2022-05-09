use std::rc::Rc;

use crate::geometry::{Matrix, Transform};

use super::{
    parsers::{animation::AnimationFrame, skeleton::Skeleton},
    shape::Shape,
    texture::Texture,
};

pub struct Object {
    pub shape: Rc<Shape>,
    pub skeleton: Vec<Transform>,
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
            skeleton: vec![],
        }
    }

    pub fn set_pose(&self, pose: &AnimationFrame) {
        for i in 0..pose.transforms.len() {
            self.skeleton[i].set_transform(pose.transforms[i]);
        }
    }

    pub fn with_skeleton(mut self, skeleton: &Skeleton) -> Self {
        self.skeleton = skeleton.make_nested_transforms();
        self
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
