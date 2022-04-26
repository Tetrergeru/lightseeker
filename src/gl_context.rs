use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

use crate::{
    color::Color,
    shaders::{checkerboard::CheckerboardShader, copy_image::CopyImageShader},
    vector::Vector2, object::Object, matrix::Matrix,
};

pub struct GlContext {
    canvas: HtmlCanvasElement,
    gl: WebGl2RenderingContext,

    copy_image: CopyImageShader,
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
            copy_image: CopyImageShader::new(
                &gl,
                canvas.width() as i32,
                canvas.height() as i32,
            ),
            checkerboard: CheckerboardShader::new(
                &gl,
                canvas.width() as i32,
                canvas.height() as i32,
            ),
            canvas,
            gl,
        }
    }

    pub fn checkerboard(&self, obj: &mut Object, proj: Matrix, cell_size: f32, color_a: Color, color_b: Color) {
        self.checkerboard.draw(
            &self.gl,
            obj,
            proj,
            Vector2::new(cell_size, cell_size),
            color_a,
            color_b,
        );
    }
}
