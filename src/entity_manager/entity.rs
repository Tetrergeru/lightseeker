use crate::geometry::Transform;

use super::id_repository::Id;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct EntityId(usize);

impl Id for EntityId {
    fn empty() -> Self {
        Self(0)
    }

    fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

pub struct Entity {
    pub transform: Transform,
    pub object: ObjectId,
    pub animation: AnimationId,
    pub body: BodyId,
    pub scripts: Vec<ScriptId>,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct ObjectId(usize);

impl Id for ObjectId {
    fn empty() -> Self {
        Self(0)
    }

    fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct AnimationId(usize);

impl Id for AnimationId {
    fn empty() -> Self {
        Self(0)
    }

    fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct BodyId(usize);

impl Id for BodyId {
    fn empty() -> Self {
        Self(0)
    }

    fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct ScriptId(usize);

impl Id for ScriptId {
    fn empty() -> Self {
        Self(0)
    }

    fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}
