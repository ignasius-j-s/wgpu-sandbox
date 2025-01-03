use winit::event_loop::EventLoop;
// use winit::platform::android::activity::AndroidApp;

mod app;
pub mod graphics;
pub mod sandbox;
pub mod vertices;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = app::App::default();

    event_loop.run_app(&mut app).unwrap();
}

// #[allow(dead_code)]
// #[cfg(target_os = "android")]
// #[no_mangle]
// fn android_main(app: AndroidApp) {
//     use winit::event_loop::EventLoopBuilder;

//     let event_loop = EventLoopBuilder::new().with_android_app(app).build();
//     let mut app = app::App::default();

//     event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
//     event_loop.run_app(&mut app);
// }
