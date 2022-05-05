use web_sys::{HtmlImageElement, WebGl2RenderingContext as Gl, WebGlTexture};

#[derive(Debug)]
pub struct Texture {
    texture: WebGlTexture,
}

impl Texture {
    pub fn new(image: HtmlImageElement, gl: &Gl) -> Self {
        let texture = gl.create_texture().unwrap();
        gl.bind_texture(Gl::TEXTURE_2D, Some(&texture));
        gl.tex_image_2d_with_u32_and_u32_and_html_image_element(
            Gl::TEXTURE_2D,
            0,
            Gl::RGBA as i32,
            Gl::RGBA,
            Gl::UNSIGNED_BYTE,
            &image,
        )
        .unwrap();
        gl.generate_mipmap(Gl::TEXTURE_2D);

        Self { texture }
    }

    pub fn new_depth(gl: &Gl, w: u32, h: u32) -> Self {
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

        Self::from_texture(texture)
    }

    pub fn with_size(gl: &Gl, w: u32, h: u32) -> Self {
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

        Self::from_texture(texture)
    }

    pub fn from_texture(texture: WebGlTexture) -> Self {
        Self { texture }
    }

    pub fn location(&self) -> &WebGlTexture {
        &self.texture
    }
}
