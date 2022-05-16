use std::rc::Rc;

use crate::geometry::{Matrix, Transform};

use super::{
    parsers::{
        animation::{Animation, AnimationFrame},
        skeleton::{BoneTransform, Skeleton},
    },
    shape::Shape,
    texture::Texture,
};

pub struct Object {
    pub shape: Rc<Shape>,
    pub skeleton: Vec<BoneTransform>,
    pub texture: Rc<Texture>,
    pub transform: Transform,
    pub animation: Option<Rc<Animation>>,
    pub animation_frame: f32,
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
            animation: None,
            animation_frame: 0.0,
        }
    }

    pub fn set_pose(&self, pose: &AnimationFrame) {
        for i in 0..pose.transforms.len() {
            self.skeleton[i].set_pose(pose.transforms[i]);
        }
    }

    pub fn has_skeleton(&self) -> bool {
        !self.skeleton.is_empty()
    }

    pub fn with_skeleton(mut self, skeleton: &Skeleton) -> Self {
        self.skeleton = skeleton.make_nested_transforms(self.transform.clone());
        self
    }

    pub fn get_bone_transform(&self, bone: usize) -> &Transform {
        &self.skeleton[bone].transform
    }

    pub fn with_animation(mut self, animation: Rc<Animation>) -> Self {
        self.animation = Some(animation);
        self
    }

    pub fn tick_animation(&mut self, delta_time: f32) {
        self.animation_frame += delta_time * 20.0;
    }

    pub fn bone_matrices(&self) -> impl Iterator<Item = Matrix> + '_ {
        let animation = &self.animation.as_ref().unwrap().frames;
        let frame_idx = (self.animation_frame.floor() as usize) % animation.len();

        self.set_pose(&animation[frame_idx]);
        self.skeleton.iter().map(|it| it.matrix())
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
