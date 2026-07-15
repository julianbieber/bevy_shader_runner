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

vec3 oklab2srgb(vec3 c) {
    float l_ = c.x + 0.3963377774 * c.y + 0.2158037573 * c.z;
    float m_ = c.x - 0.1055613458 * c.y - 0.0638541728 * c.z;
    float s_ = c.x - 0.0894841775 * c.y - 1.2914855480 * c.z;

    float l = l_ * l_ * l_;
    float m = m_ * m_ * m_;
    float s = s_ * s_ * s_;

    vec3 lin = vec3(
            4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
            -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
            -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s
        );

    // linear -> sRGB gamma encode
    vec3 sign_ = sign(lin);
    vec3 abs_ = abs(lin);
    vec3 srgb = mix(
            1.055 * pow(abs_, vec3(1.0 / 2.4)) - 0.055,
            abs_ * 12.92,
            lessThan(abs_, vec3(0.0031308))
        );
    return sign_ * srgb;
}

float discretizeToValue(float value, float minVal, float maxVal, int n) {
    float t = clamp((value - minVal) / (maxVal - minVal), 0.0, 1.0);
    float step = 1.0 / float(n);
    float nearestStep = round(t / step); // round instead of floor -> nearest border
    nearestStep = clamp(nearestStep, 0.0, float(n));
    return minVal + nearestStep * step * (maxVal - minVal);
}

vec3 rayHitY(vec3 origin, vec3 dir, float targetY) {
    float t = (targetY - origin.y) / dir.y;
    return origin + dir * t;
}


mat3 rot(float a, float b, float c) {
    return mat3(1.0, 0.0, 0.0,
        0.0, cos(a), -sin(a),
        0.0, sin(a), cos(a)) *
        mat3(cos(b), -sin(b), 0.0,
            sin(b), cos(b), 0.0,
            0.0, 0.0, 1.0) *
        mat3(cos(c), 0.0, -sin(c),
            0.0, 1.0, 0.0,
            sin(c), 0.0, cos(c));
}

float hash(vec3 p) {
    p = fract(p * 0.1031);
    p += sin(dot(p, p.zyx + 31.32) * 23.32);
    return fract((p.x + p.y) * p.z);
}

float cnoise(vec3 p) {
    vec3 ip = floor(p);
    vec3 fp = fract(p);

    float a = hash(ip + vec3(0, 0, 0));
    float b = hash(ip + vec3(1, 0, 0));
    float c = hash(ip + vec3(0, 1, 0));
    float d = hash(ip + vec3(1, 1, 0));
    float e = hash(ip + vec3(0, 0, 1));
    float f = hash(ip + vec3(1, 0, 1));
    float g = hash(ip + vec3(0, 1, 1));
    float h = hash(ip + vec3(1, 1, 1));

    return mix(
        mix(mix(a, b, fp.x), mix(c, d, fp.x), fp.y),
        mix(mix(e, f, fp.x), mix(g, h, fp.x), fp.y),
        fp.z
    );
}

float fbm(vec3 x, float H) {
    float G = exp2(-H);
    float f = 1.0;
    float a = 1.0;
    float t = 0.0;
    for (int i = 0; i < 2; i++)
    {
        t += a * cnoise(f * x);
        f *= 2.0;
        a *= G;
    }
    return t;
}

float sdBox(vec3 p, vec3 b)
{
    float r = 0.1;
    vec3 q = abs(p) - b + r;
    return length(max(q, 0.0)) + min(max(q.x, max(q.y, q.z)), 0.0) - r;
}

vec3 map(vec3 p) {
    p.y = abs(p.y);
    vec3 op = p;

    float an = 6.283185 / (15.0);

    float a = atan(p.x, p.z);
    float s = round(a / an);
    float os = s;
    float ra = s * an;

    p = p * rot(0.0, 0.0, ra);
    p.xz = p.xz - 30.0 * round(p.xz / 30.0);
    if (p.y < 3.0) {
        p.y = p.y - 1 * round(p.y / 1.0);
    }

    an = 6.283185 / (12.0);
    a = atan(p.x, p.z);
    s = round(a / an);
    ra = s * an;

    p = p * rot(0.0, 0.0, ra);

    float dBrick = sdBox(p - vec3(0.1, 0.1, 3.0), vec3(0.5, 0.2, 0.2)) - fbm(p * 20.0 * rot(0.3, .2, 0.1), 0.5) * 0.01;
    float ground = op.y + 5.0 - fbm(op * 3.0, 0.7) * 0.2;

    float d = min(dBrick, ground);
    // d = dBrick;

    float v = 0.0;
    if (d == dBrick) {
        v = 1.0;
    } else if (d == ground) {
        v = 5.0;
    }

    return vec3(d, v, dBrick);
}
vec3 map2(in vec3 p)
{
    float d = sdBox(p, vec3(4.0) * fbm(p, 0.8));
    vec3 res = vec3(d, 1.0, 0.0, 0.0);

    float s = 1.0;
    for (int m = 0; m < 3; m++)
    {
        p = p * rot(0.4, 0.2, 0.3);
        p.y *= 1.2;
        p.z *= 0.7;
        vec3 a = mod(p * s, 2.0) - 1.0;
        s *= 3.0;
        vec3 r = abs(1.0 - 3.0 * abs(a));

        float da = max(r.x, r.y);
        float db = max(r.y, r.z);
        float dc = max(r.z, r.x);
        float c = (min(da, min(db, dc)) - 1.0) / s;

        if (c > d)
        {
            d = c;
            res = vec3(d, 0.2 * da * db * dc, (1.0 + float(m)) / 4.0, 0.0);
        }
    }

    return res;
}

vec3 normal(vec3 p) {
    float ep = 0.001;
    vec2 h = vec2(ep, 0.0);
    return normalize(vec3(
            map(p + h.xyy).x - map(p - h.xyy).x,
            map(p + h.yxy).x - map(p - h.yxy).x,
            map(p + h.yyx).x - map(p - h.yyx).x
        ));
}

vec3 brick(float s) {
    vec3 a = vec3(0.027314467,0.0,0.0);
    vec3 b = vec3(0.09088415,0.035168752,0.14184175);
    vec3 c = vec3(0.2240782,0.29684663,0.16193646);
    vec3 d = vec3(0.0, 0.0, 0.0);
    return a + b * cos(6.283185 * (s * c + d));
}

vec3 sky(float s) {
    vec3 a = vec3(0.11355171,0.5361374,0.0) * 2.0 - 1.0;
    vec3 b = vec3(0.3952538,0.0,0.29543316);
    vec3 c = vec3(0.30681217,0.0,0.25450206);
    vec3 d = vec3(0.0, 0.0, 0.0);


    return a + b * cos(6.283185 * (s * c + d));
}
vec3 c1(float s) {
    vec3 a = vec3(0.0, 0.0, 0.0);
    vec3 b = vec3(0.2, 0.3, 0.2);
    vec3 c = vec3(0.1, 0.1, 0.2);
    vec3 d = vec3(0.0, 0.0, 0.0);


    return a + b * cos(6.283185 * (s * c + d));
}
vec3 c2(float s) {
    vec3 a = vec3(0.2, 0.2, 0.2);
    vec3 b = vec3(0.3, 0.1, 0.1);
    vec3 c = vec3(0.2, 0.1, 0.2);
    vec3 d = vec3(0.0, 0.0, 0.0);

    return a + b * cos(6.283185 * (s * c + d));
}
vec3 c3(float s) {
    vec3 a = vec3(0.0, 0.0, 0.0);
    vec3 b = vec3(0.1, 0.1, 0.3);
    vec3 c = vec3(0.3, 0.2, 0.1);
    vec3 d = vec3(0.0, 0.0, 0.0);

    return a + b * cos(6.283185 * (s * c + d));
}
vec3 c4(float s) {
    vec3 a = slider1.xyz * 2.0 - 1.0;
    vec3 b = slider2.xyz;
    vec3 c = slider3.xyz;
    vec3 d = slider4.xzy;

    return a + b * cos(6.283185 * (s * c + d));
}

vec3 march(vec3 ro, vec3 rd) {
    float t = 0.0;
    vec3 p = vec3(0.0);
    float min_d = 1000000.0;
    vec3 d = vec3(0.0);

    for (int i = 0; i < 100; ++i) {
        p = ro + rd * t;

        d = map(p);

        t += d.x * 0.8;
        min_d = min(min_d, d.z);
        if (d.x < 0.001) {
            vec3 n = normal(p);
            vec3 sun = vec3 (10.0, 200.0 , 10.0);
            vec3 sun_dir = normalize(sun - p);
            float i = dot(n, sun_dir);
            if (p.y < 0.0) {
                vec3 c = vec3(0.0);
                vec3 sky_hit = rayHitY(ro, rd, 60.0);
                sky_hit = rayHitY(ro, rd, fbm(sky_hit*0.2+iTime, 0.5)*10.0 + 40.0);

                if (fbm(p*30.0+iTime, 0.4) < (0.5 - fbm(sky_hit+iTime, 0.1)*0.2)) {
                    c = sky(fbm(sky_hit*0.2+iTime, 0.7));
                } else {
                    c = brick(fbm(p, 0.6));
                }
                // c.x = discretizeToValue(c.x, -1.0, 1.0, 4);
                // c.y = discretizeToValue(c.y, -1.0, 1.0, 4);
                // c.z = discretizeToValue(c.z, -1.0, 1.0, 4);

                return c;
            } else {
                vec3 c = brick(t);
                c.x *= i;
                return c;
            }
        }
        if (t > 100.0) {
            break;
        }
    }

    vec3 sky_hit = rayHitY(ro, rd, 60.0);
    sky_hit = rayHitY(ro, rd, fbm(sky_hit*0.2+iTime, 0.5)*10.0 + 40.0);
    if (p.y > 0.0) {
        vec3 s = sky(fbm(sky_hit*0.01, 0.2));
        return s;
    } else {
        return sky(fbm(sky_hit*0.01, 0.2))*fbm(sky_hit*0.02, 0.7);
    }
}

void main() {
    vec3 rd = vec3(uv, -1.0);
    rd -= 0.5;
    rd *= 2.0;
    rd.y *= -1.0;
    rd.x *= iResolution.x / iResolution.y;
    // rd.x *= -1.0;
    rd.z = -1.0;
    rd = normalize(rd);
    // rd.y -= 1.0;
    rd = rd * rot(slider2.x * 10.0, slider2.y * 10.0, slider2.z * 10.0);

    vec3 ro = vec3(0.0, 7.0 + 10.0 * (slider2.w - 0.5), 10.0 + (slider1.w - 0.5) * 10.0);
    fragColor = vec4(oklab2srgb(march(ro, rd)), 1.0);
    // fragColor = vec4(oklab2srgb(c4(uv.x)), 0.0);
}
