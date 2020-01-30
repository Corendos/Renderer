#version 450

layout(location = 0) in vec3 frag_ambient;
layout(location = 1) in vec3 frag_diffuse;
layout(location = 2) in vec3 frag_position;
layout(location = 3) in vec3 frag_normal;
layout(location = 4) in float frag_specular;

layout(set = 0, binding = 0) uniform WorldData {
    mat4 view_matrix;
    mat4 projection_matrix;
    vec3 light_position;
    vec3 view_position;
} world_data;

layout(location = 0) out vec4 f_color;

void main() {
    vec3 normal = normalize(frag_normal);
    vec3 light_direction = normalize(world_data.light_position - frag_position);
    vec3 view_direction = normalize(world_data.view_position - frag_position);

    vec3 reflect_direction = reflect(-light_direction, normal);
    float diffuse_dot = dot(normal, light_direction);
    float max_diffuse = max(diffuse_dot, 0.0);
    vec3 diffuse = max(diffuse_dot, 0.0) * frag_diffuse;

    vec3 specular = 0.5 * pow(max(dot(view_direction, reflect_direction), 0.0), frag_specular) * vec3(1.0);

    f_color = vec4(frag_ambient + diffuse + specular, 1.0);
}