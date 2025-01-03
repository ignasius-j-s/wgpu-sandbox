use wgpu::include_wgsl;

use crate::graphics::Renderable;

pub struct Sandbox {
    pipeline: wgpu::RenderPipeline,
}

impl Sandbox {
    pub fn new(device: &wgpu::Device, view_format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(include_wgsl!("shader.wgsl"));

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("pipeline layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("render pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: view_format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: Default::default(),
            multisample: Default::default(),
            depth_stencil: None,
            multiview: None,
            cache: None,
        });

        Self { pipeline }
    }
}
impl Renderable for Sandbox {
    fn render(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.draw(0..3, 0..1);
    }
}
