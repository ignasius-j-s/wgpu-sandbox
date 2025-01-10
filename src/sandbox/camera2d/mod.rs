use camera::Camera2D;
use wgpu::{include_wgsl, util::DeviceExt};
use winit::{
    event::{ElementState, KeyEvent},
    keyboard::{KeyCode, PhysicalKey},
};

use crate::{
    graphics::Renderable,
    vertices::{Vertex, VertexPosCol},
};

pub mod camera;

pub struct Sandbox {
    pipeline: wgpu::RenderPipeline,
    camera: Camera2D,
    vertex_buffer: wgpu::Buffer,
    camera_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    num_vertices: u32,
}

impl Sandbox {
    pub fn new(device: &wgpu::Device, view_format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(include_wgsl!("shader.wgsl"));

        let vertices = &[
            VertexPosCol {
                position: [320.0, 140.0],
                color: [1.0, 1.0, 1.0],
            },
            VertexPosCol {
                position: [220.0, 290.0],
                color: [0.0, 1.0, 1.0],
            },
            VertexPosCol {
                position: [420.0, 290.0],
                color: [0.0, 0.0, 1.0],
            },
        ];

        let bind_group_layout = Camera2D::bind_group_layout(device);

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("pipeline layout"),
            bind_group_layouts: &[&bind_group_layout],
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

        let camera = Camera2D::new(640, 480, 1.0);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("camera buffer"),
            contents: bytemuck::cast_slice(&camera.camera_matrix().to_cols_array_2d()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("camera bind group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &camera_buffer,
                    offset: 0,
                    size: None,
                }),
            }],
        });

        Self {
            pipeline,
            vertex_buffer,
            camera,
            bind_group,
            camera_buffer,
            num_vertices: vertices.len() as _,
        }
    }
}

impl Renderable for Sandbox {
    fn handle_input(&mut self, key_event: KeyEvent, queue: &wgpu::Queue) {
        let KeyEvent {
            physical_key,
            state,
            ..
        } = key_event;

        if state == ElementState::Released {
            return;
        };

        match physical_key {
            PhysicalKey::Code(KeyCode::ArrowUp) => {
                self.camera.y -= 3.0;
            }
            PhysicalKey::Code(KeyCode::ArrowDown) => {
                self.camera.y += 3.0;
            }
            PhysicalKey::Code(KeyCode::ArrowLeft) => {
                self.camera.x -= 3.0;
            }
            PhysicalKey::Code(KeyCode::ArrowRight) => {
                self.camera.x += 3.0;
            }
            _ => (),
        }

        queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&self.camera.camera_matrix().to_cols_array_2d()),
        );
    }

    fn render(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..self.num_vertices, 0..1);
    }
}
