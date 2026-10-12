@group(2) @binding(0) var<uniform> iTime: f32;
@group(2) @binding(1) var<uniform> iResolution: vec2f;

fn rot(x: f32, y: f32, z: f32) -> mat3x3f {
    return mat3x3f(1.0, 0.0, 0.0, 0.0, cos(x), -sin(x), 0.0, sin(x), cos(x)) *
           mat3x3f(cos(y), 0.0, -sin(y), 0.0, 1.0, 0.0, sin(x), 0.0, cos(x)) *
           mat3x3f(cos(z), -sin(z), 0.0, sin(z), cos(z), 0.0, 0.0, 0.0, 1.0);
}

fn dotnoise(p: vec3f) -> f32 {
    var x = p;
    var acc = 0.0;
    for (var i = 0; i < 3; i++) {
        x = rot(0.1, 0.2, 0.3) * x;
        acc += dot(cos(x), cos(x.yzx));
    }
    return acc / 3.0;
}

fn simpleMap(p: vec3f) -> f32 {
    return length(p) - dotnoise(sin(p + iTime)) - 1.0;
}

fn simpleNormal(p: vec3f) -> vec3f {
    let ep = 0.001;
    let h = vec2f(ep, 0.0);
    return normalize(vec3f(
        simpleMap(p + h.xyy) - simpleMap(p - h.xyy),
        simpleMap(p + h.yxy) - simpleMap(p - h.yxy),
        simpleMap(p + h.yyx) - simpleMap(p - h.yyx)
    ));
}

fn map(q: vec3f) -> f32 {
    var p = q;
    for (var i = 1; i < 4; i++) {
        let fi = f32(i);
        p -= simpleNormal(p * fi) * dotnoise(p * fi) * 1.0 / fi;
    }
    return simpleMap(p);
}

fn normal(p: vec3f) -> vec3f {
    let ep = 0.001;
    let h = vec2f(ep, 0.0);
    return normalize(vec3f(
        map(p + h.xyy) - map(p - h.xyy),
        map(p + h.yxy) - map(p - h.yxy),
        map(p + h.yyx) - map(p - h.yyx)
    ));
}

fn march(ro: vec3f, rd: vec3f) -> vec3f {
    var t = 0.0;
    for (var i = 0; i < 100; i++) {
        let p = ro + rd * t;
        let d = map(p);
        if d < 0.001 {
            let n = normal(p);
            let l = dot(n, normalize(vec3f(1.0, sin(iTime), -0.4)));
            return vec3f(1.0) * max(l, 0.2);
        }
        t += abs(d);
    }
    return vec3f(0.0);
}

@fragment
fn fragment(@location(0) uv: vec2f) -> @location(0) vec4f {
    let ro = vec3f(0.0, 0.0, -10.0);
    var rd = vec3f(uv.x, uv.y, 0.0);
    rd -= 0.5;
    rd *= 2.0;
    rd.y *= -1.0;
    rd.x *= iResolution.x / iResolution.y;
    rd.z = 1.0;
    rd = normalize(rd);
    return vec4f(march(ro, rd), 1.0);
}
