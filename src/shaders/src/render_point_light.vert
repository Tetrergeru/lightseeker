#version 300 es
precision mediump float;

#define PI 3.141592

in vec3 vertexPosition;
in vec2 vertexTexture;

out vec2 textCoord;
out vec4 fragPosition;

uniform mat4 projection;

uniform float near;
uniform float far;

uniform float flip;

float angleFromVec(float x, float y) {
    vec2 n = normalize(vec2(x, y));
    float cosA = n.x;
    float sinA = n.y;
    if (sinA > 0.0) {
        return acos(cosA);
    } else {
        if (cosA > 0.0) {
            return -acos(cosA);
        } else {
            return 2.0 * PI - acos(cosA);
        }
    }
}

void main() {
    fragPosition = projection * vec4(vertexPosition, 1.0);

    float x = 2.0 * angleFromVec(fragPosition.x, flip * fragPosition.z) / PI - 1.0;
    float y = 2.0 * angleFromVec(fragPosition.y, flip * fragPosition.z) / PI - 1.0;
    float z = 2.0 * (length(fragPosition) - near) / (far - near) - 1.0;

    gl_Position = vec4(x, y, z, 1.0);
    textCoord = vertexTexture;
}
