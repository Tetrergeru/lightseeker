use web_sys::{WebGl2RenderingContext as Gl, WebGlProgram, WebGlUniformLocation};

use super::{init_shader_program, uniform_texture, VS_SOURCE};
use crate::{matrix::Matrix, objects::object::Object};

pub struct CheckerboardShader {
    program: WebGlProgram,

    width: i32,
    height: i32,

    vertex_position_location: u32,
    vertex_normal_location: u32,
    vertex_textcoord_location: u32,

    projection_location: WebGlUniformLocation,
    texture_location: WebGlUniformLocation,
}

const FS_SOURCE: &str = include_str!("src/checkerboard.frag");

impl CheckerboardShader {
    pub fn new(gl: &Gl, width: i32, height: i32) -> Self {
        let program = init_shader_program(gl, VS_SOURCE, FS_SOURCE);

        let vertex_position_location = gl.get_attrib_location(&program, "vertexPosition") as u32;
        let vertex_normal_location = gl.get_attrib_location(&program, "vertexNormal") as u32;
        let vertex_textcoord_location = gl.get_attrib_location(&program, "vertexTexture") as u32;

        let projection_location = gl.get_uniform_location(&program, "projection").unwrap();
        let texture_location = gl.get_uniform_location(&program, "image").unwrap();
        Self {
            program,
            width,
            height,

            vertex_position_location,
            vertex_normal_location,
            vertex_textcoord_location,

            projection_location,
            texture_location,
        }
    }

    pub fn set_size(&mut self, w: i32, h: i32) {
        self.width = w;
        self.height = h;
    }

    pub fn draw(&self, gl: &Gl, obj: &Object, proj: Matrix) {
        gl.use_program(Some(&self.program));

        gl.viewport(0, 0, self.width, self.height);

        gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&obj.shape.get_buffer()));

        gl.vertex_attrib_pointer_with_i32(
            self.vertex_position_location,
            3,
            Gl::FLOAT,
            false,
            obj.shape.step() * 4,
            obj.shape.point_offset() * 4,
        );
        gl.enable_vertex_attrib_array(self.vertex_position_location);

        gl.vertex_attrib_pointer_with_i32(
            self.vertex_normal_location,
            3,
            Gl::FLOAT,
            false,
            obj.shape.step() * 4,
            obj.shape.norm_offset() * 4,
        );
        gl.enable_vertex_attrib_array(self.vertex_normal_location);

        gl.vertex_attrib_pointer_with_i32(
            self.vertex_textcoord_location,
            2,
            Gl::FLOAT,
            false,
            obj.shape.step() * 4,
            obj.shape.texture_coord_offset() * 4,
        );
        gl.enable_vertex_attrib_array(self.vertex_textcoord_location);

        gl.uniform_matrix4fv_with_f32_array(Some(&self.projection_location), true, &proj);

        gl.bind_texture(Gl::TEXTURE_2D, Some(obj.texture.location()));
        uniform_texture(gl, &self.texture_location, obj.texture.location());

        gl.enable(Gl::BLEND);
        gl.blend_func(Gl::SRC_ALPHA, Gl::ONE_MINUS_SRC_ALPHA);

        gl.draw_arrays(Gl::TRIANGLES, 0, obj.shape.buffer_length());
        gl.disable(Gl::BLEND);
    }
}
