use web_sys::{WebGl2RenderingContext as Gl, WebGlProgram, WebGlUniformLocation};

use super::init_shader_program;
use crate::{geometry::Matrix, objects::object::Object};

pub struct RenderLight {
    program: WebGlProgram,

    width: i32,
    height: i32,

    vertex_position_location: u32,
    vertex_bones_location: u32,
    vertex_weights_location: u32,

    projection_location: WebGlUniformLocation,

    bones_count_location: WebGlUniformLocation,
    bones_locations: Vec<WebGlUniformLocation>,
}

const VS_SOURCE: &str = include_str!("src/render_light.vert");
const FS_SOURCE: &str = include_str!("src/render_light.frag");

const MAX_BONES: usize = 100;

impl RenderLight {
    pub fn new(gl: &Gl, width: i32, height: i32) -> Self {
        let program = init_shader_program(gl, VS_SOURCE, FS_SOURCE);

        let vertex_position_location = gl.get_attrib_location(&program, "vertexPosition") as u32;
        let vertex_bones_location = gl.get_attrib_location(&program, "vertexBones") as u32;
        let vertex_weights_location = gl.get_attrib_location(&program, "vertexWeights") as u32;

        let projection_location = gl.get_uniform_location(&program, "projection").unwrap();

        let bones_count_location = gl.get_uniform_location(&program, "boneCount").unwrap();
        let bones_locations = (0..MAX_BONES)
            .map(|i| {
                gl.get_uniform_location(&program, &format!("bones[{}]", i))
                    .unwrap()
            })
            .collect();

        Self {
            program,
            width,
            height,

            vertex_position_location,
            vertex_bones_location,
            vertex_weights_location,

            projection_location,
            bones_count_location,
            bones_locations,
        }
    }

    pub fn set_size(&mut self, w: i32, h: i32) {
        self.width = w;
        self.height = h;
    }

    pub fn draw(&self, gl: &Gl, obj: &Object, light: Matrix) {
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
            self.vertex_bones_location,
            4,
            Gl::FLOAT,
            false,
            obj.shape.step() * 4,
            obj.shape.bones_coord_offset() * 4,
        );
        gl.enable_vertex_attrib_array(self.vertex_bones_location);

        gl.vertex_attrib_pointer_with_i32(
            self.vertex_weights_location,
            4,
            Gl::FLOAT,
            false,
            obj.shape.step() * 4,
            obj.shape.weights_coord_offset() * 4,
        );
        gl.enable_vertex_attrib_array(self.vertex_weights_location);

        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.projection_location),
            true,
            &(light * obj.transform_matrix()),
        );

        gl.uniform1i(Some(&self.bones_count_location), obj.skeleton.len() as i32);
        gl.uniform_matrix4fv_with_f32_array(Some(&self.bones_locations[0]), true, &Matrix::ident());

        if obj.has_skeleton() {
            for (i, bone) in obj.bone_matrices().enumerate() {
                gl.uniform_matrix4fv_with_f32_array(
                    Some(&self.bones_locations[i + 1]),
                    true,
                    &bone,
                );
            }
        }

        gl.draw_arrays(Gl::TRIANGLES, 0, obj.shape.buffer_length());
    }
}
