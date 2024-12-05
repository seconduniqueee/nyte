struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>
};

struct ColoredVertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) fill_color: vec4<f32>,
}

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;

    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;

    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}

@vertex
fn vs_main_two(
    @builtin(vertex_index) in_vertex_index: u32,
) -> ColoredVertexOutput {
    var out: ColoredVertexOutput;
//
//    let x = f32(1 - i32(in_vertex_index)) * 0.5;
//    let y = f32(i32((in_vertex_index & 1u) ^ 1u) * 2 + 1) * 0.5;

    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    let clr = f32(in_vertex_index) / 10;

    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.fill_color = vec4<f32>(clr, clr, clr, 1.0);

    return out;
}

@fragment
fn fs_main_two(in: ColoredVertexOutput) -> @location(0) vec4<f32> {
    return in.fill_color;
}