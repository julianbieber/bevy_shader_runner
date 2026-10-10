@group(2) @binding(0) var<uniform> iTime: f32;
@group(2) @binding(1) var<uniform> iResolution: vec2f;

@fragment
fn fragment(@location(0) uv: vec2f) -> @location(0) vec4f {
    return vec4f(uv * 4.0, abs(sin(iTime)), 1.0);
}
