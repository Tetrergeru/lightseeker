#version 300 es

in vec3 vertexPosition;
in vec3 vertexNormal;
in vec2 vertexTexture;

out vec2 textCoord;
out vec4 fragInLight;

uniform mat4 position;
uniform mat4 camera;
uniform mat4 light;

void main() {
    fragInLight = light * position * vec4(vertexPosition, 1.0);
    gl_Position = camera * position * vec4(vertexPosition, 1.0);

    textCoord = vertexTexture;
}
