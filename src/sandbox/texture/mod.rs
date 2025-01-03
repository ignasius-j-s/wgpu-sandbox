use crate::{
    graphics::Renderable,
    vertices::{Vertex, VertexPosTex},
};
use texture::Texture;
use wgpu::{include_wgsl, util::DeviceExt};

pub mod texture;

pub struct Sandbox {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    indices_len: u32,
}

impl Sandbox {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        view_format: wgpu::TextureFormat,
    ) -> Self {
        let shader = device.create_shader_module(include_wgsl!("shader.wgsl"));
        let texture =
            Texture::from_jpeg(device, queue, "src/sandbox/texture/funyarinpa.jpg").unwrap();

        let vertices = &[
            // top-left
            VertexPosTex {
                position: [-1.0, 1.0],
                texture_coord: [0.0, 0.0],
            },
            // top-right
            VertexPosTex {
                position: [1.0, 1.0],
                texture_coord: [1.0, 0.0],
            },
            // bottom-left
            VertexPosTex {
                position: [-1.0, -1.0],
                texture_coord: [0.0, 1.0],
            },
            // bottom-right
            VertexPosTex {
                position: [1.0, -1.0],
                texture_coord: [1.0, 1.0],
            },
        ];
        let indices: &[u16] = &[0, 2, 1, 1, 2, 3];

        let bind_group_layout = Texture::bind_group_layout(device);

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("render pipeline layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("pipeline layout"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[VertexPosTex::desc()],
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

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("vertex buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("texture bind group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
        });

        Self {
            pipeline,
            vertex_buffer,
            index_buffer,
            bind_group,
            indices_len: indices.len() as _,
        }
    }
}

impl Renderable for Sandbox {
    fn render(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.indices_len, 0, 0..1);
    }
}
