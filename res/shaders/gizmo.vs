#version 450

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec3 ambient;
layout(location = 3) in vec3 diffuse;
layout(location = 4) in float specular_exponent;

layout(location = 0) out vec3 vertex_out_color;

layout(set = 0, binding = 0) uniform Data {
    mat4 model;
    mat4 normal;
    mat4 view;
    mat4 proj;
    vec3 light_position;
    vec3 view_position;
} uniforms;

void main() {
    vertex_out_color = diffuse;
    gl_Position = uniforms.proj * uniforms.view * uniforms.model * vec4(position, 1.0);
}