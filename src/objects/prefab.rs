use std::rc::Rc;

use crate::{
    geometry::{Transform, Vector3},
    world::World,
};

use super::{
    object::Object, parsers::animation::Animation, rigid_body::RigidBody, shape::Shape,
    texture::Texture,
};

pub trait Prefab {
    fn construct(&self, transform: Transform, world: &mut World);
}

pub struct ObjectPrefab {
    shape: Rc<Shape>,
    texture: Rc<Texture>,
    rigid_body: Option<(Vector3, Vector3, bool)>,
    animation: Option<Rc<Animation>>,
}

impl ObjectPrefab {
    pub fn new(shape: &Rc<Shape>, texture: &Rc<Texture>) -> Self {
        Self {
            shape: shape.clone(),
            texture: texture.clone(),
            rigid_body: None,
            animation: None,
        }
    }

    pub fn with_body(mut self, size: Vector3, offset: Vector3, movable: bool) -> Self {
        self.rigid_body = Some((size, offset, movable));
        self
    }

    pub fn with_animation(mut self, animation: &Rc<Animation>) -> Self {
        self.animation = Some(animation.clone());
        self
    }
}

impl Prefab for ObjectPrefab {
    fn construct(&self, transform: Transform, world: &mut World) {
        let mut object = Object::new(self.shape.clone(), self.texture.clone(), transform.clone());
        if let Some((size, offset, movable)) = self.rigid_body.as_ref() {
            let mut body_clone = RigidBody::new(*size, *offset, transform);
            if *movable {
                body_clone = body_clone.as_movable();
            }
            world.add_body(body_clone);
        }
        if let Some(animation) = self.animation.as_ref() {
            object = object
                .with_skeleton(&animation.skeleton)
                .with_animation(animation.clone());
        }
        world.add_object(object);
    }
}
