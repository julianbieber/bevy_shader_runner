#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Normal;
layout(location = 2) in vec2 Vertex_Uv;

layout(location = 0) out vec2 v_Uv;
layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 clip_from_world;
} camera_view;

void main() {
    v_Uv = Vertex_Uv;
    gl_Position = camera_view.clip_from_world * vec4(Vertex_Position, 1.0);

}
