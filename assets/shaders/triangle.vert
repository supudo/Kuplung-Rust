#version 410 core
precision mediump float;

attribute vec2 vs_position;
attribute vec3 vs_color;
uniform float vs_angle;

varying vec3 v_color;

void main() {
    v_color = vs_color;
    gl_Position = vec4(vs_position, 0.0, 1.0);
    gl_Position.x *= cos(vs_angle);
}