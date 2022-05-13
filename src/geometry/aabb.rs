use super::{Matrix, Transform, Vector2, Vector3, transform::RawTransform};

#[derive(Clone)]
pub struct Aabb {
    center: Transform,
    half_size: Vector3,
}

impl Aabb {
    pub fn new(center: Transform, size: Vector3) -> Self {
        Self {
            center,
            half_size: size * 0.5,
        }
    }

    pub fn frame_matrix(&self) -> Matrix {
        let mut tr = RawTransform::new();
        tr.scale = self.half_size;
        tr.position = self.center.position();
        tr.reverse_matrix()
    }

    pub fn find_mtv(&self, other: &Self, slope_height: Option<f32>) -> Option<Vector3> {
        let self_center = self.center.position();
        let other_center = other.center.position();

        if let Some(slope_height) = slope_height {
            let slf_bot = self_center.y() - self.half_size.y();
            let oth_top = other_center.y() + other.half_size.y();
            if slf_bot < oth_top && slf_bot + slope_height > oth_top {
                return Some(Vector3::from_xyz(0.0, oth_top - slf_bot, 0.0));
            }

            let slf_top = self_center.y() + self.half_size.y();
            let oth_bot = other_center.y() - other.half_size.y();
            if oth_bot < slf_top && oth_bot + slope_height > slf_top {
                return Some(Vector3::from_xyz(0.0, oth_bot - slf_top, 0.0));
            }
        }

        let mut min = f32::MAX;
        let mut vector = None;

        for i in 0..3 {
            let slf = Vector2::from_xy(
                self_center.get(i) - self.half_size.get(i),
                self_center.get(i) + self.half_size.get(i),
            );

            let oth = Vector2::from_xy(
                other_center.get(i) - other.half_size.get(i),
                other_center.get(i) + other.half_size.get(i),
            );

            let mtv = Self::one_dimensional_mtv(slf, oth);
            if let Some(mtv) = mtv {
                if mtv < min {
                    let mut vec = Vector3::from_xyz(0.0, 0.0, 0.0);
                    vec.set(i, mtv);
                    vector = Some(vec);

                    min = mtv.abs();
                }
            } else {
                return None;
            }
        }

        vector
    }

    fn one_dimensional_mtv(a: Vector2, b: Vector2) -> Option<f32> {
        if a.y() < b.x() || a.x() > b.y() {
            None
        } else if a.y() < b.y() && a.x() < b.x() {
            Some(b.x() - a.y())
        } else if a.y() > b.y() && a.x() > b.x() {
            Some(b.y() - a.x())
        } else {
            Some(f32::INFINITY)
        }
    }
}
