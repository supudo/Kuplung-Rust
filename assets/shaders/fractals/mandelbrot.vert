#version 410 core

layout (location = 0) in vec3 vs_position;

out vec4 fs_position;

void main() {
    gl_Position = vec4(vs_position.x, vs_position.y, vs_position.z, 1.0);
    fs_position = gl_Position;
}