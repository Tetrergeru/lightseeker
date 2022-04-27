use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

use crate::light_src::LightSrc;
use crate::shaders::wire_light::WireLight;
use crate::{matrix::Matrix, objects::object::Object, shaders::checkerboard::CheckerboardShader};

pub struct GlContext {
    gl: WebGl2RenderingContext,

    checkerboard: CheckerboardShader,
    wire_light: WireLight,
}

impl GlContext {
    pub fn new(canvas: HtmlCanvasElement, width: u32, height: u32) -> Self {
        canvas.set_width(width);
        canvas.set_height(height);
        let gl = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()
            .unwrap();
        gl.enable(WebGl2RenderingContext::DEPTH_TEST);
        let w = canvas.width() as i32;
        let h = canvas.height() as i32;
        Self {
            checkerboard: CheckerboardShader::new(&gl, w, h),
            wire_light: WireLight::new(&gl, w, h),
            gl,
        }
    }

    pub fn checkerboard(&self, obj: &Object, proj: Matrix) {
        self.checkerboard.draw(&self.gl, obj, proj);
    }

    pub fn wire_light(&self, light: &LightSrc, proj: Matrix) {
        self.wire_light.draw(&self.gl, proj, light.matrix())
    }

    pub fn gl(&self) -> WebGl2RenderingContext {
        self.gl.clone()
    }
}
