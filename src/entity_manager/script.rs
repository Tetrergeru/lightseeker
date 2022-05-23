use std::{any::Any, cell::UnsafeCell, rc::Rc};

use super::{entity::EntityId, entity_factory::EntityFactory};

pub struct ScriptInst(Rc<UnsafeCell<dyn Script>>);

impl ScriptInst {
    #[allow(clippy::needless_lifetimes)]
    pub fn raw_borrow<'a>(&'a self) -> &'a dyn Script {
        let r = &self.0;
        unsafe { &*r.get() as &dyn Script }
    }

    #[allow(clippy::needless_lifetimes)]
    #[allow(clippy::mut_from_ref)]
    pub fn raw_borrow_mut<'a>(&'a self) -> &'a mut dyn Script {
        let r = &self.0;
        unsafe { &mut *r.get() as &mut dyn Script }
    }

    pub fn init(&self, entity_id: EntityId) {
        self.raw_borrow_mut().init(entity_id);
    }

    pub fn update(&self, ctx: &EntityFactory, delta_time: f32) {
        self.raw_borrow_mut().update(ctx, delta_time);
    }
}

pub trait Script: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn init(&mut self, _entity_id: EntityId) {}
    fn update(&mut self, _ctx: &EntityFactory, _delta_time: f32) {}
}

struct Script1 {}

impl Script for Script1 {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
