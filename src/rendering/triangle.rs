#[rustfmt::skip]
pub static VERTEX_DATA: [f32; 15] = [
  -0.5, -0.5,  1.0,  0.0,  0.0,
  0.0,  0.5,  0.0,  1.0,  0.0,
  0.5, -0.5,  0.0,  0.0,  1.0,
];

pub const VERTEX_SHADER_SOURCE: &[u8] = b"
#version 410 core
precision mediump float;

attribute vec2 position;
attribute vec3 color;

varying vec3 v_color;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    v_color = color;
}
\0";

pub const FRAGMENT_SHADER_SOURCE: &[u8] = b"
#version 410 core
precision mediump float;

varying vec3 v_color;

void main() {
    gl_FragColor = vec4(v_color, 1.0);
}
\0";