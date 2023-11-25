#[macro_use]
extern crate glium;
extern crate winit;

use glium::{
    backend::glutin::SimpleWindowBuilder,
    Surface
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop
};

mod objects;
use crate::objects::Object3D;
mod teapot;

fn view_matrix(position: &[f64; 3], direction: &[f64; 3], up: &[f64; 3]) -> [[f64; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}

fn main() {
    let event_loop = EventLoop::new(); // Главный цикл отрисовки окна

    let (_window, _display) = SimpleWindowBuilder::new()
        .with_inner_size(500, 500)
        .with_title("Engine")
        .build(&event_loop);

    _window.set_min_inner_size(Some(winit::dpi::LogicalSize::new(350.0, 250.0)));

    let test_cube = objects::Cube::new([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);

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
            Event::MainEventsCleared => {
                _window.request_redraw();
            },
            Event::RedrawRequested(_) => {
                // Отрисовка кадра
                let mut frame = _display.draw();
                frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

                let (width, height) = (_window.inner_size().width, _window.inner_size().height);
                //println!("{width}, {height}");
                let screen = [
                    [0.01, 0.0, 0.0, 0.0],
                    [0.0, 0.01, 0.0, 0.0],
                    [0.0, 0.0, 0.01, 0.0],
                    [0.0, 0.0, 0.0, 1.0f64]
                ];

                // Глобальный свет сцены
                let global_light = [-1.0, 0.4, 0.9f64];

                let view = view_matrix(&[2.0, -1.0, 1.0], &[-2.0, 1.0, 1.0], &[0.0, 1.0, 0.0]);
                
                let perspective = {
                    let (width, height) = frame.get_dimensions();
                    let aspect_ratio = height as f64 / width as f64;

                    let fov: f64 = 3.141592 / 3.0;
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

                let params = glium::DrawParameters {
                    depth: glium::Depth {
                        test: glium::draw_parameters::DepthTest::IfLess,
                        write: true,
                        .. Default::default()
                    },
                    .. Default::default()
                };

                test_cube.render(&_display, &mut frame, screen, view, perspective, global_light, &params);

                // Окончание отрисовки кадра
                frame.finish().unwrap();
            },
            _ => ()
        }
    });
    
}