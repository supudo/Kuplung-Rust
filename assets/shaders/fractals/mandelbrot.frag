#version 410 core

in vec4 fs_position;
in float fs_window_width;
in float fs_window_height;

out vec4 FragColor;

vec4 mandelbrot_color(vec4 v_position);

void main() {
    //FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    //FragColor = vec4(1.0, (mod(gl_FragCoord.y, 256) / 256), 1.0, 1.0);
    FragColor = mandelbrot_color(fs_position);
}

vec4 mandelbrot_color(vec4 vpos) {
    vec2 c = vpos.xy / 1 * 4.0 - 2.0;

    vec2 z = c;
    float i;
    for (i = 0; i < 9; i++) {
        z = vec2(pow(z.x, 2) - pow(z.y, 2), 2 * z.x * z.y) + c;
    }

    vec4 resultColor = vec4(0.0);
    if (length(z) <= 1.0) {
        resultColor = vec4(0.0, 0.0, 0.0, 1.0);
    }
    else {
        resultColor = vec4(1.0, 1.0, 1.0, 1.0);
    }
    return resultColor;
}