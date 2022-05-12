#version 300 es
precision mediump float;

out vec4 color;
uniform sampler2D image;

in vec2 textCoord;

void main() {
    color = texture(image, textCoord);
}