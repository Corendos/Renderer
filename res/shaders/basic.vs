#version 450

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec3 ambient;
layout(location = 3) in vec3 diffuse;
layout(location = 4) in float specular_exponent;

layout(location = 0) out vec3 frag_ambient;
layout(location = 1) out vec3 frag_diffuse;
layout(location = 2) out vec3 frag_position;
layout(location = 3) out vec3 frag_normal;
layout(location = 4) out float frag_specular;

layout(set = 0, binding = 0) uniform Data {
    mat4 model;
    mat4 normal;
    mat4 view;
    mat4 proj;
    vec3 light_position;
    vec3 view_position;
} uniforms;

void main() {
    mat4 modelview = uniforms.view * uniforms.model;
    vec4 world_position = uniforms.proj * modelview * vec4(position, 1.0);
    vec3 worldspace_normal = mat3(uniforms.normal) * normal;
    frag_ambient = ambient;
    frag_diffuse = diffuse;
    frag_position = (uniforms.model * vec4(position, 1.0)).xyz;
    frag_normal = worldspace_normal;
    frag_specular = specular_exponent;

    gl_Position = world_position;
}