use crate::vertices::{Vertex, VertexPosCol};
use wgpu::{include_wgsl, util::DeviceExt};

use crate::graphics::Renderable;

pub struct Sandbox {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    vertices_len: u32,
}

impl Sandbox {
    pub fn new(device: &wgpu::Device, view_format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(include_wgsl!("shader.wgsl"));

        let vertices = &[
            // tri1
            VertexPosCol {
                position: [-0.7, -0.7], // bottom-left
                color: [1.0, 0.0, 0.0],
            },
            VertexPosCol {
                position: [0.7, -0.7], // bottom-right
                color: [0.0, 1.0, 0.0],
            },
            VertexPosCol {
                position: [-0.7, 0.7], // top-left
                color: [1.0, 1.0, 0.0],
            },
            // tri2
            VertexPosCol {
                position: [-0.7, 0.7], // top-left
                color: [1.0, 1.0, 0.0],
            },
            VertexPosCol {
                position: [0.7, -0.7], // bottom-right
                color: [0.0, 1.0, 0.0],
            },
            VertexPosCol {
                position: [0.7, 0.7], // top-right
                color: [0.0, 0.0, 1.0],
            },
        ];

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
                buffers: &[VertexPosCol::desc()],
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
            primitive: wgpu::PrimitiveState::default(),
            multisample: Default::default(),
            depth_stencil: None,
            multiview: None,
            cache: None,
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("vertex buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            pipeline,
            vertex_buffer,
            vertices_len: vertices.len() as _,
        }
    }
}
impl Renderable for Sandbox {
    fn render(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..self.vertices_len, 0..1);
    }
}
