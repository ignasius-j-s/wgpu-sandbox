struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
    @location(0) pos: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) col: vec4<f32>,
}

@group(0) @binding(0)
var<uniform> colors: array<vec4<f32>, 3>;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    out.pos = vec4f(in.pos, 0.0, 1.0);
    out.col = colors[in.vertex_index];

    return out;
}


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.col;
}
