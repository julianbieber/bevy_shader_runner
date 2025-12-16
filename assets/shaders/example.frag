#version 450
layout(location = 0) in vec2 uv;

layout(location = 0) out vec4 fragColor;

layout(set = 2, binding = 0) uniform float iTime;
layout(set = 2, binding = 1) uniform vec2 iResolution;


void main() {
    fragColor = vec4(uv*4.0, abs(sin(iTime)), 1.0);
}
