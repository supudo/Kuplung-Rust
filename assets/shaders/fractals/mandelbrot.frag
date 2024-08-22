#version 410 core
precision highp float;

in vec4 fs_position;
uniform float u_window_width;
uniform float u_window_height;
uniform int u_iterations;

uniform bool u_black_and_white;
uniform int u_color_palette;
uniform vec2 u_zoomCenter;
uniform float u_zoomSize;

out vec4 FragColor;

vec4 mandelbrot_color_normal(vec4 v_position);
vec4 mandelbrot_color_grainy(vec4 v_position);
vec3 hsv2rgb(vec3 c);

// ===========================
// Normal color palette
// ===========================

vec4 mandelbrot_color_normal(vec4 v_position) {
    vec4 resultColor = vec4(1.0);
    vec2 c = v_position.xy / 1.5 * 2.0 - 0.6;
    vec2 z = c;
    float i;
    for (i = 0; i < u_iterations; i++) {
        z = vec2(pow(z.x, 2) - pow(z.y, 2), 2 * z.x * z.y) + c;
        if (length(z) > 2.0) {
            break;
        }
    }

    if (i == u_iterations) {
        resultColor = vec4(0.0, 0.0, 0.0, 1.0);
    }
    else {
        float val = i / float(u_iterations);
        if (u_black_and_white) {
            resultColor = vec4(hsv2rgb(vec3(val, 1.0, 1.0)), 1.0);
        }
        else {
            resultColor = vec4(val, val, val, 1.0);
        }
    }
    return resultColor;
}

vec3 hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

// ===========================
// Grainy color palette
// ===========================

vec2 f(vec2 x, vec2 c) {
    return mat2(x, -x.y, x.x) * x + c;
}

vec3 palette_grainy(float t, vec3 c1, vec3 c2, vec3 c3, vec3 c4) {
    return a + b * cos(6.28318 * (c * t + d));
}

vec4 mandelbrot_color_grainy(vec4 v_position) {
    vec2 uv = v_position.xy / vec2(u_window_width, u_window_height);
    vec2 c0 = u_zoomCenter + (uv * 4.0 - vec2(2.0)) * (u_zoomSize / 4.0);
    vec2 x = vec2(0.0);
    bool escaped = false;
    int iterations;
    for (int i = 0; i < 10000; i++) {
        if (i > u_iterations) break;
        iterations = i;
        x = f(x, c0);
        if (length(x) > 2.0) {
            escaped = true;
            break;
        }
    }
    float t = float(iterations) / float(u_iterations);
    vec3 a = vec3(0.0);
    vec3 b = vec3(0.59, 0.55, 0.75);
    vec3 c = vec3(0.1, 0.2, 0.3);
    vec3 d = vec3(0.75);
    vec3 pallete_color = palette_grainy(t, a, b, c, d);
    vec4 bg_color = vec4(0.85, 0.99, 1.0, 1.0);
    return escaped ? vec4(pallete_color, 1.0) : bg_color;
}

// ===========================
// ...
// ===========================

void main() {
    if (u_color_palette == 0)
        FragColor = mandelbrot_color_normal(fs_position);
    else
        FragColor = mandelbrot_color_grainy(fs_position);
}