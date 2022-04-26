use web_sys::{WebGl2RenderingContext as Gl, WebGlBuffer, WebGlProgram, WebGlUniformLocation};

use crate::{color::Color, vector::Vector2};

use super::{init_shader_program, make_f32_buffer, VS_SOURCE};

pub struct CheckerboardShader {
    program: WebGlProgram,
    buffer: WebGlBuffer,
    buffer_length: i32,

    width: i32,
    height: i32,

    vertex_location: u32,
    cell_size_location: WebGlUniformLocation,
    color_a_location: WebGlUniformLocation,
    color_b_location: WebGlUniformLocation,
}

const FS_SOURCE: &str = include_str!("src/checkerboard.frag");

impl CheckerboardShader {
    pub fn new(gl: &Gl, width: i32, height: i32) -> Self {
        let program = init_shader_program(gl, VS_SOURCE, FS_SOURCE);

        let buffer = make_f32_buffer(
            gl,
            &[
                -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, //
                -1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
            ],
        );
        let buffer_length = 6;
        let vertex_location = gl.get_attrib_location(&program, "vertexPosition") as u32;
        let cell_size_location = gl.get_uniform_location(&program, "cellSize").unwrap();
        let color_a_location = gl.get_uniform_location(&program, "colorA").unwrap();
        let color_b_location = gl.get_uniform_location(&program, "colorB").unwrap();
        Self {
            program,
            buffer,
            buffer_length,
            width,
            height,
            vertex_location,
            cell_size_location,
            color_a_location,
            color_b_location,
        }
    }

    pub fn set_size(&mut self, w: i32, h: i32) {
        self.width = w;
        self.height = h;
    }

    pub fn draw(&self, gl: &Gl, cell_size: Vector2, color_a: Color, color_b: Color) {
        gl.viewport(0, 0, self.width, self.height);

        gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&self.buffer));
        gl.vertex_attrib_pointer_with_i32(self.vertex_location, 2, Gl::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(self.vertex_location);

        gl.use_program(Some(&self.program));

        gl.uniform2f(
            Some(&self.cell_size_location),
            cell_size.x as f32 / self.width as f32,
            cell_size.y as f32 / self.height as f32,
        );

        gl.uniform3f(
            Some(&self.color_a_location),
            color_a.get_r() as f32 / 255.0,
            color_a.get_g() as f32 / 255.0,
            color_a.get_b() as f32 / 255.0,
        );

        gl.uniform3f(
            Some(&self.color_b_location),
            color_b.get_r() as f32 / 255.0,
            color_b.get_g() as f32 / 255.0,
            color_b.get_b() as f32 / 255.0,
        );

        gl.enable(Gl::BLEND);
        gl.blend_func(Gl::SRC_ALPHA, Gl::ONE_MINUS_SRC_ALPHA);

        gl.draw_arrays(Gl::TRIANGLES, 0, self.buffer_length);
        gl.disable(Gl::BLEND);
    }
}
