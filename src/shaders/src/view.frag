#version 300 es
precision mediump float;

in vec2 textCoord;
in vec4 fragInLight;

out vec4 color;

uniform sampler2D image;
uniform sampler2D lightmap;

uniform int isDepth;

void main() {
    vec3 frag = vec3(0.5, 0.5, 0.5) + (fragInLight.xyz / fragInLight.w) * 0.5;

    bool isInLight = true
        && frag.x > 0.0 
        && frag.x < 1.0
        && frag.y > 0.0 
        && frag.y < 1.0
        && frag.z > 0.0 
        && frag.z < 1.0;

    if (!isInLight || abs(frag.z - texture(lightmap, frag.xy).r) >= 0.01) {
        color = vec4(texture(image, textCoord).rgb * 0.3, 1.0);
    } else {
        color = texture(image, textCoord);
    }
    
    if (isDepth != 0) {
        float r = texture(image, textCoord).r;
        color = vec4(r, r, r, 1.0);
    }
}