#version 410 core

layout (location = 0) in vec3 vs_position;
uniform float vs_angle;

void main() {
    gl_Position = vec4(vs_position.x, vs_position.y, vs_position.z, 1.0);
    gl_Position.x *= cos(vs_angle);
}