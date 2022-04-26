#version 300 es
precision mediump float;

in vec4 fragCoord;
in vec2 textCoord;

out vec4 color;

uniform sampler2D image;

void main() {
    color = texture(image, textCoord);
}