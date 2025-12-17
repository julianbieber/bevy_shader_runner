struct CameraViewProj {
    clip_from_world: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera_view: CameraViewProj;

struct VertexIn {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,   // present to match your GLSL input (unused here)
    @location(2) uv: vec2<f32>,
};

struct VertexOut {
    @location(0) v_uv: vec2<f32>,
    @builtin(position) position: vec4<f32>,
};

@vertex
fn main(in: VertexIn) -> VertexOut {
    var out: VertexOut;
    out.v_uv = in.uv;
    out.position = camera_view.clip_from_world * vec4<f32>(in.position, 1.0);
    return out;
}
