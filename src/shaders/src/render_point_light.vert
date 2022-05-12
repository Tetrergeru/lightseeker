#version 300 es
precision mediump float;

#define MAX_BONES 32

in vec3 vertexPosition;
in vec4 vertexBones;
in vec4 vertexWeights;

out vec2 textCoord;

uniform mat4 projection;

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

    gl_Position = projection * pos;
}
