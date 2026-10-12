@group(2) @binding(0) var<uniform> iTime: f32;
@group(2) @binding(1) var<uniform> iResolution: vec2f;

fn rot(x: f32, y: f32, z: f32) -> mat3x3f {
    return mat3x3f(1.0, 0.0, 0.0, 0.0, cos(x), -sin(x), 0.0, sin(x), cos(x)) *
           mat3x3f(cos(y), 0.0, -sin(y), 0.0, 1.0, 0.0, -sin(y), 0.0, cos(y)) *
           mat3x3f(cos(z), -sin(z), 0.0, sin(z), cos(z), 0.0, 0.0, 0.0, 1.0);
}

fn dotnoise(p: vec3f) -> f32 {
    var x = p;
    var a = 0.0;

    for (var i = 0; i < 5; i++) {
        let fi = f32(i);
        x = rot(fi, fi, fi) * x;
        a += dot(cos(x), cos(x.yzx));
    }

    return a * (1.0 / 3.0);
}

fn fbm_dot(x: vec3f) -> f32 {
    var acc = 0.0;
    let freq_m = 2.0;
    var freq = 1.0;
    let amp_m = 0.5;
    var amp = 10.0;

    for (var i = 0; i < 10; i++) {
        acc += abs(dotnoise(x * freq)) * amp;
        freq *= freq_m;
        amp *= amp_m;
    }
    return acc;
}

fn map(p: vec3f) -> f32 {
    return p.y - fbm_dot(vec3f(p.x, 0.0, p.z)) * 0.1;
}

fn normal_map(p: vec3f) -> vec3f {
    let e = 0.0001;
    let h = vec2f(e, 0.0);

    return normalize(vec3f(
        map(p + h.xyy) - map(p - h.xyy),
        map(p + h.yxy) - map(p - h.yxy),
        map(p + h.yyx) - map(p - h.yyx)
    ));
}

fn fog(p: vec3f) -> f32 {
    return dotnoise(p.zyx);
}

fn color_scale(base: vec3f, range: vec3f, freq: vec3f, offset: vec3f, x: f32) -> vec3f {
    return base + range * cos(freq * x + offset);
}

fn ground_color(p: vec3f) -> vec3f {
    return color_scale(vec3f(0.0, 0.5, 0.2), vec3f(0.2, 0.2, 0.4), vec3f(0.2), vec3f(iTime, iTime * 0.2, iTime * 0.5), p.y);
}

fn fog_c(i: f32) -> vec3f {
    return color_scale(vec3f(0.2, 0.2, 0.2), vec3f(0.5, 0.2, 0.4), vec3f(0.2), vec3f(iTime, iTime * 0.2, iTime * 0.5), i);
}

fn march(ro: vec3f, rd: vec3f) -> vec3f {
    var t = 0.0;
    var n = vec3f(0.0);
    let l = normalize(vec3f(100.0, 1.0, 0.0));

    var p = vec3f(0.0);
    for (var i = 0; i < 100; i++) {
        p = ro + rd * t;
        let d = map(p);

        if d < 0.001 {
            n = normal_map(p);
            break;
        }
        t += d;
    }

    let split = 10.0;

    let t_dist = min(t, 1000.0) / split;

    var fog_acc = 0.0;

    for (var i = 0.0; i < split; i += 1.0) {
        let local_t = t_dist * i;
        let fp = ro + rd * local_t;
        fog_acc += abs(fog(fp + iTime));
    }

    let fog_i = pow(2.23, -fog_acc);
    if t > 100.0 {
        return vec3f(fog_i);
    }

    return mix(ground_color(p) * dot(n, l) * ground_color(n), fog_c(fog_i), (1.0 - fog_i * (2.0 + iTime * 10.0)) * 0.5);
}

@fragment
fn fragment(@location(0) uv: vec2f) -> @location(0) vec4f {
    let ro = vec3f(0.0, -map(vec3f(0.0, 0.0, -10.0 + iTime)) + 1.6, -10.0 + iTime);
    var rd = vec3f(uv, 1.0);
    rd.x -= 0.5;
    rd.x *= 2.0;
    rd.y -= 0.5;
    rd.y *= -2.0;

    rd.x *= iResolution.x / iResolution.y;
    rd = rot(0.0, iTime * 0.1, 0.0) * normalize(rd);
    return vec4f(march(ro, rd), 1.0);
}
