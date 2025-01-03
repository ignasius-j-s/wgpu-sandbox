struct VertexInput {
    @location(0) pos: vec2<f32>,
    @location(1) tex_coord: vec2<f32>
}

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    out.pos = vec4f(in.pos, 0.0, 1.0);
    out.tex_coord = in.tex_coord;

    return out;
}

@group(0) @binding(0)
var texture: texture_2d<f32>;

@group(0) @binding(1)
var texture_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(texture, texture_sampler, in.tex_coord);
}
