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

layout(set = 0, binding = 0) uniform WorldData {
    mat4 view_matrix;
    mat4 projection_matrix;
    vec3 light_position;
    vec3 view_position;
} world_data;

layout(set = 1, binding = 0) uniform ModelData {
    mat4 model_matrix;
    mat4 normal_matrix;
} model_data;

void main() {
    mat4 modelview = world_data.view_matrix * model_data.model_matrix;
    vec4 world_position = world_data.projection_matrix * modelview * vec4(position, 1.0);
    vec3 worldspace_normal = mat3(model_data.normal_matrix) * normal;
    frag_ambient = ambient;
    frag_diffuse = diffuse;
    frag_position = (model_data.model_matrix * vec4(position, 1.0)).xyz;
    frag_normal = worldspace_normal;
    frag_specular = specular_exponent;

    gl_Position = world_position;
}