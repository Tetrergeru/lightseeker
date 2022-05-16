#version 300 es
precision mediump float;

#define PI 3.14159265
#define MAX_LIGHTS 16
#define SHADOW_BIAS 0.001

in vec2 textCoord;
in vec4 fragNormal;
in vec4 fragPosition;

out vec4 color;

const int DirectionalLightType = 0;
const int PointLightType = 1;
const int LightSequenceEnd = -1;

struct Light {
    int type;

    float diffuse;
    float specular;
    sampler2D map;
    vec3 color;

    // Point light will use all 4 matrices, whereas directional light will use only the first
    mat4[4] projection;
    vec3 position;
    // These parameters are specific to directional light
    vec3 direction;
    float fov;
    float innerFov;
};

uniform Light[MAX_LIGHTS] lights;
uniform int ignoreLight;

uniform sampler2D textureMap;

uniform vec3 cameraPosition;

float ambient = 0.2;

float calculatePhong(float lightDiffuse, float lightSpecular, vec3 toLightVec, vec3 normal) {
    float diff = clamp(dot(normal, toLightVec), 0.0, 1.0);
    float newDiffuse = lightDiffuse * diff;

    vec3 reflection = dot(toLightVec, normal) * normal * 2.0 - toLightVec;
    vec3 toCameraVec = -normalize(cameraPosition - fragPosition.xyz);
    float spec = pow(max(dot(reflection, toCameraVec), 0.0), 32.0);
    float newSpecular = lightSpecular * spec;

    return newDiffuse + newSpecular;
}

bool in_range(float a) {
    return 0.0 < a && a < 1.0;
}

float calculateProjectorLight(Light light, vec3 normal) {
    vec4 fragInLight = light.projection[0] * fragPosition;
    vec3 frag = vec3(0.5, 0.5, 0.5) + (fragInLight.xyz / fragInLight.w) * 0.5;
    bool isInLight = in_range(frag.x) && in_range(frag.y) && in_range(frag.z);

    if (!isInLight || frag.z - texture(light.map, frag.xy).r >= SHADOW_BIAS) {
        return 0.0;
    }

    vec3 toLightVec = normalize(light.position - fragPosition.xyz);
    vec3 fromLightVec = -toLightVec;

    float theta = dot(fromLightVec, light.direction);
    float B = light.fov / 2.0;
    float A = B / 4.0;
    float epsilon = cos(A) - cos(B);
    float intensity = clamp((theta - cos(B)) / epsilon, 0.0, 1.0);

    return intensity * light.diffuse + calculatePhong(0.0, light.specular, toLightVec, normal);
}

vec2 textureByDirection(int direction, vec2 texture) {
    return direction == 0 ? texture * 0.5
        : direction == 1 ? vec2(0.5, 0.0) + texture * 0.5
        : direction == 2 ? vec2(0.0, 0.5) + texture * 0.5
        : vec2(0.5, 0.5) + texture * 0.5;
}

float calculatePointLight(Light light, vec3 normal) {
    vec3 toLightVec = normalize(light.position - fragPosition.xyz);
    for (int i = 0; i < 4; i++) {
        vec4 frag = light.projection[i] * fragPosition;

        frag /= frag.w;
        frag = vec4(0.5, 0.5, 0.5, 0.5) + frag * 0.5;

        bool isInLight = in_range(frag.x) && in_range(frag.y) && in_range(frag.z);
        if (isInLight) {
            if (frag.z - texture(light.map, textureByDirection(i, frag.xy)).r < SHADOW_BIAS) {
                return calculatePhong(light.diffuse, light.specular, toLightVec, normal);
            }
            break;
        }
    }

    return 0.0;
}

void main() {
    vec3 normal = normalize(fragNormal.xyz);

    vec3 brightness;
    if (ignoreLight == 0) {
        brightness = vec3(ambient, ambient, ambient);
        for (int i = 0; i < MAX_LIGHTS; i++) {
            if (lights[i].type == LightSequenceEnd) {
                break;
            } else if (lights[i].type == DirectionalLightType) {
                brightness += lights[i].color * calculateProjectorLight(lights[i], normal);
            } else {
                brightness += lights[i].color * calculatePointLight(lights[i], normal);
            }
        }
    } else {
        brightness = vec3(1.0, 1.0, 1.0);
    }

    float dist = clamp((distance(fragPosition.xyz, -cameraPosition) - 5.0) / 15.0, 0.0, 1.0);
    color = mix(
        vec4(
            texture(textureMap, textCoord).rgb * brightness,
            1.0
        ),
        vec4(1.0),
        dist
    );
}

