use std::sync::Arc;

use winit::{dpi::PhysicalSize, window::Window};

pub trait Renderable {
    fn update(&mut self) {}
    fn render(&self, render_pass: &mut wgpu::RenderPass);
}

pub struct GraphicsContext {
    #[allow(unused)]
    instance: wgpu::Instance,
    #[allow(unused)]
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: Option<wgpu::Surface<'static>>,
    renderable: Box<dyn Renderable>,
    #[allow(unused)]
    view_format: wgpu::TextureFormat,
    surface_config: wgpu::SurfaceConfiguration,
}

impl GraphicsContext {
    pub fn new(window: Arc<Window>) -> GraphicsContext {
        log::debug!("initializing wgpu");
        let instance = wgpu::Instance::new(Default::default());

        log::debug!("creating wgpu surface");
        let surface = instance
            .create_surface(window.clone())
            .expect("create surface failed");

        log::debug!("creating wgpu adapter");
        let req_adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::None,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        });
        let adapter = pollster::block_on(req_adapter).expect("create adapter failed");

        log::debug!("creating wgpu device");
        let req_device = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_defaults(),
                memory_hints: Default::default(),
            },
            None,
        );
        let (device, queue) = pollster::block_on(req_device).expect("create device failed");

        log::debug!("configuring wgpu surface");
        let size = window.inner_size();
        let view_format = surface.get_capabilities(&adapter).formats[0];
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: view_format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: 2,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        // change this to switch between examples
        let example = crate::sandbox::texture::Sandbox::new(&device, &queue, view_format);

        Self {
            instance,
            adapter,
            device,
            queue,
            renderable: Box::new(example),
            surface: Some(surface),
            view_format,
            surface_config: config,
        }
    }

    pub fn render(&mut self) {
        let surface = self.surface.as_ref().unwrap();
        let render_texture = surface
            .get_current_texture()
            .expect("failed to acquire next swapchain texture");
        let render_texture_view = render_texture.texture.create_view(&Default::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("command encoder"),
            });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &render_texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        self.renderable.render(&mut render_pass);
        drop(render_pass);

        self.queue.submit(Some(encoder.finish()));
        render_texture.present();
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        let surface = self.surface.as_ref().unwrap();
        self.surface_config.width = size.width.max(1);
        self.surface_config.height = size.height.max(1);
        surface.configure(&self.device, &self.surface_config);
    }

    #[cfg(not(target_os = "android"))]
    pub fn resume(&mut self, _window: Arc<Window>) {}

    #[cfg(target_os = "android")]
    pub fn resume(&mut self, window: Arc<Window>) {
        let surface = self.instance.create_surface(window.clone()).unwrap();
        let size = window.inner_size();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.view_format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: 2,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        };

        surface.configure(&self.device, &config);
        self.surface = Some(surface);
    }

    #[cfg(not(target_os = "android"))]
    pub fn suspend(&mut self) {}

    #[cfg(target_os = "android")]
    pub fn suspend(&mut self) {
        self.surface = None;
    }
}
