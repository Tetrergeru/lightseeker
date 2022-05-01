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
    depth: Rc<Texture>,
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
            depth: Rc::new(Texture::from_texture(depth)),
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

    pub fn depth(&self) -> &Rc<Texture> {
        &self.depth
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
            Some(self.depth.location()),
            0,
        );
        gl.framebuffer_texture_2d(
            Gl::FRAMEBUFFER,
            Gl::COLOR_ATTACHMENT0,
            Gl::TEXTURE_2D,
            Some(self.texture.location()),
            0,
        );
        gl.viewport(0, 0, self.w as i32, self.h as i32);
    }

    pub fn viewport(&self, gl: &Gl, direction: i32) {
        let h = self.h as i32 / 2;
        let w = self.w as i32 / 2;
        match direction {
            0 => gl.viewport(0, 0, w, h),
            1 => gl.viewport(w, 0, w, h),
            2 => gl.viewport(0, h, w, h),
            3 => gl.viewport(w, h, w, h),
            _ => panic!("Direction should be in 0..4"),
        }
    }

    fn fov() -> f32 {
        2.0 * 6.0_f32.sqrt().atan()
    }

    fn aspect() -> f32 {
        2.0 / 3.0_f32.sqrt()
    }

    fn v_angle_0() -> f32 {
        std::f32::consts::PI / 2.0 + Self::fov() * 3.0 / 2.0
    }

    fn v_angle_123() -> f32 {
        std::f32::consts::PI / 2.0 + Self::fov() / 2.0
    }

    fn perspective() -> Matrix {
        // float f = tan(PI * 0.5 - 0.5 * fov());
        // float range_inv = 1.0 / (near - far);
        // return mat4(
        //     f / aspect(), 0.0, 0.0, 0.0,
        //     0.0, f, 0.0, 0.0,
        //     0.0, 0.0, (near + far) * range_inv, -1.0,
        //     0.0, 0.0, near * far * range_inv * 2.0, 0.0
        // );
        let near = 1.0;//6.0_f32.sqrt() / 12.0;
        Matrix::perspective(Self::fov(), Self::aspect(), near, 20.0)
    }

    pub fn matrices(&self) -> Vec<Matrix> {
        use std::f32::consts::PI;
        let h_angles = [0.0, PI / 3.0, PI, -PI / 3.0];
        let v_angles = [
            Self::v_angle_0(),
            Self::v_angle_123(),
            Self::v_angle_123(),
            Self::v_angle_123(),
        ];

        let matrix = |i| {
            Self::perspective()
                * Matrix::rotation_x(v_angles[i])
                * Matrix::rotation_y(h_angles[i])
                * Matrix::translate(self.position * -1.0)
        };

        vec![matrix(0), matrix(1), matrix(2), matrix(3)]
    }
}
