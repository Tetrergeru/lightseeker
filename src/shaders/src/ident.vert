#version 300 es

in vec4 vertexPosition;

out vec4 fragCoord;

uniform mat4 projection;

void main() {
    gl_Position = projection * vertexPosition;
    fragCoord = vertexPosition;
}
