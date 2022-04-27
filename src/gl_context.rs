use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext as Gl};

use crate::light_src::LightSrc;
use crate::shaders::render_light::RenderLight;
use crate::shaders::wire_light::WireLight;
use crate::{matrix::Matrix, objects::object::Object, shaders::view::CheckerboardShader};

pub struct GlContext {
    gl: Gl,

    view: CheckerboardShader,
    wire_light: WireLight,
    render_light: RenderLight,
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
            view: CheckerboardShader::new(&gl, w, h),
            wire_light: WireLight::new(&gl, w, h),
            render_light: RenderLight::new(&gl, w, h),
            gl,
        }
    }

    pub fn view(&self, obj: &Object, proj: Matrix, light: &LightSrc) {
        self.view.draw(&self.gl, obj, proj, light);
    }

    pub fn wire_light(&self, light: &LightSrc, proj: Matrix) {
        self.wire_light.draw(&self.gl, proj, light.matrix())
    }

    pub fn render_light(&self, obj: &Object, light: &LightSrc) {
        self.render_light.draw(&self.gl, obj, light)
    }

    pub fn gl(&self) -> Gl {
        self.gl.clone()
    }
}
