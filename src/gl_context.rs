use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext as Gl};

use crate::camera::Camera;
use crate::light_src::LightSrc;
use crate::point_light_src::PointLightSrc;
use crate::shaders::render_light::RenderLight;
use crate::shaders::render_point_light::RenderPointLight;
use crate::shaders::wire_light::WireLight;
use crate::vector::Vector3;
use crate::{matrix::Matrix, objects::object::Object, shaders::view::ViewShader};

pub struct GlContext {
    gl: Gl,

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
            view: ViewShader::new(&gl, w, h),
            wire_light: WireLight::new(&gl, w, h),
            render_light: RenderLight::new(&gl, w, h),
            render_point_light: RenderPointLight::new(&gl, w, h),
            gl,
        }
    }

    pub fn view(&self, obj: &Object, camera: &Camera, light: &[LightSrc], pl: &PointLightSrc) {
        self.view.draw(&self.gl, obj, camera, light, pl);
    }

    pub fn wire_light(&self, light: Matrix, proj: Matrix) {
        self.wire_light.draw(&self.gl, proj, light)
    }

    pub fn render_light(&self, obj: &Object, light: &LightSrc) {
        self.render_light.draw(&self.gl, obj, light)
    }

    pub fn render_point_light(&self, obj: &Object, light: Vector3, direction: i32) {
        self.render_point_light.draw(&self.gl, obj, light, direction)
    }

    pub fn gl(&self) -> Gl {
        self.gl.clone()
    }
}
