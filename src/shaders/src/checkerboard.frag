#version 300 es
precision mediump float;

in vec2 fragCoord;

out vec4 color;

uniform vec2 cellSize;
uniform vec3 colorA;
uniform vec3 colorB;

void main() {
    int x = int((fragCoord.x + 1.0) / (2.0 * cellSize.x));
    int y = int((fragCoord.y + 1.0) / (2.0 * cellSize.y));
    if ((x + y) % 2 == 1)
        color = vec4(colorA, 1.0);
    else
        color = vec4(colorB, 1.0);
}