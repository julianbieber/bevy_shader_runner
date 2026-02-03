#version 450
layout(location = 0) in vec2 uv;

layout(location = 0) out vec4 fragColor;

layout(set = 2, binding = 0) uniform float iTime;
layout(set = 2, binding = 1) uniform vec2 iResolution;
layout(set = 2, binding = 2) uniform vec4 slider1;
layout(set = 2, binding = 3) uniform vec4 slider2;
layout(set = 2, binding = 4) uniform vec4 slider3;
layout(set = 2, binding = 5) uniform vec4 slider4;
layout(set = 2, binding = 6) uniform vec4 slider5;

vec3 cos_s(vec3 x) {
    return (cos(x)*0.5) + 0.5;
}

vec3 color(vec3 base, vec3 scale, vec3 freq, vec3 freq_o, float x) {
    return base + scale * cos_s(freq * x + freq_o);
}

mat3x3 rot(vec3 r) {
    return mat3x3(1.0, 0.0, 0.0, 0.0, cos(r.x), -sin(r.x), 0.0, sin(r.x), cos(r.x)) *
    mat3x3(cos(r.y), 0.0, -sin(r.y), 0.0, 1.0, 0.0, sin(r.y), 0.0, cos(r.y)) *
    mat3x3(cos(r.z), -sin(r.z), 0.0, sin(r.z), cos(r.z), 0.0, 0.0,0.0,1.0);
}

float dotnois(vec3 x) {
    float a = 0.0;
    for (int i = 0; i < 3; ++i) {
        x *= rot(x);
        a += dot(cos(x), cos(x.yzx));
        x *= x;
    }
    return a;
}

float mapFire(vec3 p) {
    return length(p*p.y) - 6.0;
}

vec3 flame(vec3 ro, vec3 rd) {
    float t = 0.0;
    vec3 p = vec3(0.0);

    for (int i = 0; i < 100; ++i) {
        p = ro + rd * t;

        float d = mapFire(p);

        t += d;
        if (d < 0.001) {
            return color(slider1.xyz, vec3(0.01, 0.02, 0.1), vec3(0.01, 0.01, 0.2), vec3(iTime), dotnois(p+iTime));
        }
    }

    return color(slider1.xyz, vec3(0.01, 0.02, 0.1), vec3(0.01, 0.01, 0.2), vec3(iTime), dotnois(p+iTime));
}

float map(vec3 p) {
    return length(p) - 6.0;
}

vec3 march(vec3 ro, vec3 rd) {
    float t = 0.0;
    vec3 p = vec3(0.0);
    float min_d = 10000.0;

    for (int i = 0; i < 100; ++i) {
        p = ro + rd * t;

        float d = map(p);

        t += d;
        if (d < 0.001) {
            return flame(p, rd);
        }
        min_d = min(min_d, d);
    }

    return vec3(0.0);
}

void main() {
    vec3 rd = vec3(uv, -1.0);
    rd -= 0.5;
    rd *= 2.0;
    rd.y *= -1.0;
    rd.x *= iResolution.x / iResolution.y;
    rd.z = 1.0;
    rd = normalize(rd);

    vec3 ro = vec3(0.0, 0.0, -10.0);
    fragColor = vec4(march(ro, rd), 1.0);
}
