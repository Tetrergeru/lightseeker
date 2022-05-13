use web_sys::{WebGl2RenderingContext as Gl, WebGlBuffer, WebGlProgram, WebGlUniformLocation};

use super::init_shader_program;
use crate::{geometry::{Matrix, Vector3}, shaders::make_f32_buffer};

pub struct WireLight {
    program: WebGlProgram,

    width: i32,
    height: i32,

    vertex_position_location: u32,

    projection_location: WebGlUniformLocation,
    light_location: WebGlUniformLocation,
    color_location: WebGlUniformLocation,

    buffer: WebGlBuffer,
}

const VS_SOURCE: &str = include_str!("src/wire_light.vert");
const FS_SOURCE: &str = include_str!("src/wire_light.frag");

impl WireLight {
    pub fn new(gl: &Gl, width: i32, height: i32) -> Self {
        let program = init_shader_program(gl, VS_SOURCE, FS_SOURCE);

        let vertex_position_location = gl.get_attrib_location(&program, "vertexPosition") as u32;

        let projection_location = gl.get_uniform_location(&program, "projection").unwrap();
        let light_location = gl.get_uniform_location(&program, "light").unwrap();
        let color_location = gl.get_uniform_location(&program, "color").unwrap();

        let cube_lines: Vec<f32> = vec![
            // bottom
            -1.0, -1.0, -1.0, //
            1.0, -1.0, -1.0, //
            -1.0, -1.0, -1.0, //
            -1.0, -1.0, 1.0, //
            1.0, -1.0, 1.0, //
            1.0, -1.0, -1.0, //
            1.0, -1.0, 1.0, //
            -1.0, -1.0, 1.0, //
            // top
            -1.0, 1.0, -1.0, //
            1.0, 1.0, -1.0, //
            -1.0, 1.0, -1.0, //
            -1.0, 1.0, 1.0, //
            1.0, 1.0, 1.0, //
            1.0, 1.0, -1.0, //
            1.0, 1.0, 1.0, //
            -1.0, 1.0, 1.0, //
            // sides
            -1.0, -1.0, -1.0, //
            -1.0, 1.0, -1.0, //
            1.0, -1.0, -1.0, //
            1.0, 1.0, -1.0, //
            -1.0, -1.0, 1.0, //
            -1.0, 1.0, 1.0, //
            1.0, -1.0, 1.0, //
            1.0, 1.0, 1.0, //
        ];

        Self {
            program,
            width,
            height,

            vertex_position_location,

            projection_location,
            light_location,
            color_location,

            buffer: make_f32_buffer(gl, &cube_lines),
        }
    }

    pub fn set_size(&mut self, w: i32, h: i32) {
        self.width = w;
        self.height = h;
    }

    pub fn draw(&self, gl: &Gl, proj: Matrix, light: Matrix, color: Vector3) {
        gl.use_program(Some(&self.program));

        gl.viewport(0, 0, self.width, self.height);

        gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&self.buffer));

        gl.vertex_attrib_pointer_with_i32(self.vertex_position_location, 3, Gl::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(self.vertex_position_location);

        gl.uniform_matrix4fv_with_f32_array(Some(&self.projection_location), true, &proj);
        gl.uniform_matrix4fv_with_f32_array(Some(&self.light_location), true, &light);
        gl.uniform3fv_with_f32_array(Some(&self.color_location), &color);

        gl.enable(Gl::BLEND);
        gl.blend_func(Gl::SRC_ALPHA, Gl::ONE_MINUS_SRC_ALPHA);

        gl.draw_arrays(Gl::LINES, 0, 24);
        gl.disable(Gl::BLEND);
    }
}
