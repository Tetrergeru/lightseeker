use super::Vector3;

pub struct Ray {
    pub point: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(p: Vector3, d: Vector3) -> Self {
        Self {
            point: p,
            direction: d,
        }
    }
}

pub struct Plane {
    pub point: Vector3,
    pub u: Vector3,
    pub v: Vector3,
}

impl Plane {
    pub fn new(p: Vector3, u: Vector3, v: Vector3) -> Self {
        Self { point: p, u, v }
    }
}

pub fn cast_ray(ray: &Ray, plane: &Plane) -> Option<(f32, Vector3)> {
    let p = ray.point;
    let s = plane.point;
    let r = p - s;

    let d = -ray.direction;
    let u = plane.u;
    let v = plane.v;

    let delta = det(u, v, d);

    let u_0 = det(r, v, d) / delta;
    let v_0 = det(u, r, d) / delta;
    let d_0 = det(u, v, r) / delta;

    if (0.0..1.0).contains(&u_0) && (0.0..1.0).contains(&v_0) {
        Some((d_0, p + d * (-d_0)))
    } else {
        None
    }
}

pub fn det(a: Vector3, b: Vector3, c: Vector3) -> f32 {
    let d1 = a.x() * (b.y() * c.z() - b.z() * c.y());
    let d2 = b.x() * (a.y() * c.z() - a.z() * c.y());
    let d3 = c.x() * (a.y() * b.z() - a.z() * b.y());
    d1 + d2 + d3
}
