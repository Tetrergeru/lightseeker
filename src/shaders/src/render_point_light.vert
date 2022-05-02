#version 300 es
precision mediump float;

#define PI 3.141592

in vec3 vertexPosition;
in vec2 vertexTexture;

out vec2 textCoord;

uniform mat4 projection;

void main() {
    gl_Position = projection * vec4(vertexPosition, 1.0);
    textCoord = vertexTexture;
}
