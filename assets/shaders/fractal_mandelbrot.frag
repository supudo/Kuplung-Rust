#version 410 core
precision mediump float;

in vec3 fs_vertexPosition;

out vec4 fragColor;

void main() {
    vec2 c = fs_vertexPosition.xy / 767.0 * 4.0 - 2.0;

    vec2 z = c;
    float i;
    for (i = 0; i < 9; i++) {
        z = vec2(pow(z.x, 2) - pow(z.y, 2), 2 * z.x * z.y) + c;
    }

    if (length(z) <= 2.0) {
        fragColor = vec4(0.0, 0.0, 0.0, 1.0);
    }
    else {
        fragColor = vec4(1.0, 1.0, 1.0, 1.0);
    }
}