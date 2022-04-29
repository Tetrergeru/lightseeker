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

in vec2 textCoord;
in vec4 fragNormal;
in vec4 fragPosition;

out vec4 color;

uniform sampler2D image;

uniform int isDepth;


uniform vec3 cameraPosition;

float ambient = 0.2;

float calculatePhong(Light light, vec3 toLightVec, vec3 normal) {
    float diff = clamp(dot(normal, -toLightVec), 0.0, 1.0);
    float newDiffuse = light.diffuse * diff;

    vec3 reflection = dot(toLightVec, normal) * normal * 2.0 - toLightVec;
    vec3 toCameraVec = normalize(cameraPosition - fragPosition.xyz);
    float spec = pow(max(dot(reflection, toCameraVec), 0.0), 32.0);
    float newSpecular = light.specular * spec;

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

    return intensity * calculatePhong(light, toLightVec, normal);
}

void main() {
    if (isDepth != 0) {
        color = vec4(texture(image, textCoord).rrr, 1.0);
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
    
    color = vec4(texture(image, textCoord).rgb * brightness, 1.0);
}
