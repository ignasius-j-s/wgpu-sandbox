use glam::{vec3, Mat4};

#[derive(Debug)]
pub struct Camera2D {
    pub width: f32,
    pub height: f32,
    pub scale: f32,
    pub x: f32,
    pub y: f32,
}

impl Camera2D {
    pub fn new(width: u32, height: u32, scale: f32) -> Self {
        Self {
            width: width as f32,
            height: height as f32,
            scale,
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::orthographic_rh(0.0, self.width, self.height, 0.0, -1.0, 1.0)
    }

    pub fn view_matrix(&self) -> Mat4 {
        let translation = Mat4::from_translation(-vec3(self.x, self.y, 1.0));
        let scale = Mat4::from_scale(vec3(self.scale, self.scale, 1.0));

        scale * translation
    }

    pub fn camera_matrix(&self) -> Mat4 {
        (self.projection_matrix() * self.view_matrix()).transpose()
    }

    pub fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        use wgpu::BindGroupLayoutEntry as Entry;

        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[Entry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        })
    }
}
