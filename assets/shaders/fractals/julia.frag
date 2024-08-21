#version 410 core

in vec4 fs_position;
in float fs_window_width;
in float fs_window_height;
flat in int fs_iterations;
in vec2 fs_color;

out vec4 FragColor;

vec4 julia_color(vec4 v_position);
vec3 hsv2rgb(vec3 c);

void main() {
    vec2 z = fs_color;
    int i;
    for (i = fs_iterations; i != 0; i--) {
        float real = z.x * z.x - z.y * z.y - 0.74543;
        float imag = 2.0 * z.x * z.y + 0.11301;
        // Sequences with abs(z) > 2 will always diverge
        if (real * real + imag * imag > 4.0)
        break;
        z.x = real;
        z.y = imag;
    }

    float conv = float(i) / float(fs_iterations);
    float red = i == 0 ? 0.0 : 1.0 - conv;
    float green = 0.0;
    float blue = i == 0 ? 0.0 : conv;

    FragColor = vec4(red, green, blue, 1.0);
}

vec3 hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}