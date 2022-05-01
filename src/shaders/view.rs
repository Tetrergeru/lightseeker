use web_sys::{WebGl2RenderingContext as Gl, WebGlProgram, WebGlUniformLocation};

use super::init_shader_program;
use crate::{
    camera::Camera,
    light_src::LightSrc,
    objects::{object::Object},
    point_light_src::PointLightSrc,
};

pub struct ViewShader {
    program: WebGlProgram,

    width: i32,
    height: i32,

    vertex_position_location: u32,
    vertex_normal_location: u32,
    vertex_textcoord_location: u32,

    camera_location: WebGlUniformLocation,
    camera_pos_location: WebGlUniformLocation,
    position_location: WebGlUniformLocation,
    normal_mat_location: WebGlUniformLocation,
    texture_location: WebGlUniformLocation,
    is_depth_location: WebGlUniformLocation,
    light: Vec<LightUniform>,
    point_light_pos: WebGlUniformLocation,
    point_light_map: WebGlUniformLocation,
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
        let camera_pos_location = gl.get_uniform_location(&program, "cameraPosition").unwrap();
        let position_location = gl.get_uniform_location(&program, "position").unwrap();
        let normal_mat_location = gl.get_uniform_location(&program, "normalMat").unwrap();
        let texture_location = gl.get_uniform_location(&program, "image").unwrap();
        let is_depth_location = gl.get_uniform_location(&program, "isDepth").unwrap();

        let mut light = vec![];
        for i in 0..16 {
            light.push(LightUniform::new(gl, &program, &format!("lights[{}]", i)));
        }

        let point_light_pos = gl
            .get_uniform_location(&program, "pointLightPosition")
            .unwrap();
        let point_light_map = gl.get_uniform_location(&program, "pointLightmap").unwrap();

        Self {
            program,
            width,
            height,

            vertex_position_location,
            vertex_normal_location,
            vertex_textcoord_location,

            camera_location,
            camera_pos_location,
            position_location,
            normal_mat_location,
            texture_location,
            is_depth_location,

            light,
            point_light_pos,
            point_light_map,
        }
    }

    pub fn set_size(&mut self, w: i32, h: i32) {
        self.width = w;
        self.height = h;
    }

    pub fn draw(
        &self,
        gl: &Gl,
        obj: &Object,
        camera: &Camera,
        light: &[LightSrc],
        pl: &PointLightSrc,
    ) {
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

        gl.uniform1i(
            Some(&self.is_depth_location),
            if obj.ignored_by_light { 1 } else { 0 },
        );

        if !obj.ignored_by_light {
            let mut texture_id = Gl::TEXTURE1;

            for (i, light) in light.iter().enumerate() {
                self.light[i].bind(gl, light, texture_id as u32);
                log::debug!("View self.light[i].bind({})", texture_id- Gl::TEXTURE0);
                texture_id += 1;
            }
            if light.len() < self.light.len() {
                self.light[light.len()].bind_noting(gl);
            }

            gl.active_texture(texture_id);
            gl.bind_texture(Gl::TEXTURE_2D, Some(pl.depth().location()));
            gl.uniform1i(
                Some(&self.point_light_map),
                (texture_id - Gl::TEXTURE0) as i32,
            );

            gl.uniform3fv_with_f32_array(Some(&self.point_light_pos), &pl.position());
            // texture_id += 1;
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
    diffuse: WebGlUniformLocation,
    specular: WebGlUniformLocation,
    projection: WebGlUniformLocation,
    map: WebGlUniformLocation,
    position: WebGlUniformLocation,
    direction: WebGlUniformLocation,
    fov: WebGlUniformLocation,
}

impl LightUniform {
    fn new(gl: &Gl, program: &WebGlProgram, prefix: &str) -> Self {
        let uniform = |name: &str| {
            gl.get_uniform_location(program, &format!("{}.{}", prefix, name))
                .unwrap()
        };
        Self {
            diffuse: uniform("diffuse"),
            specular: uniform("specular"),
            projection: uniform("projection"),
            map: uniform("map"),
            position: uniform("position"),
            direction: uniform("direction"),
            fov: uniform("fov"),
        }
    }

    fn bind_noting(&self, gl: &Gl) {
        gl.uniform1f(Some(&self.diffuse), -1.0);
    }

    fn bind(&self, gl: &Gl, light: &LightSrc, texture: u32) {
        gl.uniform_matrix4fv_with_f32_array(Some(&self.projection), true, &light.matrix());
        gl.uniform3fv_with_f32_array(Some(&self.position), &light.position());
        gl.uniform3fv_with_f32_array(Some(&self.direction), &light.direction());
        gl.uniform1f(Some(&self.diffuse), light.diffuse);
        gl.uniform1f(Some(&self.specular), light.specular);
        gl.uniform1f(Some(&self.fov), light.fov);

        gl.active_texture(texture);
        gl.bind_texture(Gl::TEXTURE_2D, Some(light.depth().location()));
        gl.uniform1i(Some(&self.map), (texture - Gl::TEXTURE0) as i32);
    }
}
