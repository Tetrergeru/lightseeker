#version 300 es

#define MAX_PARTICLES 200
#define STRIDE (3 + 3 + 2 + 4 + 4)

in vec3 vertexPosition;

out vec2 textCoord;

uniform mat4 projection;

uniform mat4[MAX_PARTICLES] transforms;

uniform int vertsInParticle;

uniform sampler2D verts;
uniform int vertsStride;

ivec2 vertexIdxToTexel(int idx) {
    return ivec2((idx % vertsStride) * STRIDE, idx / vertsStride);
}

void main() {
    int particleIdx = gl_VertexID / vertsInParticle;
    int vertexIdx = gl_VertexID % vertsInParticle;
    ivec2 vertexTexel = vertexIdxToTexel(vertexIdx);

    gl_Position = projection * transforms[particleIdx] * vec4(
        texelFetch(verts, vertexTexel + ivec2(0, 0), 0).r,
        texelFetch(verts, vertexTexel + ivec2(1, 0), 0).r,
        texelFetch(verts, vertexTexel + ivec2(2, 0), 0).r,
        1.0
    );

    textCoord = vec2(
        texelFetch(verts, vertexTexel + ivec2(6, 0), 0).r,
        texelFetch(verts, vertexTexel + ivec2(7, 0), 0).r
    );
}
