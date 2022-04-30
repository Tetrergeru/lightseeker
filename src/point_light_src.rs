use std::rc::Rc;

use web_sys::{WebGl2RenderingContext as Gl, WebGlFramebuffer, WebGlTexture};

use crate::{matrix::Matrix, objects::texture::Texture, vector::Vector3};

pub struct PointLightSrc {
    pub position: Vector3,
    pub diffuse: f32,
    pub specular: f32,

    w: u32,
    h: u32,
    matrix: Matrix,
    framebuffer: WebGlFramebuffer,
    texture: Rc<Texture>,
    depth: WebGlTexture,
}

impl PointLightSrc {
    pub fn new(gl: &Gl, point: Vector3) -> Self {
        let w = 2048;
        let h = 2048;

        let texture = Self::create_texture(gl, w, h);
        let depth = Self::create_light_texture(gl, w, h);
        let framebuffer = gl.create_framebuffer().unwrap();

        let mut light = Self {
            position: point,
            matrix: Matrix::zero(),
            framebuffer,
            texture: Rc::new(Texture::from_texture(texture)),
            diffuse: 1.0,
            specular: 1.0,
            w,
            h,
            depth,
        };
        light.eval_matrix();
        light
    }

    fn create_texture(gl: &Gl, w: u32, h: u32) -> WebGlTexture {
        let texture = gl.create_texture().unwrap();

        gl.bind_texture(Gl::TEXTURE_2D, Some(&texture));
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            Gl::TEXTURE_2D,
            0,
            Gl::RGBA as i32,
            w as i32,
            h as i32,
            0,
            Gl::RGBA,
            Gl::UNSIGNED_BYTE,
            None,
        )
        .unwrap();
        gl.tex_parameteri(Gl::TEXTURE_2D, Gl::TEXTURE_MAG_FILTER, Gl::NEAREST as i32);
        gl.tex_parameteri(Gl::TEXTURE_2D, Gl::TEXTURE_MIN_FILTER, Gl::NEAREST as i32);
        gl.tex_parameteri(Gl::TEXTURE_2D, Gl::TEXTURE_WRAP_S, Gl::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(Gl::TEXTURE_2D, Gl::TEXTURE_WRAP_T, Gl::CLAMP_TO_EDGE as i32);

        texture
    }

    fn create_light_texture(gl: &Gl, w: u32, h: u32) -> WebGlTexture {
        let texture = gl.create_texture().unwrap();

        gl.bind_texture(Gl::TEXTURE_2D, Some(&texture));
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            Gl::TEXTURE_2D,
            0,
            Gl::DEPTH_COMPONENT32F as i32,
            w as i32,
            h as i32,
            0,
            Gl::DEPTH_COMPONENT,
            Gl::FLOAT,
            None,
        )
        .unwrap();
        gl.tex_parameteri(Gl::TEXTURE_2D, Gl::TEXTURE_MAG_FILTER, Gl::NEAREST as i32);
        gl.tex_parameteri(Gl::TEXTURE_2D, Gl::TEXTURE_MIN_FILTER, Gl::NEAREST as i32);
        gl.tex_parameteri(Gl::TEXTURE_2D, Gl::TEXTURE_WRAP_S, Gl::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(Gl::TEXTURE_2D, Gl::TEXTURE_WRAP_T, Gl::CLAMP_TO_EDGE as i32);

        texture
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.position += Vector3::from_xyz(x, y, z);
        self.eval_matrix();
    }

    fn eval_matrix(&mut self) {
        self.matrix = Matrix::ident() * Matrix::translate(self.position * -1.0)
    }

    pub fn matrix(&self) -> Matrix {
        self.matrix
    }

    pub fn framebuffer(&self) -> &WebGlFramebuffer {
        &self.framebuffer
    }

    pub fn texture(&self) -> &Rc<Texture> {
        &self.texture
    }

    pub fn position(&self) -> Vector3 {
        self.position
    }

    pub fn bind(&self, gl: &Gl) {
        gl.bind_framebuffer(Gl::FRAMEBUFFER, Some(self.framebuffer()));
        gl.framebuffer_texture_2d(
            Gl::FRAMEBUFFER,
            Gl::DEPTH_ATTACHMENT,
            Gl::TEXTURE_2D,
            Some(&self.depth),
            0,
        );
        gl.framebuffer_texture_2d(
            Gl::FRAMEBUFFER,
            Gl::COLOR_ATTACHMENT0,
            Gl::TEXTURE_2D,
            Some(self.texture().location()),
            0,
        );
        gl.viewport(0, 0, self.w as i32, self.h as i32);
    }

    pub fn viewport(&self, gl: &Gl, flip: f32) {
        if flip > 0.0 {
            gl.viewport(0, 0, self.w as i32, self.h as i32 / 2);
        } else {
            gl.viewport(0, self.h as i32 / 2, self.w as i32, self.h as i32 / 2);
        }
    }
}
