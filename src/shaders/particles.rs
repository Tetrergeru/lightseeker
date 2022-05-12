use web_sys::{WebGl2RenderingContext as Gl, WebGlProgram, WebGlUniformLocation};

use super::init_shader_program;
use crate::{camera::Camera, objects::particles::Particles};

pub struct ParticlesShader {
    program: WebGlProgram,

    width: i32,
    height: i32,

    projection_location: WebGlUniformLocation,
    verts_in_particle_location: WebGlUniformLocation,
    verts_location: WebGlUniformLocation,
    texture_location: WebGlUniformLocation,

    transforms_locations: Vec<WebGlUniformLocation>,
}

const VS_SOURCE: &str = include_str!("src/particles.vert");
const FS_SOURCE: &str = include_str!("src/particles.frag");

const MAX_PARTICLES: usize = 32;

impl ParticlesShader {
    pub fn new(gl: &Gl, width: i32, height: i32) -> Self {
        let program = init_shader_program(gl, VS_SOURCE, FS_SOURCE);

        let projection_location = gl.get_uniform_location(&program, "projection").unwrap();
        let verts_in_particle_location = gl
            .get_uniform_location(&program, "vertsInParticle")
            .unwrap();
        let verts_location = gl.get_uniform_location(&program, "verts").unwrap();
        let texture_location = gl.get_uniform_location(&program, "image").unwrap();

        let transforms_locations = (0..MAX_PARTICLES)
            .map(|i| {
                gl.get_uniform_location(&program, &format!("transforms[{}]", i))
                    .unwrap()
            })
            .collect();

        Self {
            program,
            width,
            height,

            projection_location,
            verts_in_particle_location,
            texture_location,
            verts_location,

            transforms_locations,
        }
    }

    pub fn set_size(&mut self, w: i32, h: i32) {
        self.width = w;
        self.height = h;
    }

    pub fn draw(&self, gl: &Gl, particles: &Particles, camera: &Camera) {
        gl.use_program(Some(&self.program));
        gl.viewport(0, 0, self.width, self.height);

        gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&particles.shape.get_buffer()));

        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.projection_location),
            true,
            &(camera.matrix() * particles.transform.matrix()),
        );

        gl.uniform1i(Some(&self.verts_in_particle_location), particles.vertices());
        gl.uniform1i(Some(&self.verts_location), particles.particles.len() as i32);

        gl.active_texture(Gl::TEXTURE0);
        gl.bind_texture(Gl::TEXTURE_2D, Some(particles.texture.location()));
        gl.uniform1i(Some(&self.texture_location), 0);

        gl.active_texture(Gl::TEXTURE1);
        gl.bind_texture(Gl::TEXTURE_2D, Some(particles.buffer.location()));
        gl.uniform1i(Some(&self.verts_location), 1);

        for (i, particle) in particles.particles.iter().enumerate() {
            gl.uniform_matrix4fv_with_f32_array(
                Some(&self.transforms_locations[i]),
                true,
                &particle.matrix(),
            );
        }

        gl.draw_arrays(
            Gl::TRIANGLES,
            0,
            particles.particles.len() as i32 * particles.vertices(),
        );
    }
}
