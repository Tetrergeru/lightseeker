#version 300 es

#define MAX_BONES 32

in vec2 vertexTexture;
in vec3 vertexPosition;
in vec3 vertexNormal;
in vec4 vertexBones;
in vec4 vertexWeights;

out vec2 textCoord;
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
        pos = bones[int(vertexBones.x)] * vec4(vertexPosition, 1.0) * vertexWeights.x
            + bones[int(vertexBones.y)] * vec4(vertexPosition, 1.0) * vertexWeights.y
            + bones[int(vertexBones.z)] * vec4(vertexPosition, 1.0) * vertexWeights.z
            + bones[int(vertexBones.w)] * vec4(vertexPosition, 1.0) * vertexWeights.w;
        pos /= pos.w;
    }
    gl_Position = camera * position * pos;

    fragNormal = normalMat * vec4(vertexNormal, 1.0);
    textCoord = vertexTexture;
    fragPosition = position * pos;
}
