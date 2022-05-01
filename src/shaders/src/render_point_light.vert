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

// 0 -- down, 1 -- front-left, 2 -- back, 3 -- front-right
uniform int direction;

float fov() {
    return 2.0 * atan(sqrt(6.0));
}

float aspect() {
    return 2.0 / sqrt(3.0);
}

float vAngle0() {
    return PI / 2.0 + fov() * 2.65 / 2.0;
}

float vAngle123() {
    return PI / 2.0 + fov() / 2.0;
}

mat4 perspective() {
    float f = tan(PI * 0.5 - 0.5 * fov());
    float range_inv = 1.0 / (near - far);
    return mat4(
        f / aspect(), 0.0, 0.0, 0.0,
        0.0, f, 0.0, 0.0,
        0.0, 0.0, (near + far) * range_inv, -1.0,
        0.0, 0.0, near * far * range_inv * 2.0, 0.0
    );
}

mat4 rot_h(float angle) {
    float Cos = cos(angle);
    float Sin = sin(angle);
    return mat4(
        Cos, 0.0, -Sin, 0.0,
        0.0, 1.0, 0.0, 0.0,
        Sin, 0.0, Cos, 0.0,
        0.0, 0.0, 0.0, 1.0
    );
}

mat4 rot_v(float angle) {
    float Cos = cos(angle);
    float Sin = sin(angle);
    return mat4(
        1.0, 0.0, 0.0, 0.0,
        0.0, Cos, -Sin, 0.0,
        0.0, Sin, Cos, 0.0,
        0.0, 0.0, 0.0, 1.0
    );
}

void main() {
    float hAngle = direction == 0 ? 0.0
        : direction == 1 ? PI / 3.0
        : direction == 2 ? PI
        : -PI / 3.0;
    float vAngle = direction == 0 ? vAngle0() : vAngle123();

    fragPosition = perspective() * rot_v(vAngle) * rot_h(hAngle) * projection * vec4(vertexPosition, 1.0);

    // float x = 2.0 * angleFromVec(fragPosition.x, flip * fragPosition.z) / PI - 1.0;
    // float y = 2.0 * angleFromVec(fragPosition.y, flip * fragPosition.z) / PI - 1.0;
    // float z = 2.0 * (length(vec3(fragPosition.xy, fragPosition.z)) - near) / (far - near) - 1.0;

    gl_Position = fragPosition;
    textCoord = vertexTexture;
}
