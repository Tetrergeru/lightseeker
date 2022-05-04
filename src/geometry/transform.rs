use std::cell::RefCell;
use std::rc::Rc;

use crate::geometry::{Matrix, Vector3};

#[derive(Clone)]
pub struct Transform(Rc<RefCell<TransformInternal>>);

impl Transform {
    pub fn new() -> Self {
        Self::from_start(RawTransform::new())
    }

    pub fn from_start(start: RawTransform) -> Self {
        Self(Rc::new(RefCell::new(TransformInternal {
            start,
            raw: RawTransform::new(),
            parent: None,
        })))
    }

    pub fn set_parent(&self, parent: Transform) {
        let mut this = self.0.borrow_mut();
        this.parent = Some(parent);
    }

    pub fn from_xyz_hv(x: f32, y: f32, z: f32, h: f32, v: f32) -> Self {
        let mut t = Self::new();
        t.translate(x, y, z);
        t.rotate_h(h);
        t.rotate_v(v);
        t
    }

    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        let mut t = Self::new();
        t.translate(x, y, z);
        t
    }

    pub fn matrix(&self) -> Matrix {
        self.0.borrow().matrix()
    }

    pub fn reverse_matrix(&self) -> Matrix {
        self.0.borrow().reverse_matrix()
    }

    pub fn normal_matrix(&self) -> Matrix {
        self.0.borrow().normal_matrix()
    }

    pub fn direction(&self) -> Vector3 {
        let this = self.0.borrow();
        this.raw.direction()
    }

    pub fn position(&self) -> Vector3 {
        let this = self.0.borrow();
        this.raw.position()
    }

    pub fn translate(&mut self, dx: f32, dy: f32, dz: f32) {
        let mut this = self.0.borrow_mut();
        this.raw.translate(dx, dy, dz);
    }

    pub fn rotate_h(&mut self, dh: f32) {
        let mut this = self.0.borrow_mut();
        this.raw.rotate_h(dh)
    }

    pub fn rotate_v(&mut self, dv: f32) {
        let mut this = self.0.borrow_mut();
        this.raw.rotate_v(dv)
    }

    pub fn scale(&mut self, scale_factor: f32) {
        let mut this = self.0.borrow_mut();
        this.raw.scale(scale_factor);
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::new()
    }
}

pub struct TransformInternal {
    start: RawTransform,
    raw: RawTransform,
    parent: Option<Transform>,
}

impl TransformInternal {
    pub fn matrix(&self) -> Matrix {
        let mut matrix = self.start.matrix() * self.raw.matrix();
        if let Some(parent) = &self.parent {
            matrix = matrix * parent.matrix();
        }
        matrix
    }

    pub fn reverse_matrix(&self) -> Matrix {
        let mut matrix = self.start.reverse_matrix() * self.raw.reverse_matrix();
        if let Some(parent) = &self.parent {
            matrix = parent.reverse_matrix() * matrix;
        }
        matrix
    }

    pub fn normal_matrix(&self) -> Matrix {
        let mut matrix = self.start.normal_matrix() * self.raw.normal_matrix();
        if let Some(parent) = &self.parent {
            matrix = matrix * parent.normal_matrix();
        }
        matrix
    }
}

pub struct RawTransform {
    position: Vector3,
    scale: f32,
    rotation: Vector3,
}

impl RawTransform {
    pub fn new() -> Self {
        Self {
            position: Vector3::zero(),
            scale: 1.0,
            rotation: Vector3::zero(),
        }
    }

    pub fn from_xyz_hv(x: f32, y: f32, z: f32, h: f32, v: f32) -> Self {
        let mut t = Self::new();
        t.translate(x, y, z);
        t.rotate_h(h);
        t.rotate_v(v);
        t
    }

    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        let mut t = Self::new();
        t.translate(x, y, z);
        t
    }

    pub fn direction(&self) -> Vector3 {
        self.normal_matrix() * Vector3::from_xyz(0.0, 0.0, -1.0)
    }

    pub fn matrix(&self) -> Matrix {
        Matrix::translate(self.position)
            * Matrix::rotation_z(self.rotation.z())
            * Matrix::rotation_x(self.rotation.x())
            * Matrix::rotation_y(self.rotation.y())
            * Matrix::scale(self.scale)
    }

    pub fn reverse_matrix(&self) -> Matrix {
        Matrix::rotation_z(-self.rotation.z())
            * Matrix::rotation_x(-self.rotation.x())
            * Matrix::rotation_y(-self.rotation.y())
            * Matrix::translate(self.position * -1.0)
    }

    pub fn normal_matrix(&self) -> Matrix {
        Matrix::rotation_z(-self.rotation.z())
            * Matrix::rotation_x(-self.rotation.x())
            * Matrix::rotation_y(-self.rotation.y())
    }

    pub fn position(&self) -> Vector3 {
        self.position
    }

    pub fn translate(&mut self, dx: f32, dy: f32, dz: f32) {
        self.position += Vector3::from_xyz(dx, dy, dz);
    }

    pub fn rotate_h(&mut self, dh: f32) {
        self.rotation += Vector3::from_xyz(0.0, dh, 0.0);
    }

    pub fn rotate_v(&mut self, dv: f32) {
        self.rotation += Vector3::from_xyz(dv, 0.0, 0.0);
        log::debug!("RawTransform rotate_v self.rotation: {:?}", self.rotation);
    }

    pub fn scale(&mut self, scale_factor: f32) {
        self.scale *= scale_factor;
    }
}

impl Default for RawTransform {
    fn default() -> Self {
        Self::new()
    }
}
