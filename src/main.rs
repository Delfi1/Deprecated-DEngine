#[macro_use]
extern crate glium;
extern crate winit;

use glium::{
    backend::glutin::SimpleWindowBuilder,
    Surface
};
use objects::Object3D;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop
};

mod scenes;

fn rgba_to_float(red: i64, green: i64, blue: i64, alpha: f64) -> (f32, f32, f32, f32) {
    let r: f32 = red as f32 / 255.0 as f32;
    let g: f32 = green as f32 / 255.0 as f32;
    let b: f32 = blue as f32 / 255.0 as f32;

    return (r, g, b, alpha as f32);
}

fn main() {
    let event_loop = EventLoop::new(); // Главный цикл отрисовки окна

    let (_window, _display) = SimpleWindowBuilder::new()
        .with_inner_size(500, 500)
        .with_title("Engine")
        .build(&event_loop);

    _window.set_min_inner_size(Some(winit::dpi::LogicalSize::new(350.0, 250.0)));

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        control_flow.set_wait();
    
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("Closing Application");
                control_flow.set_exit();
            },
            Event::RedrawEventsCleared => {
                _window.request_redraw();
            },
            Event::RedrawRequested(_) => {
                // Отрисовка кадра
                let mut frame = _display.draw();
                frame.clear_color_and_depth(rgba_to_float(18, 18, 24, 1.0), 1.0);

                let perspective = {
                    let (width, height) = frame.get_dimensions();
                    let aspect_ratio = height as f32 / width as f32;

                    let fov: f32 = 3.141592 / 3.0;
                    let zfar = 1024.0;
                    let znear = 0.1;

                    let f = 1.0 / (fov / 2.0).tan();

                    [
                        [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                        [         0.0         ,     f ,              0.0              ,   0.0],
                        [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
                        [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
                    ]
                };

                let global_light = [-1.0, 0.4, 0.9f32];

                let params = glium::DrawParameters {
                    depth: glium::Depth {
                        test: glium::draw_parameters::DepthTest::IfLess,
                        write: true,
                        .. Default::default()
                    },
                    //backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
                    .. Default::default()
                };

                // Окончание отрисовки кадра
                frame.finish().unwrap();
            },
            _ => ()
        }
    });
    
}