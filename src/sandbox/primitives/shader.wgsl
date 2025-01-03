@vertex
fn vs_main(@builtin(vertex_index) index: u32) -> @builtin(position) vec4<f32> {
    var pos = array<vec2<f32>, 6>(
        vec2f(-0.2, 0.9),
        vec2f(-0.8, 0.7),
        vec2f(-0.1, 0.05),

        vec2f(0.9, 0.3),
        vec2f(-0.3, -0.4),
        vec2f(0.3, -0.6),
    );

    return vec4<f32>(pos[index], 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 1.0, 0.0, 1.0);
}
