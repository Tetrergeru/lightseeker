#version 300 es

#define MAX_PARTICLES 32
#define STRIDE (3 + 3 + 2 + 4 + 4)

in vec3 vertexPosition;

out vec2 textCoord;

uniform mat4 projection;

uniform mat4[MAX_PARTICLES] transforms;

uniform int vertsInParticle;

uniform sampler2D verts;

void main() {
    int particleIdx = gl_VertexID / vertsInParticle;
    int vertexIdx = gl_VertexID % vertsInParticle;

    gl_Position = projection * transforms[particleIdx] * vec4(
        texelFetch(verts, ivec2(0, vertexIdx), 0).r,
        texelFetch(verts, ivec2(1, vertexIdx), 0).r,
        texelFetch(verts, ivec2(2, vertexIdx), 0).r,
        1.0
    );

    textCoord = vec2(
        texelFetch(verts, ivec2(6, vertexIdx), 0).r,
        texelFetch(verts, ivec2(7, vertexIdx), 0).r
    );
}
