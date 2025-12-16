struct TimeUniform {
    time : f32,
};

struct ResolutionUniform {
    resolution: vec2f,
};

@group(2) @binding(0)
var<uniform> u_time : TimeUniform;

@group(2) @binding(1)
var<uniform> u_res : ResolutionUniform;

struct VSOutput {
    @location(0) v_uv : vec2<f32>,
};

@fragment
fn fs_main(in: VSOutput) -> @location(0) vec4<f32> {
    let t = abs(sin(u_time.time));
    return vec4<f32>(in.v_uv, t, 1.0);
}
