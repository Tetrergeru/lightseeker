use web_sys::{WebGl2RenderingContext as Gl, WebGlProgram, WebGlUniformLocation};

use super::{init_shader_program, uniform_texture};
use crate::{objects::object::Object, light::Directional};

pub struct RenderLight {
    program: WebGlProgram,

    width: i32,
    height: i32,

    vertex_position_location: u32,
    vertex_textcoord_location: u32,

    projection_location: WebGlUniformLocation,
    texture_location: WebGlUniformLocation,
}

const VS_SOURCE: &str = include_str!("src/render_light.vert");
const FS_SOURCE: &str = include_str!("src/render_light.frag");

impl RenderLight {
    pub fn new(gl: &Gl, width: i32, height: i32) -> Self {
        let program = init_shader_program(gl, VS_SOURCE, FS_SOURCE);

        let vertex_position_location = gl.get_attrib_location(&program, "vertexPosition") as u32;
        let vertex_textcoord_location = gl.get_attrib_location(&program, "vertexTexture") as u32;

        let projection_location = gl.get_uniform_location(&program, "projection").unwrap();
        let texture_location = gl.get_uniform_location(&program, "image").unwrap();

        Self {
            program,
            width,
            height,

            vertex_position_location,
            vertex_textcoord_location,

            projection_location,
            texture_location,
        }
    }

    pub fn set_size(&mut self, w: i32, h: i32) {
        self.width = w;
        self.height = h;
    }

    pub fn draw(&self, gl: &Gl, obj: &Object, light: &Directional) {
        gl.use_program(Some(&self.program));

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
            self.vertex_textcoord_location,
            2,
            Gl::FLOAT,
            false,
            obj.shape.step() * 4,
            obj.shape.texture_coord_offset() * 4,
        );
        gl.enable_vertex_attrib_array(self.vertex_textcoord_location);

        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.projection_location),
            true,
            &(light.matrix() * obj.transform_matrix()),
        );

        gl.bind_texture(Gl::TEXTURE_2D, Some(obj.texture.location()));
        uniform_texture(gl, &self.texture_location, obj.texture.location());

        gl.draw_arrays(Gl::TRIANGLES, 0, obj.shape.buffer_length());
    }
}
