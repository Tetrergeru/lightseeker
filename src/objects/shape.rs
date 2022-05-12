use web_sys::{WebGl2RenderingContext, WebGlBuffer};

use crate::{
    geometry::{vector::Vector4, Vector2, Vector3},
    shaders::make_f32_buffer,
};

use super::parsers::{
    shape::{ObjParser, VertexData},
    skinning::Skinning,
};

pub struct Shape {
    vertices: Vec<VertexData>,
    buffer: WebGlBuffer,
}

impl Shape {
    pub fn parse(file: &str, gl: &WebGl2RenderingContext) -> Self {
        Self::new(ObjParser::parse(file), gl)
    }

    pub fn parse_with_skin(file: &str, skin: &Skinning, gl: &WebGl2RenderingContext) -> Self {
        Self::new(ObjParser::parse_with_skin(file, skin), gl)
    }

    fn new(vertices: Vec<VertexData>, gl: &WebGl2RenderingContext) -> Self {
        Self {
            buffer: make_f32_buffer(gl, &Self::make_buffer(&vertices)),
            vertices,
        }
    }

    pub fn get_buffer(&self) -> WebGlBuffer {
        self.buffer.clone()
    }

    pub fn to_f32_vec(&self) -> Box<[f32]> {
        Self::make_buffer(&self.vertices)
    }

    fn make_buffer(vertices: &[VertexData]) -> Box<[f32]> {
        let mut vec_f32 = Vec::with_capacity(vertices.len() * (3 + 3 + 2 + 4 + 4));
        for vertex in vertices.iter() {
            Self::push_vector3(&mut vec_f32, vertex.point);
            Self::push_vector3(&mut vec_f32, vertex.normal);
            Self::push_vector2(&mut vec_f32, vertex.texture_coord);
            Self::push_vector4(&mut vec_f32, vertex.bones);
            Self::push_vector4(&mut vec_f32, vertex.weights);
        }
        vec_f32.into_boxed_slice()
    }

    fn push_vector3(vec_f32: &mut Vec<f32>, vector: Vector3) {
        vec_f32.push(vector.x());
        vec_f32.push(vector.y());
        vec_f32.push(vector.z());
    }

    fn push_vector4(vec_f32: &mut Vec<f32>, vector: Vector4) {
        vec_f32.push(vector.x());
        vec_f32.push(vector.y());
        vec_f32.push(vector.z());
        vec_f32.push(vector.w());
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
        3 + 3
    }

    pub fn bones_coord_offset(&self) -> i32 {
        3 + 3 + 2
    }

    pub fn weights_coord_offset(&self) -> i32 {
        3 + 3 + 2 + 4
    }

    pub fn step(&self) -> i32 {
        3 + 3 + 2 + 4 + 4
    }
}
