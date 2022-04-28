#version 300 es
precision mediump float;

in vec2 textCoord;
in vec4 fragInLight;
in vec4 fragNormal;
in vec4 fragPosition;

out vec4 color;

uniform sampler2D image;
uniform sampler2D lightmap;

uniform int isDepth;

uniform vec3 lightLocation;
uniform vec3 lightDirection;

float ambient = 0.1;
float diffuseCoeff = 1.6;
float specularCoeff = 0.3;

void main() {
    vec3 normal = normalize(fragNormal.xyz);

    if (isDepth != 0) {
        float r = texture(image, textCoord).r;
        color = vec4(r, r, r, 1.0);
        return;
    }

    vec3 frag = vec3(0.5, 0.5, 0.5) + (fragInLight.xyz / fragInLight.w) * 0.5;

    bool isInLight = true
        && frag.x > 0.0 
        && frag.x < 1.0
        && frag.y > 0.0 
        && frag.y < 1.0
        && frag.z > 0.0 
        && frag.z < 1.0;

    float brightness = ambient;
    if (isInLight && frag.z - texture(lightmap, frag.xy).r < 0.001) {
        vec3 lightLoc = vec3(lightLocation.x, lightLocation.y, -lightLocation.z);
        vec3 lightDir = vec3(lightDirection.x, lightDirection.y, -lightDirection.z);
        vec3 fromLightVec = normalize(lightLoc - fragPosition.xyz);

        float diff = max(dot(normal, fromLightVec), 0.0);
        float newDiffuse = diffuseCoeff * diff;
        
        vec3 halfwayDir = fromLightVec; //?
        float spec = pow(max(dot(halfwayDir, normal), 0.0), 32.0);
        float newSpecular = specularCoeff * spec;
        
        float theta = dot(-fromLightVec, normalize(lightDir));
        float epsilon = cos(0.3) - cos(3.1415 / 3.0);
        float intensity = clamp((theta - cos(3.1415 / 3.0)) / epsilon, 0.0, 1.0);
        
        brightness += intensity * 1.0 + 0.0001 * (newDiffuse + newSpecular);//
    }
    
    color = vec4(texture(image, textCoord).rgb * brightness, 1.0);
}