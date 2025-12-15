#version 450
layout(location = 0) in vec2 v_Uv;

layout(location = 0) out vec4 o_Target;

layout(set = 2, binding = 0) uniform float time;


void main() {
    o_Target = vec4(v_Uv, abs(sin(time)), 1.0);
}
