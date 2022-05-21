use crate::geometry::{aabb::Aabb, raycast::Ray, Matrix, Transform, Vector3};

pub struct RigidBody {
    aabb: Aabb,
    pub transform: Transform,
    movable: bool,
}

pub const SLOPE_HEIGHT: f32 = 0.2;

impl RigidBody {
    pub fn new(size: Vector3, offset: Vector3, transform: Transform) -> Self {
        Self {
            aabb: Aabb::new(
                {
                    let t = Transform::new();
                    t.translate_vec(offset);
                    t.set_parent(transform.clone());
                    t
                },
                size,
            ),
            transform,
            movable: false,
        }
    }

    pub fn as_movable(mut self) -> Self {
        self.movable = true;
        self
    }

    pub fn collide(&self, other: &Self) {
        let mtv = self.aabb.find_mtv(&other.aabb, Some(SLOPE_HEIGHT));
        if let Some(mtv) = mtv {
            if self.movable && other.movable {
                self.transform.translate_vec(mtv * 0.5);
                other.transform.translate_vec(mtv * -0.5);
            } else if self.movable {
                self.transform.translate_vec(mtv);
            } else if other.movable {
                other.transform.translate_vec(mtv * -1.0);
            }
        }
    }

    pub fn cast_ray(&self, ray: &Ray) -> Option<(f32, Vector3)> {
        self.aabb.cast_ray(ray)
    }

    pub fn frame_matrix(&self) -> Matrix {
        self.aabb.frame_matrix()
    }
}
