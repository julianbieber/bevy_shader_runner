#version 450
layout(location = 0) in vec2 uv;

layout(location = 0) out vec4 fragColor;

layout(set = 2, binding = 0) uniform float iTime;
layout(set = 2, binding = 1) uniform vec2 iResolution;

vec3 rgb(int r, int g, int b) {
    return vec3(float(r) / 255.0, float(g) / 255.0, float(b) / 255.0);
}

mat3x3 rot(float x, float y, float z) {
    return mat3x3(1.0, 0.0, 0.0, 0.0, cos(x), -sin(x), 0.0, sin(x), cos(x)) *
        mat3x3(cos(y), 0.0, -sin(y), 0.0, 1.0, 0.0, -sin(y), 0.0, cos(y)) *
        mat3x3(cos(z), -sin(z), 0.0, sin(z), cos(z), 0.0, 0.0, 0.0, 1.0);
}

float dotnoise(vec3 x) {
    float a = 0.0;
    vec3 o = x;

    for (int i = 0; i < 5; ++i) {
        x = rot(i, i, i) * x;
        a += dot(cos(x), cos(x.yzx));
    }

    return a * (1.0 / 3.0);
}

float fbm_dot(vec3 x) {
    float acc = 0.0;
    float freq_m = 2.0;
    float freq = 1.;
    float amp_m = 0.5;
    float amp = 10.0;

    for (int i = 0; i < 10; ++i) {
        acc += abs(dotnoise(x * freq)) * amp;
        freq *= freq_m;
        amp *= amp_m;
    }
    return acc;
}

float map(vec3 p) {
    return p.y - fbm_dot(vec3(p.x, 0.0, p.z)) * .1;
}

vec3 normal_map(vec3 p) {
    float e = 0.0001;
    vec2 h = vec2(e, 0.0);

    return normalize(vec3(
            map(p + h.xyy) - map(p - h.xyy),
            map(p + h.yxy) - map(p - h.yxy),
            map(p + h.yyx) - map(p - h.yyx)
        ));
}

float fog(vec3 p) {
    return dotnoise(p.zyx);
}
vec3 cos_scaled(vec3 p) {
    return 2.0 * (cos(p) + 1.0);
}

vec3 color_scale(vec3 base, vec3 range, vec3 freq, vec3 offset, float x) {
    return base + range * cos(freq * x + offset);
}

vec3 ground_color(vec3 p) {
    return color_scale(vec3(0.0, 0.5, 0.2), vec3(0.2, 0.2, 0.4), vec3(0.2), vec3(iTime, iTime*0.2, iTime*0.5), p.y);
}

vec3 fog_c(float i) {
    return color_scale(vec3(0.2, 0.2, 0.2), vec3(0.5, 0.2, 0.4), vec3(0.2), vec3(iTime, iTime*0.2, iTime*0.5), i);
}

vec3 march(vec3 ro, vec3 rd) {
    float t = 0.0;
    vec3 n = vec3(0.0);
    vec3 l = normalize(vec3(100.0, 1.0, 0.0));

    vec3 p = vec3(0.0);
    for (int i = 0; i < 100; ++i) {
        p = ro + rd * t;
        float d = map(p);

        if (d < 0.001) {
            n = normal_map(p);
            break;
        }
        t += d;
    }

    float split = 10.0;

    float t_dist = min(t, 1000.0) / split;

    float fog_acc = 0.0;

    for (float i = 0.0; i < split; i += 1.0) {
        float local_t = t_dist * i;
        vec3 fp = ro + (rd) * local_t;
        fog_acc += abs(fog(fp+iTime));
    }

    float fog_i = pow(2.23, -fog_acc);
    if (t > 100.0) {
        return vec3(fog_i);
    }

    return mix(ground_color(p) * dot(n, l)*ground_color(n), fog_c(fog_i), (1.0 - fog_i*(2.0+iTime*10.0))*0.5) ;
}

void main() {
    vec3 ro = vec3(0.0, -map(vec3(0.0, 0.0, -10.0+iTime))+1.6, -10.0+iTime);
    vec3 rd = vec3(uv, 1.0);
    rd.x -= 0.5;
    rd.x *= 2.0;
    rd.y -= 0.5;
    rd.y *= -2.0;

    rd.x *= iResolution.x / iResolution.y;
    rd = rot(0.0, iTime*0.1, 0.0) * normalize(rd);
    fragColor = vec4(march(ro, rd), 1.0);
    // fragColor= vec4(rd.yyy, 1.0);
}
