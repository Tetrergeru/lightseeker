use std::{cell::Ref, collections::HashMap};

use crate::{
    geometry::Transform,
    objects::{object::Object, parsers::animation::Animation, rigid_body::RigidBody},
};

use super::{
    entity::{AnimationId, BodyId, Entity, EntityId, ObjectId, ScriptId},
    id_repository::{Id, IdRepositpry},
    script::{Script, ScriptInst},
};

pub struct EntityFactory {
    pub entities: IdRepositpry<EntityId, Entity>,
    pub objects: IdRepositpry<ObjectId, Object>,
    pub bodies: IdRepositpry<BodyId, RigidBody>,
    pub animatons: IdRepositpry<AnimationId, Animation>,
    pub scripts: IdRepositpry<ScriptId, ScriptInst>,
}

impl EntityFactory {
    pub fn new() -> Self {
        Self {
            entities: IdRepositpry::new(),
            objects: IdRepositpry::new(),
            bodies: IdRepositpry::new(),
            animatons: IdRepositpry::new(),
            scripts: IdRepositpry::new(),
        }
    }

    pub fn add_entity(&mut self, transform: Transform) -> EntityId {
        self.entities.insert(Entity {
            transform,
            object: ObjectId::empty(),
            animation: AnimationId::empty(),
            body: BodyId::empty(),
            scripts: vec![],
        })
    }

    pub fn add_object(&self, entity: &EntityId, object: Object) -> ObjectId {
        let id = self.objects.insert(object);
        let mut entity = self.entities.get_mut(entity).unwrap();
        entity.object = id.clone();
        id
    }

    pub fn add_body(&self, entity: &EntityId, body: RigidBody) -> BodyId {
        let id = self.bodies.insert(body);
        let mut entity = self.entities.get_mut(entity).unwrap();
        entity.body = id.clone();
        id
    }

    pub fn add_animation(&self, entity: &EntityId, animation: Animation) -> AnimationId {
        let id = self.animatons.insert(animation);
        let mut entity = self.entities.get_mut(entity).unwrap();
        entity.animation = id.clone();
        id
    }

    pub fn add_script(&self, entity: &EntityId, script: ScriptInst) -> ScriptId {
        let id = self.scripts.insert(script);
        let mut entity = self.entities.get_mut(entity).unwrap();
        entity.scripts.push(id.clone());
        id
    }

    pub fn find_script<S: Script + 'static>(&self, entity: &EntityId) -> Option<&S> {
        let script_id = self.get_script_id::<S>(entity)?;
        self.get_script(&script_id)
    }

    pub fn find_script_mut<S: Script + 'static>(&self, entity: &EntityId) -> Option<&mut S> {
        let script_id = self.get_script_id::<S>(entity)?;
        self.get_script_mut(&script_id)
    }

    pub fn get_script<S: Script + 'static>(&self, script_id: &ScriptId) -> Option<&S> {
        let script = self.scripts.get(script_id)?;
        let script = script.raw_borrow().as_any();
        if script.is::<S>() {
            Some(script.downcast_ref::<S>().unwrap())
        } else {
            None
        }
    }

    pub fn get_script_mut<S: Script + 'static>(&self, script_id: &ScriptId) -> Option<&mut S> {
        let script = self.scripts.get(script_id)?;
        let script = script.raw_borrow_mut().as_any_mut();
        if script.is::<S>() {
            Some(script.downcast_mut::<S>().unwrap())
        } else {
            None
        }
    }

    pub fn get_script_id<S: Script + 'static>(&self, entity: &EntityId) -> Option<ScriptId> {
        let entity = self.entities.get(entity).unwrap();
        for script_id in entity.scripts.iter() {
            let script = self.scripts.get(script_id)?;
            let script = script.raw_borrow().as_any();
            if script.is::<S>() {
                return Some(script_id.clone());
            }
        }
        None
    }

    pub fn iter_objects(&self) -> Ref<HashMap<ObjectId, Object>> {
        self.objects.iter()
    }

    pub fn iter_scripts(&self) -> Ref<HashMap<ScriptId, ScriptInst>> {
        self.scripts.iter()
    }
}

impl Default for EntityFactory {
    fn default() -> Self {
        Self::new()
    }
}
