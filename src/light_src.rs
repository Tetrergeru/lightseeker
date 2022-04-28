use std::rc::Rc;

use web_sys::{WebGl2RenderingContext as Gl, WebGlFramebuffer, WebGlTexture};

use crate::{matrix::Matrix, objects::texture::Texture, vector::Vector3};

pub struct LightSrc {
    pub position: Vector3,
    pub angle_h: f32,
    pub angle_v: f32,

    w: u32,
    h: u32,
    matrix: Matrix,
    framebuffer: WebGlFramebuffer,
    texture: Rc<Texture>,
}

impl LightSrc {
    pub fn new(gl: &Gl, point: Vector3, angle_h: f32, angle_v: f32) -> Self {
        let w = 2048;
        let h = 2048;

        let texture = Self::create_light_texture(gl, w, h);
        let framebuffer = gl.create_framebuffer().unwrap();

        let mut light = Self {
            position: point,
            angle_h,
            angle_v,
            matrix: Matrix::zero(),
            framebuffer,
            texture: Rc::new(Texture::from_texture(texture)),
            w,
            h,
        };
        light.eval_matrix();
        light
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

    fn eval_matrix(&mut self) {
        self.matrix = Matrix::ident()
            * Matrix::perspective(std::f32::consts::PI / 2.0, 1.0, 1.0, 20.0)
            * Matrix::rotation_x(self.angle_v)
            * Matrix::rotation_y(self.angle_h)
            * Matrix::translate(self.position)
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

    pub fn direction(&self) -> Vector3 {
        Matrix::ident()
            * Matrix::rotation_x(self.angle_v)
            * Matrix::rotation_y(self.angle_h)
            * Vector3::from_xyz(0.0, 0.0, 1.0)
    }

    pub fn bind(&self, gl: &Gl) {
        gl.bind_framebuffer(Gl::FRAMEBUFFER, Some(self.framebuffer()));
        gl.framebuffer_texture_2d(
            Gl::FRAMEBUFFER,
            Gl::DEPTH_ATTACHMENT,
            Gl::TEXTURE_2D,
            Some(self.texture().location()),
            0,
        );
        gl.viewport(0, 0, self.w as i32, self.h as i32);
    }
}
