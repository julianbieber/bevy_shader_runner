@group(2) @binding(0) var<uniform> iTime: f32;
@group(2) @binding(1) var<uniform> iResolution: vec2f;
@group(2) @binding(2) var<uniform> slider1: vec4f;
@group(2) @binding(3) var<uniform> slider2: vec4f;
@group(2) @binding(4) var<uniform> slider3: vec4f;
@group(2) @binding(5) var<uniform> slider4: vec4f;
@group(2) @binding(6) var<uniform> slider5: vec4f;

fn cos_s(x: vec3f) -> vec3f {
    return (cos(x) * 0.5) + 0.5;
}

fn color(base: vec3f, scale: vec3f, freq: vec3f, freq_o: vec3f, x: f32) -> vec3f {
    return base + scale * cos_s(freq * x + freq_o);
}

fn rot(r: vec3f) -> mat3x3f {
    return mat3x3f(1.0, 0.0, 0.0, 0.0, cos(r.x), -sin(r.x), 0.0, sin(r.x), cos(r.x)) *
           mat3x3f(cos(r.y), 0.0, -sin(r.y), 0.0, 1.0, 0.0, sin(r.y), 0.0, cos(r.y)) *
           mat3x3f(cos(r.z), -sin(r.z), 0.0, sin(r.z), cos(r.z), 0.0, 0.0, 0.0, 1.0);
}

fn dotnois(p: vec3f) -> f32 {
    var x = p;
    var a = 0.0;
    for (var i = 0; i < 3; i++) {
        x = x * rot(x);
        a += dot(cos(x), cos(x.yzx));
        x = x * x;
    }
    return a;
}

fn flame(p: vec3f) -> vec3f {
    return color(slider1.xyz, vec3f(0.01, 0.02, 0.1), vec3f(0.01, 0.01, 0.2), vec3f(iTime), dotnois(p + iTime));
}

fn map(p: vec3f) -> f32 {
    return length(p) - 6.0;
}

fn march(ro: vec3f, rd: vec3f) -> vec3f {
    var t = 0.0;
    var min_d = 10000.0;

    for (var i = 0; i < 100; i++) {
        let p = ro + rd * t;
        let d = map(p);

        t += d;
        if d < 0.001 {
            return flame(p);
        }
        min_d = min(min_d, d);
    }

    return flame(vec3f(min_d));
}

@fragment
fn fragment(@location(0) uv: vec2f) -> @location(0) vec4f {
    var rd = vec3f(uv, -1.0);
    rd -= 0.5;
    rd *= 2.0;
    rd.y *= -1.0;
    rd.x *= iResolution.x / iResolution.y;
    rd.z = 1.0;
    rd = normalize(rd);

    let ro = vec3f(0.0, 0.0, -10.0);
    return vec4f(march(ro, rd), 1.0);
}
