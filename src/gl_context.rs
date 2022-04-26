use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

use crate::{
    color::Color,
    shaders::{checkerboard::CheckerboardShader, copy_image::CopyImageShader},
    vector::Vector2,
};

pub struct GlContext {
    canvas: HtmlCanvasElement,
    context: WebGl2RenderingContext,

    copy_image: CopyImageShader,
    checkerboard: CheckerboardShader,
}

impl GlContext {
    pub fn new(canvas: HtmlCanvasElement, width: u32, height: u32) -> Self {
        canvas.set_width(width);
        canvas.set_height(height);
        let context = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()
            .unwrap();
        Self {
            copy_image: CopyImageShader::new(
                &context,
                canvas.width() as i32,
                canvas.height() as i32,
            ),
            checkerboard: CheckerboardShader::new(
                &context,
                canvas.width() as i32,
                canvas.height() as i32,
            ),
            canvas,
            context,
        }
    }

    pub fn checkerboard(&self, cell_size: f64, color_a: Color, color_b: Color) {
        self.checkerboard.draw(
            &self.context,
            Vector2::new(cell_size as f64, cell_size as f64),
            color_a,
            color_b,
        );
    }
}
