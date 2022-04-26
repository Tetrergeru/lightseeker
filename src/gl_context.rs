use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

use crate::{matrix::Matrix, objects::object::Object, shaders::checkerboard::CheckerboardShader};

pub struct GlContext {
    gl: WebGl2RenderingContext,

    checkerboard: CheckerboardShader,
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
        Self {
            checkerboard: CheckerboardShader::new(
                &gl,
                canvas.width() as i32,
                canvas.height() as i32,
            ),
            gl,
        }
    }

    pub fn checkerboard(&self, obj: &Object, proj: Matrix) {
        self.checkerboard.draw(&self.gl, obj, proj);
    }

    pub fn gl(&self) -> WebGl2RenderingContext {
        self.gl.clone()
    }
}
