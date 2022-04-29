#version 300 es

in vec3 vertexPosition;
in vec3 vertexNormal;
in vec2 vertexTexture;

out vec2 textCoord;
out vec4 fragInLight;
out vec4 fragNormal;
out vec4 fragPosition;

uniform mat4 position;
uniform mat4 normalMat;
uniform mat4 camera;

void main() {
    gl_Position = camera * position * vec4(vertexPosition, 1.0);

    textCoord = vertexTexture;
    fragNormal = normalMat * vec4(vertexNormal, 1.0);
    fragPosition = position * vec4(vertexPosition, 1.0);
}
