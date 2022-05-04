use web_sys::{WebGl2RenderingContext, WebGlBuffer};

use crate::{
    geometry::{Vector2, Vector3},
    shaders::make_f32_buffer,
};

use super::parsers::shape::{ObjParser, VertexData};

pub struct Shape {
    vertices: Vec<VertexData>,
    buffer: WebGlBuffer,
}

impl Shape {
    pub fn parse_obj_file(file: &str, gl: &WebGl2RenderingContext) -> Self {
        Self::new(ObjParser::parse(file), gl)
    }

    fn new(vertices: Vec<VertexData>, gl: &WebGl2RenderingContext) -> Self {
        Self {
            buffer: Self::make_buffer(&vertices, gl),
            vertices,
        }
    }

    pub fn get_buffer(&self) -> WebGlBuffer {
        self.buffer.clone()
    }

    fn make_buffer(vertices: &[VertexData], gl: &WebGl2RenderingContext) -> WebGlBuffer {
        let mut vec_f32 = Vec::with_capacity(vertices.len() * (3 + 3 + 2));
        for vertex in vertices.iter() {
            Self::push_vector3(&mut vec_f32, vertex.point);
            Self::push_vector3(&mut vec_f32, vertex.normal);
            Self::push_vector2(&mut vec_f32, vertex.texture_coord);
        }
        make_f32_buffer(gl, &vec_f32)
    }

    fn push_vector3(vec_f32: &mut Vec<f32>, vector: Vector3) {
        vec_f32.push(vector.x());
        vec_f32.push(vector.y());
        vec_f32.push(vector.z());
    }

    fn push_vector2(vec_f32: &mut Vec<f32>, vector: Vector2) {
        vec_f32.push(vector.x());
        vec_f32.push(vector.y());
    }

    pub fn buffer_length(&self) -> i32 {
        self.vertices.len() as i32
    }

    pub fn point_offset(&self) -> i32 {
        0
    }

    pub fn norm_offset(&self) -> i32 {
        3
    }

    pub fn texture_coord_offset(&self) -> i32 {
        6
    }

    pub fn step(&self) -> i32 {
        3 + 3 + 2
    }
}
