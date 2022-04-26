use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

use crate::{
    color::Color,
    shaders::{checkerboard::CheckerboardShader, copy_image::CopyImageShader},
    vector::Vector2, object::Object, matrix::Matrix,
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

    pub fn checkerboard(&self, obj: &mut Object, proj: Matrix, cell_size: f32, color_a: Color, color_b: Color) {
        self.checkerboard.draw(
            &self.context,
            obj,
            proj,
            Vector2::new(cell_size, cell_size),
            color_a,
            color_b,
        );
    }
}
