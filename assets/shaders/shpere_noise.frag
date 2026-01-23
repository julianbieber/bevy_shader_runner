#version 450
layout(location = 0) in vec2 uv;

layout(location = 0) out vec4 fragColor;

layout(set = 2, binding = 0) uniform float iTime;
layout(set = 2, binding = 1) uniform vec2 iResolution;


mat3 rot(float x, float y, float z) {
    return mat3(1.0, 0.0, 0.0, 0.0, cos(x), -sin(x), 0.0, sin(x), cos(x)) *
           mat3(cos(y), 0.0, -sin(y), 0.0, 1.0, 0.0, sin(x), 0.0, cos(x)) *
           mat3(cos(z),-sin(z),0.0,sin(z), cos(z),0.0,0.0,0.0, 1.0);
}


float dotnoise(vec3 x) {
    float acc = 0.0;
    for (int i = 0; i < 3; ++i) {
        x = rot(0.1,0.2,0.3) * x;
        acc += dot(cos(x), cos(x.yzx));
    }
    return acc /= 3.0;
}

float simpleMap(vec3 p) {
    return length(p) - dotnoise(sin(p+iTime)) - 1.0;
}
vec3 simpleNormal(vec3 p) {
    float ep = 0.001;
    vec2 h = vec2(ep, 0.0);
    return normalize(vec3(
            simpleMap(p + h.xyy) - simpleMap(p - h.xyy),
            simpleMap(p + h.yxy) - simpleMap(p - h.yxy),
            simpleMap(p + h.yyx) - simpleMap(p - h.yyx)
        ));
}

float map(vec3 p) {
    for (int i = 1; i < 4; ++i) {
        p -= (simpleNormal(p*i) * (dotnoise(vec3((p*i))))* 1.0/ i);
    }
    return simpleMap(p);
}


vec3 normal(vec3 p) {
    float ep = 0.001;
    vec2 h = vec2(ep, 0.0);
    return normalize(vec3(
            map(p + h.xyy) - map(p - h.xyy),
            map(p + h.yxy) - map(p - h.yxy),
            map(p + h.yyx) - map(p - h.yyx)
        ));
}

vec3 march(vec3 ro, vec3 rd) {
    float t = 0.0;
    float acc = 0.0;
    for (int i = 0; i < 100; ++i) {
        vec3 p = ro + rd * t;
        float d = map(p);
        if (d < 0.001) {
            vec3 n = normal(p);
            float l = dot(n, normalize(vec3(1.0, sin(iTime), -0.4)));
            return vec3(1.0) * max(l, 0.2);
        }
        t += abs(d);
    }
    return vec3(0.0);
}

void main() {
    vec3 ro = vec3(0.0, 0.0, -10.0);
    vec3 rd = vec3(uv.x, uv.y, 0.0);
    rd -= 0.5;
    rd *= 2.0;
    rd.y *= -1.0;
    rd.x *= iResolution.x / iResolution.y;
    rd.z = 1.0;
    rd = normalize(rd);
    fragColor = vec4(march(ro, rd), 1.0);
}
