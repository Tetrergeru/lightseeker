use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext as Gl, WebGlFramebuffer};

use crate::{
    camera::Camera,
    geometry::Matrix,
    light::{Directional, Light, Point},
    objects::object::Object,
    shaders::{
        render_light::RenderLight, render_point_light::RenderPointLight, view::ViewShader,
        wire_light::WireLight,
    },
};

pub struct GlContext {
    gl: Gl,
    framebuffer: WebGlFramebuffer,

    view: ViewShader,
    wire_light: WireLight,
    render_light: RenderLight,
    render_point_light: RenderPointLight,
}

impl GlContext {
    pub fn new(canvas: HtmlCanvasElement, width: u32, height: u32) -> Self {
        canvas.set_width(width);
        canvas.set_height(height);
        let gl = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<Gl>()
            .unwrap();
        gl.enable(Gl::DEPTH_TEST);
        let w = canvas.width() as i32;
        let h = canvas.height() as i32;
        Self {
            framebuffer: gl.create_framebuffer().unwrap(),
            view: ViewShader::new(&gl, w, h),
            wire_light: WireLight::new(&gl, w, h),
            render_light: RenderLight::new(&gl, w, h),
            render_point_light: RenderPointLight::new(&gl, w, h),
            gl,
        }
    }

    pub fn view(&self, obj: &Object, camera: &Camera, light: &[Light]) {
        self.view.draw(&self.gl, obj, camera, light);
    }

    pub fn wire_light(&self, light: Matrix, proj: Matrix) {
        self.wire_light.draw(&self.gl, proj, light)
    }

    pub fn bind_framebuffer(&self, light: &Light) {
        self.gl
            .bind_framebuffer(Gl::FRAMEBUFFER, Some(&self.framebuffer));
        self.gl.framebuffer_texture_2d(
            Gl::FRAMEBUFFER,
            Gl::DEPTH_ATTACHMENT,
            Gl::TEXTURE_2D,
            Some(light.depth().location()),
            0,
        );
        if let Light::Point(p) = light {
            self.gl.framebuffer_texture_2d(
                Gl::FRAMEBUFFER,
                Gl::COLOR_ATTACHMENT0,
                Gl::TEXTURE_2D,
                Some(p.texture.location()),
                0,
            );
        }
        let (w, h) = light.texture_bounds();
        self.gl.viewport(0, 0, w as i32, h as i32);
    }

    pub fn unbind_framebuffer(&self) {
        self.gl.bind_framebuffer(Gl::FRAMEBUFFER, None);
    }

    pub fn render_light(&self, obj: &Object, light: &Light) {
        match light {
            Light::Directional(d) => self.render_directional_light(obj, d),
            Light::Point(p) => self.render_point_light(obj, p),
        }
    }

    pub fn render_directional_light(&self, obj: &Object, light: &Directional) {
        self.render_light.draw(&self.gl, obj, light)
    }

    pub fn render_point_light(&self, obj: &Object, light: &Point) {
        for (i, m) in light.matrices().into_iter().enumerate() {
            self.point_light_viewport(light, i);
            self.render_point_light.draw(&self.gl, obj, m);
        }
    }

    pub fn point_light_viewport(&self, light: &Point, direction: usize) {
        let (h, w) = light.texture_bounds();
        let h = h as i32 / 2;
        let w = w as i32 / 2;
        match direction {
            0 => self.gl.viewport(0, 0, w, h),
            1 => self.gl.viewport(w, 0, w, h),
            2 => self.gl.viewport(0, h, w, h),
            3 => self.gl.viewport(w, h, w, h),
            _ => panic!("Direction should be in 0..4"),
        }
    }

    pub fn gl(&self) -> Gl {
        self.gl.clone()
    }

    pub fn clear(&self) {
        self.gl.clear(Gl::COLOR_BUFFER_BIT | Gl::DEPTH_BUFFER_BIT);
    }
}
