use crate::vector::{Vector2, Vector3};

#[derive(Clone)]
pub struct VertexData {
    pub point: Vector3,
    pub normal: Vector3,
    pub texture_coord: Vector2,
}
