#version 410 core

in vec4 fs_position;
uniform float u_window_width;
uniform float u_window_height;
uniform int u_iterations;

uniform bool u_black_and_white;

out vec4 FragColor;

vec4 mandelbrot_color(vec4 v_position);
vec3 hsv2rgb(vec3 c);

void main() {
    vec2 c = fs_position.xy / 1.5 * 2.0 - 0.6;
    vec2 z = c;
    float i;
    for (i = 0; i < u_iterations; i++) {
        z = vec2(pow(z.x, 2) - pow(z.y, 2), 2 * z.x * z.y) + c;
        if (length(z) > 2.0) {
            break;
        }
    }

    if (i == u_iterations) {
        FragColor = vec4(0.0, 0.0, 0.0, 1.0);
    }
    else {
        float val = i / float(u_iterations);
        if (u_black_and_white) {
            FragColor = vec4(hsv2rgb(vec3(val, 1.0, 1.0)), 1.0);
        }
        else {
            FragColor = vec4(val, val, val, 1.0);
        }
    }
}

vec3 hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}