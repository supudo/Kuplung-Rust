#version 410 core

in vec4 fs_position;
in float fs_window_width;
in float fs_window_height;
flat in int fs_iterations;

out vec4 FragColor;

vec4 mandelbrot_color(vec4 v_position);
vec3 hsv2rgb(vec3 c);

void main() {
    //FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    //FragColor = vec4(1.0, (mod(gl_FragCoord.y, 256) / 256), 1.0, 1.0);
    FragColor = mandelbrot_color(fs_position);
}

vec4 mandelbrot_color(vec4 vpos) {
    vec4 resultColor = vec4(0.0);

    vec2 c = vpos.xy / 1 * 4.0 - 2.0;
    vec2 z = c;
    float i;
    for (i = 0; i < fs_iterations; i++) {
        z = vec2(pow(z.x, 2) - pow(z.y, 2), 2 * z.x * z.y) + c;
        if (length(z) > 2.0) {
            break;
        }
    }

    if (i == fs_iterations) {
        resultColor = vec4(0.0, 0.0, 0.0, 1.0);
    }
    else {
        float val = i / float(fs_iterations);
        //resultColor = vec4(val, val, val, 1.0);
        resultColor = vec4(hsv2rgb(vec3(val, 1.0, 1.0)), 1.0);
    }
    return resultColor;
}

vec3 hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}