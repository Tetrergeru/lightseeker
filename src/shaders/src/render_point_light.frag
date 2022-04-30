#version 300 es
precision mediump float;

in vec2 textCoord;
in vec4 fragPosition;

out vec4 color;

uniform sampler2D image;

uniform float near;
uniform float far;

void main() {
    // gl_FragDepth = 2.0 * (length(fragPosition) - near) / (far - near) - 1.0;
    color = texture(image, textCoord);
}