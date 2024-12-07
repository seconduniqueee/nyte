struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct ColoredVertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@vertex
fn vs_main(model: VertexInput) -> ColoredVertexOutput {
    var out: ColoredVertexOutput;

    out.tex_coords = model.tex_coords;
    out.clip_position = vec4<f32>(model.position, 1.0);

    return out;
}

@fragment
fn fs_main(in: ColoredVertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}