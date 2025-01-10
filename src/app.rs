use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{Key, NamedKey},
    window::{Window, WindowAttributes},
};

use crate::graphics::GraphicsContext;

#[derive(Default)]
pub struct App {
    scale_factor: f64,
    window: Option<Arc<Window>>,
    graphics_context: Option<GraphicsContext>,
}

impl App {
    fn create_window(&mut self, event_loop: &ActiveEventLoop) {
        let window_attr = WindowAttributes::default()
            .with_title("learn wgpu")
            .with_inner_size(PhysicalSize::new(640, 480));

        let window = Arc::new(event_loop.create_window(window_attr).unwrap());
        self.window = Some(window);
    }

    fn create_graphics_context(&mut self, window: Arc<Window>) {
        self.graphics_context = Some(GraphicsContext::new(window));
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            self.create_window(event_loop);
        }

        let window = self.window.as_ref().unwrap().clone();

        if self.graphics_context.is_none() {
            self.create_graphics_context(window.clone());
        }

        let gfx_context = self.graphics_context.as_mut().unwrap();

        gfx_context.resume(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let window = self.window.clone().unwrap();
        let gfx_context = self.graphics_context.as_mut().unwrap();

        if window_id != window.id() {
            return;
        }

        match event {
            WindowEvent::Resized(physical_size) => {
                gfx_context.resize(physical_size);
                window.request_redraw();
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                gfx_context.render();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: Key::Named(NamedKey::Escape),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => event_loop.exit(),
            WindowEvent::KeyboardInput { event, .. } => {
                gfx_context.handle_input(event);
                window.request_redraw();
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                self.scale_factor = scale_factor
            }
            _ => (),
        }
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        let gfx_context = self.graphics_context.as_mut().unwrap();
        gfx_context.suspend();
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        println!("exiting app");
    }
}
