use web_sys::{WebGl2RenderingContext as Gl, WebGlProgram, WebGlUniformLocation};

use super::init_shader_program;
use crate::{
    camera::Camera,
    geometry::Matrix,
    light::{Directional, Light, Point},
    objects::object::Object,
};

pub struct ViewShader {
    program: WebGlProgram,

    width: i32,
    height: i32,

    vertex_position_location: u32,
    vertex_normal_location: u32,
    vertex_textcoord_location: u32,
    vertex_bones_location: u32,
    vertex_weights_location: u32,

    camera_location: WebGlUniformLocation,
    camera_pos_location: WebGlUniformLocation,
    position_location: WebGlUniformLocation,
    normal_mat_location: WebGlUniformLocation,
    texture_location: WebGlUniformLocation,
    ignore_light_location: WebGlUniformLocation,
    light: Vec<LightUniform>,
    bones_locations: Vec<WebGlUniformLocation>,
    bones_count_location: WebGlUniformLocation,
}

const FS_SOURCE: &str = include_str!("src/view.frag");
const VS_SOURCE: &str = include_str!("src/view.vert");

const MAX_LIGHTS: usize = 16;
const MAX_BONES: usize = 32;

impl ViewShader {
    pub fn new(gl: &Gl, width: i32, height: i32) -> Self {
        let program = init_shader_program(gl, VS_SOURCE, FS_SOURCE);

        let vertex_position_location = gl.get_attrib_location(&program, "vertexPosition") as u32;
        let vertex_normal_location = gl.get_attrib_location(&program, "vertexNormal") as u32;
        let vertex_textcoord_location = gl.get_attrib_location(&program, "vertexTexture") as u32;
        let vertex_bones_location = gl.get_attrib_location(&program, "vertexBones") as u32;
        let vertex_weights_location = gl.get_attrib_location(&program, "vertexWeights") as u32;

        let camera_location = gl.get_uniform_location(&program, "camera").unwrap();
        let camera_pos_location = gl.get_uniform_location(&program, "cameraPosition").unwrap();
        let position_location = gl.get_uniform_location(&program, "position").unwrap();
        let normal_mat_location = gl.get_uniform_location(&program, "normalMat").unwrap();
        let texture_location = gl.get_uniform_location(&program, "textureMap").unwrap();

        let bones_count_location = gl.get_uniform_location(&program, "boneCount").unwrap();
        let mut bones_locations = vec![];
        for i in 0..MAX_BONES {
            bones_locations.push(
                gl.get_uniform_location(&program, &format!("bones[{}]", i))
                    .unwrap(),
            )
        }

        let ignore_light_location = gl.get_uniform_location(&program, "ignoreLight").unwrap();
        let mut light = vec![];
        for i in 0..MAX_LIGHTS {
            light.push(LightUniform::new(gl, &program, &format!("lights[{}]", i)));
        }

        Self {
            program,
            width,
            height,

            vertex_position_location,
            vertex_normal_location,
            vertex_textcoord_location,
            vertex_bones_location,
            vertex_weights_location,

            camera_location,
            camera_pos_location,
            position_location,
            normal_mat_location,
            texture_location,

            bones_locations,
            bones_count_location,

            ignore_light_location,
            light,
        }
    }

    pub fn set_size(&mut self, w: i32, h: i32) {
        self.width = w;
        self.height = h;
    }

    pub fn draw(&self, gl: &Gl, obj: &Object, camera: &Camera, light: &[Light]) {
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

        gl.uniform_matrix4fv_with_f32_array(Some(&self.camera_location), true, &camera.matrix());
        gl.uniform3fv_with_f32_array(Some(&self.camera_pos_location), &camera.position);

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

        if !obj.ignored_by_light {
            gl.uniform1i(Some(&self.ignore_light_location), 0);

            let mut texture_id = Gl::TEXTURE1;
            for (i, light) in light.iter().enumerate() {
                self.light[i].bind(gl, light, texture_id as u32);
                texture_id += 1;
            }
            if light.len() < self.light.len() {
                self.light[light.len()].bind_noting(gl);
            }
        } else {
            gl.uniform1i(Some(&self.ignore_light_location), 1);
        }

        gl.uniform1i(Some(&self.bones_count_location), obj.skeleton.len() as i32);
        gl.uniform_matrix4fv_with_f32_array(Some(&self.bones_locations[0]), true, &Matrix::ident());
        for (i, bone) in obj.skeleton.iter().enumerate() {
            gl.uniform_matrix4fv_with_f32_array(
                Some(&self.bones_locations[i + 1]),
                true,
                &bone.matrix(),
            );
        }

        gl.active_texture(Gl::TEXTURE0);
        gl.bind_texture(Gl::TEXTURE_2D, Some(obj.texture.location()));
        gl.uniform1i(Some(&self.texture_location), 0);

        gl.enable(Gl::BLEND);
        gl.blend_func(Gl::SRC_ALPHA, Gl::ONE_MINUS_SRC_ALPHA);

        gl.draw_arrays(Gl::TRIANGLES, 0, obj.shape.buffer_length());
        gl.disable(Gl::BLEND);
    }
}

struct LightUniform {
    typ: WebGlUniformLocation,
    diffuse: WebGlUniformLocation,
    specular: WebGlUniformLocation,
    projection: [WebGlUniformLocation; 4],
    map: WebGlUniformLocation,
    position: WebGlUniformLocation,
    direction: WebGlUniformLocation,
    fov: WebGlUniformLocation,
    inner_fov: WebGlUniformLocation,
    color: WebGlUniformLocation,
}

impl LightUniform {
    fn new(gl: &Gl, program: &WebGlProgram, prefix: &str) -> Self {
        let uniform = |name: &str| {
            gl.get_uniform_location(program, &format!("{}.{}", prefix, name))
                .unwrap()
        };
        Self {
            typ: uniform("type"),
            diffuse: uniform("diffuse"),
            specular: uniform("specular"),
            projection: [
                uniform("projection[0]"),
                uniform("projection[1]"),
                uniform("projection[2]"),
                uniform("projection[3]"),
            ],
            map: uniform("map"),
            position: uniform("position"),
            direction: uniform("direction"),
            fov: uniform("fov"),
            inner_fov: uniform("innerFov"),
            color: uniform("color"),
        }
    }

    fn bind_noting(&self, gl: &Gl) {
        gl.uniform1i(Some(&self.typ), -1);
    }

    fn bind(&self, gl: &Gl, light: &Light, texture: u32) {
        gl.uniform1f(Some(&self.diffuse), light.diffuse());
        gl.uniform1f(Some(&self.specular), light.specular());

        gl.active_texture(texture);
        gl.bind_texture(Gl::TEXTURE_2D, Some(light.depth().location()));
        gl.uniform1i(Some(&self.map), (texture - Gl::TEXTURE0) as i32);
        gl.uniform3fv_with_f32_array(Some(&self.color), &light.color());

        match light {
            Light::Directional(d) => self.bind_direction_specific(gl, d),
            Light::Point(p) => self.bind_point_specific(gl, p),
        }
    }

    fn bind_point_specific(&self, gl: &Gl, light: &Point) {
        gl.uniform1i(Some(&self.typ), 1);

        let light_matrices = light.matrices();
        for (i, m) in light_matrices.iter().enumerate() {
            gl.uniform_matrix4fv_with_f32_array(Some(&self.projection[i]), true, m);
        }
        gl.uniform3fv_with_f32_array(Some(&self.position), &light.position());
    }

    fn bind_direction_specific(&self, gl: &Gl, light: &Directional) {
        gl.uniform1i(Some(&self.typ), 0);

        gl.uniform_matrix4fv_with_f32_array(Some(&self.projection[0]), true, &light.matrix());
        gl.uniform3fv_with_f32_array(Some(&self.position), &light.position());
        gl.uniform3fv_with_f32_array(Some(&self.direction), &light.direction());
        gl.uniform1f(Some(&self.fov), light.fov);
        gl.uniform1f(Some(&self.inner_fov), light.inner_fov);
    }
}
