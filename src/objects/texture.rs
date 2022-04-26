use web_sys::{HtmlImageElement, WebGl2RenderingContext, WebGlTexture};

pub struct Texture {
    _image: HtmlImageElement,
    texture: WebGlTexture,
}

impl Texture {
    pub fn new(image: HtmlImageElement, gl: &WebGl2RenderingContext) -> Self {
        let texture = gl.create_texture().unwrap();
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));
        gl.tex_image_2d_with_u32_and_u32_and_html_image_element(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            WebGl2RenderingContext::RGBA as i32,
            WebGl2RenderingContext::RGBA,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            &image,
        )
        .unwrap();
        gl.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);

        Self {
            _image: image,
            texture,
        }
    }

    pub fn location(&self) -> &WebGlTexture {
        &self.texture
    }
}
