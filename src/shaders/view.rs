use web_sys::{WebGl2RenderingContext as Gl, WebGlProgram, WebGlUniformLocation};

use super::init_shader_program;
use crate::{light_src::LightSrc, matrix::Matrix, objects::object::Object};

pub struct ViewShader {
    program: WebGlProgram,

    width: i32,
    height: i32,

    vertex_position_location: u32,
    vertex_normal_location: u32,
    vertex_textcoord_location: u32,

    camera_location: WebGlUniformLocation,
    position_location: WebGlUniformLocation,
    normal_mat_location: WebGlUniformLocation,
    texture_location: WebGlUniformLocation,
    is_depth_location: WebGlUniformLocation,
    light_location: WebGlUniformLocation,
    lightmap_location: WebGlUniformLocation,
    light_pos_location: WebGlUniformLocation,
    light_dir_location: WebGlUniformLocation,
}

const FS_SOURCE: &str = include_str!("src/view.frag");
const VS_SOURCE: &str = include_str!("src/view.vert");

impl ViewShader {
    pub fn new(gl: &Gl, width: i32, height: i32) -> Self {
        let program = init_shader_program(gl, VS_SOURCE, FS_SOURCE);

        let vertex_position_location = gl.get_attrib_location(&program, "vertexPosition") as u32;
        let vertex_normal_location = gl.get_attrib_location(&program, "vertexNormal") as u32;
        let vertex_textcoord_location = gl.get_attrib_location(&program, "vertexTexture") as u32;

        let camera_location = gl.get_uniform_location(&program, "camera").unwrap();
        let position_location = gl.get_uniform_location(&program, "position").unwrap();
        let normal_mat_location = gl.get_uniform_location(&program, "normalMat").unwrap();
        let texture_location = gl.get_uniform_location(&program, "image").unwrap();
        let is_depth_location = gl.get_uniform_location(&program, "isDepth").unwrap();
        let light_location = gl.get_uniform_location(&program, "light").unwrap();
        let lightmap_location = gl.get_uniform_location(&program, "lightmap").unwrap();
        let light_pos_location = gl.get_uniform_location(&program, "lightLocation").unwrap();
        let light_dir_location = gl.get_uniform_location(&program, "lightDirection").unwrap();
        Self {
            program,
            width,
            height,

            vertex_position_location,
            vertex_normal_location,
            vertex_textcoord_location,

            camera_location,
            position_location,
            normal_mat_location,
            texture_location,
            is_depth_location,
            light_location,
            lightmap_location,
            light_pos_location,
            light_dir_location,
        }
    }

    pub fn set_size(&mut self, w: i32, h: i32) {
        self.width = w;
        self.height = h;
    }

    pub fn draw(&self, gl: &Gl, obj: &Object, proj: Matrix, light: &LightSrc) {
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

        gl.uniform_matrix4fv_with_f32_array(Some(&self.camera_location), true, &proj);

        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.position_location),
            true,
            &obj.transform_matrix(),
        );

        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.normal_mat_location),
            true,
            &obj.normal_matrix(),
        );

        gl.uniform_matrix4fv_with_f32_array(Some(&self.light_location), true, &light.matrix());

        gl.uniform1i(
            Some(&self.is_depth_location),
            if obj.ignored_by_light { 1 } else { 0 },
        );

        gl.uniform3fv_with_f32_array(Some(&self.light_pos_location), &light.position());
        gl.uniform3fv_with_f32_array(Some(&self.light_dir_location), &light.direction());

        gl.active_texture(Gl::TEXTURE0);
        gl.bind_texture(Gl::TEXTURE_2D, Some(obj.texture.location()));
        gl.uniform1i(Some(&self.texture_location), 0);

        gl.active_texture(Gl::TEXTURE1);
        gl.bind_texture(Gl::TEXTURE_2D, Some(light.texture().location()));
        gl.uniform1i(Some(&self.lightmap_location), 1);

        gl.enable(Gl::BLEND);
        gl.blend_func(Gl::SRC_ALPHA, Gl::ONE_MINUS_SRC_ALPHA);

        gl.draw_arrays(Gl::TRIANGLES, 0, obj.shape.buffer_length());
        gl.disable(Gl::BLEND);
    }
}
