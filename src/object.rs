use web_sys::{WebGl2RenderingContext, WebGlBuffer};

use crate::{shaders::make_vector4_buffer, vector::Vector4};

pub struct Object {
    triangles: Vec<Vector4>,
    buffer: Option<WebGlBuffer>,
}

impl Object {
    pub fn cube() -> Self {
        let p000 = Vector4::from_xyz(-1.0, -1.0, -1.0);
        let p111 = Vector4::from_xyz(1.0, 1.0, 1.0);
        let p001 = Vector4::from_xyz(p000.x(), p000.y(), p111.z());
        let p010 = Vector4::from_xyz(p000.x(), p111.y(), p000.z());
        let p100 = Vector4::from_xyz(p111.x(), p000.y(), p000.z());
        let p011 = Vector4::from_xyz(p000.x(), p111.y(), p111.z());
        let p101 = Vector4::from_xyz(p111.x(), p000.y(), p111.z());
        let p110 = Vector4::from_xyz(p111.x(), p111.y(), p000.z());

        let triangles: Vec<Vector4> = vec![
            p000, p100, p010, //
            p100, p110, p010, //
            p010, p110, p011, //
            p110, p111, p011, //
            p110, p100, p111, //
            p100, p101, p111, //
            p001, p101, p000, //
            p101, p100, p000, //
            p000, p001, p010, // reverse?
            p001, p011, p010, // reverse?
            p011, p111, p001, //
            p111, p101, p001, //
        ];
        Self {
            triangles,
            buffer: None,
        }
    }

    pub fn make_buffer(&mut self, gl: &WebGl2RenderingContext) -> WebGlBuffer {
        if let Some(buffer) = &self.buffer {
            return buffer.clone();
        }
        let buffer = make_vector4_buffer(gl, &self.triangles);
        self.buffer = Some(buffer.clone());
        buffer
    }

    pub fn buffer_length(&self) -> i32 {
        self.triangles.len() as i32
    }
}
