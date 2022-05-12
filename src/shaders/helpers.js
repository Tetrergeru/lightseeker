export function tex_image_2d_with_f32_array(gl, w, h, array) {
    gl.texImage2D(
        gl.TEXTURE_2D,
        0,
        gl.R32F,
        w,
        h,
        0,
        gl.RED,
        gl.FLOAT,
        array
    );
}