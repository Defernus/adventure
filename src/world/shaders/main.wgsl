// Vertex shader
struct CameraUniform {
    view_proj: mat4x4<f32>;
};

struct SunUniform {
    direction: vec3<f32>;
};

[[group(0), binding(0)]]
var<uniform> camera: CameraUniform;

[[group(1), binding(0)]]
var<uniform> sun: SunUniform;

struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] normal: vec3<f32>;
    [[location(2)]] color: vec3<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] color: vec3<f32>;
};

let SUN_VEC: vec3<f32> = vec3<f32>(1.0, 0.5, 0.0);

[[stage(vertex)]]
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;

    let light = (dot(model.normal, sun.direction) + 1.0) / 2.0;
    out.color = model.color * light;

    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);

    return out;
}

// Fragment shader

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}