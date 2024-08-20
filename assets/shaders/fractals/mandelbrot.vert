#version 410 core

layout (location = 0) in vec3 vs_position;

uniform float u_window_width;
uniform float u_window_height;
uniform float u_iterations;

out vec4 fs_position;
out float fs_window_width;
out float fs_window_height;
out float fs_iterations;

void main() {
    gl_Position = vec4(vs_position.x, vs_position.y, vs_position.z, 1.0);
    fs_position = gl_Position;
    fs_window_width = u_window_width;
    fs_window_height = u_window_height;
    fs_iterations = u_iterations;
}