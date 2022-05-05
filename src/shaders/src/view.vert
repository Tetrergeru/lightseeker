#version 300 es

#define MAX_BONES 32

in vec3 vertexPosition;
in vec3 vertexNormal;
in vec2 vertexTexture;
in vec4 vertexBones;
in vec4 vertexWeights;

out vec2 textCoord;
out vec4 fragInLight;
out vec4 fragNormal;
out vec4 fragPosition;

uniform mat4 position;
uniform mat4 normalMat;
uniform mat4 camera;

uniform int boneCount;
uniform mat4[MAX_BONES] bones;

void main() {
    vec4 pos;
    if (boneCount == 0) {
        pos = vec4(vertexPosition, 1.0);
    } else {
        pos = vec4(0.0);
        for (int i = 0; i < 4; i++) {
            pos += bones[int(vertexBones[i])] * vec4(vertexPosition, 1.0) * vertexWeights[i];
        }
        pos /= 4.0;
    }
    gl_Position = camera * position * pos;

    textCoord = vertexTexture;
    fragNormal = normalMat * vec4(vertexNormal, 1.0);
    fragPosition = pos;
}
