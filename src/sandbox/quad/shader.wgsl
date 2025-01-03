struct VertexInput {
    @location(0) pos: vec2<f32>,
    @location(1) col: vec3<f32>
}

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) col: vec4<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    out.pos = vec4f(in.pos, 0.0, 1.0);
    out.col = vec4f(in.col, 1.0);

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.col;
}
