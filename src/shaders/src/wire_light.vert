#version 300 es

in vec3 vertexPosition;

out vec4 fragCoord;

uniform mat4 projection;
uniform mat4 light;

void main() {
    gl_Position = projection * inverse(light) * vec4(vertexPosition, 1.0);
}
