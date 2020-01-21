#version 450

layout(location = 0) in vec3 frag_ambient;
layout(location = 1) in vec3 frag_diffuse;
layout(location = 2) in vec3 frag_position;
layout(location = 3) in vec3 frag_normal;
layout(location = 4) in float frag_specular;

layout(set = 0, binding = 0) uniform Data {
    mat4 model;
    mat4 normal;
    mat4 view;
    mat4 proj;
    vec3 light_position;
    vec3 view_position;
} uniforms;

layout(location = 0) out vec4 f_color;

void main() {
    vec3 normal = normalize(frag_normal);
    vec3 light_direction = normalize(uniforms.light_position - frag_position);
    vec3 view_direction = normalize(uniforms.view_position - frag_position);

    vec3 reflect_direction = reflect(-light_direction, normal);
    float diffuse_dot = dot(normal, light_direction);
    float max_diffuse = max(diffuse_dot, 0.0);
    vec3 diffuse = max(diffuse_dot, 0.0) * frag_diffuse;

    vec3 specular = pow(max(dot(view_direction, reflect_direction), 0.0), 256) * vec3(1.0);

    f_color = vec4(frag_ambient + diffuse + specular, 1.0);
}