#version 300 es

in vec3 vertexPosition;
in vec3 vertexNormal;
in vec2 vertexTexture;

out vec2 textCoord;

uniform mat4 projection;

void main() {
    gl_Position = projection * vec4(vertexPosition + vertexNormal * 0.0001, 1.0);
    textCoord = vertexTexture;
}
