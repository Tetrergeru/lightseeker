#version 300 es
precision mediump float;

in vec2 textCoord;

out vec4 color;

uniform sampler2D image;

uniform float near;
uniform float far;

void main() {
    color = texture(image, textCoord);
}