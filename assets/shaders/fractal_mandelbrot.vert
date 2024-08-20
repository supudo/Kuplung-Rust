#version 410 core
precision mediump float;

in vec2 vs_position;
uniform float vs_angle;

out vec3 fs_vertexPosition;

void main() {
    gl_Position = vec4(vs_position, 0.0, 1.0);
    gl_Position.x *= cos(vs_angle);
    fs_vertexPosition = vec3(gl_Position);
}