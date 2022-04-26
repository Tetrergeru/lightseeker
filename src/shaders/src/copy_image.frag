#version 300 es
precision mediump float;

uniform sampler2D image;

in vec2 fragCoord;

out vec4 color;

void main() {
    vec2 pos = vec2(fragCoord.x, -fragCoord.y);
    color = texture(image, vec2(0.5, 0.5) + pos * 0.5);
}