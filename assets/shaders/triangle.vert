precision mediump float;

attribute vec2 position;
attribute vec3 color;
uniform float u_angle;

varying vec3 v_color;

void main() {
    v_color = color;
    gl_Position = vec4(position, 0.0, 1.0);
    gl_Position.x *= cos(u_angle);
}