#version 410 core

in vec3 fs_color;
out vec4 FragColor;

void main() {
    FragColor = vec4(fs_color, 1.0f);
}