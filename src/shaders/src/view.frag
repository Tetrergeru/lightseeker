#version 300 es
precision mediump float;

in vec2 textCoord;

out vec4 color;

uniform sampler2D image;
uniform int isDepth;

void main() {
    if (isDepth != 0) {
        float r = texture(image, textCoord).r;
        color = vec4(r, r, r, 1.0);
    } else {
        color = texture(image, textCoord);
    }
}