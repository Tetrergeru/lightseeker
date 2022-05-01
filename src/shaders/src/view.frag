#version 300 es
precision mediump float;

#define PI 3.14159265

struct Light {
    float diffuse;
    float specular;

    mat4 projection;
    vec3 position;
    vec3 direction;
    float fov;
    sampler2D map;
};

#define MAX_LIGHTS 16

uniform Light[MAX_LIGHTS] lights;

uniform vec3 pointLightPosition;
uniform sampler2D pointLightmap;

in vec2 textCoord;
in vec4 fragNormal;
in vec4 fragPosition;

out vec4 color;

uniform sampler2D image;

uniform int isDepth;

uniform vec3 cameraPosition;

float ambient = 0.2;

float near = sqrt(6.0) / 12.0;
float far = 20.0;

float calculatePhong(float lightDiffuse, float lightSpecular, vec3 toLightVec, vec3 normal) {
    float diff = clamp(dot(normal, -toLightVec), 0.0, 1.0);
    float newDiffuse = lightDiffuse * diff;

    vec3 reflection = dot(toLightVec, normal) * normal * 2.0 - toLightVec;
    vec3 toCameraVec = normalize(cameraPosition - fragPosition.xyz);
    float spec = pow(max(dot(reflection, toCameraVec), 0.0), 32.0);
    float newSpecular = lightSpecular * spec;

    return newDiffuse + newSpecular;
}

bool in_range(float a) {
    return 0.0 < a && a < 1.0;
}

float calculateProjectorLight(Light light, vec3 normal) {
    vec4 fragInLight = light.projection * fragPosition;
    vec3 frag = vec3(0.5, 0.5, 0.5) + (fragInLight.xyz / fragInLight.w) * 0.5;
    bool isInLight = in_range(frag.x) && in_range(frag.y) && in_range(frag.z);

    if (!isInLight || frag.z - texture(light.map, frag.xy).r >= 0.01) {
        return 0.0;
    }

    vec3 fromLightVec = normalize(fragPosition.xyz - light.position);
    vec3 toLightVec = -fromLightVec;

    float theta = dot(fromLightVec, light.direction);
    float B = light.fov / 2.0;
    float A = B / 4.0;
    float epsilon = cos(A) - cos(B);
    float intensity = clamp((theta - cos(B)) / epsilon, 0.0, 1.0);

    return intensity * calculatePhong(light.diffuse, light.specular, toLightVec, normal);
}

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

vec2 textureByDirection(int direction, vec2 texture) {
    return direction == 0 ? texture * 0.5
        : direction == 1 ? vec2(0.5, 0.0) + texture * 0.5
        : direction == 2 ? vec2(0.0, 0.5) + texture * 0.5
        : vec2(0.5, 0.5) + texture * 0.5;
}

float calculatePointLight(vec3 normal) {
    vec3 fragInLight = fragPosition.xyz - pointLightPosition;

    for (int i = 0; i < 4; i++) {
        float hAngle = i == 0 ? 0.0
            : i == 1 ? PI / 3.0
            : i == 2 ? PI
            : -PI / 3.0;
        float vAngle = i == 0 ? vAngle0() : vAngle123();

        vec4 frag = perspective() * rot_v(vAngle) * rot_h(hAngle) * vec4(fragInLight, 1.0);

        frag /= frag.w;
        frag = vec4(0.5, 0.5, 0.5, 0.5) + frag * 0.5;

        bool isInLight = in_range(frag.x) && in_range(frag.y) && in_range(frag.z);
        if (isInLight) {
            if (frag.z - texture(pointLightmap, textureByDirection(i, frag.xy)).r < 0.01) {
                return calculatePhong(1.0, 1.0, -normalize(fragInLight), normal);
            }
            break;
        }
    }

    return 0.0;
}

void main() {
    if (isDepth != 0) {
        color = vec4(texture(image, textCoord).rgb, 1.0);
        return;
    }

    vec3 normal = normalize(fragNormal.xyz);

    float brightness = ambient;
    for (int i = 0; i < MAX_LIGHTS; i++) {
        if (lights[i].specular < 0.0) {
            break;
        }
        brightness += calculateProjectorLight(lights[i], normal);
    }
    brightness += calculatePointLight(normal);

    color = vec4(
        texture(image, textCoord).rgb * brightness,
        1.0
    );
}

