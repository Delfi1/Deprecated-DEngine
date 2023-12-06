#[macro_use]

extern crate glium;
use glium::{
    backend::glutin::SimpleWindowBuilder,
    Surface
};

extern crate winit;
use scenes::Camera;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, ControlFlow}
};

use std::time::Instant;

mod scenes;

const TARGET_FPS: u64 = 120;

fn main() {
    let event_loop = EventLoop::new(); // Главный цикл отрисовки окна

    let (_window, _display) = SimpleWindowBuilder::new()
        .with_inner_size(700, 500)
        .with_title("Engine")
        .build(&event_loop);

    _window.set_min_inner_size(Some(winit::dpi::LogicalSize::new(350.0, 250.0)));

    let global_light = [1.4, 0.4, 0.7f32];
    let global_camera = Camera::new([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 1.0, 1.0, 1.0);

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        .. Default::default()
    };

    let mut current_scene = scenes::Scene::new(global_camera, global_light, params);

    scenes::Cube::new(&mut current_scene, [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]);

    event_loop.run(move |event, _target, control_flow| {
        let start_time = Instant::now();
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
            Event::RedrawRequested(_) => {
                // Отрисовка сцены
                current_scene.render(&_display);
            },
            _ => ()
        };
        // FPS 
        match *control_flow {
            ControlFlow::Exit => (),
            _ => {
                _window.request_redraw();

                let elapsed_time = Instant::now().duration_since(start_time).as_millis() as u64;
    
                let wait_millis = match 1000 / TARGET_FPS >= elapsed_time {
                    true => 1000 / TARGET_FPS - elapsed_time,
                    false => 0
                };
                let new_inst = start_time + std::time::Duration::from_millis(wait_millis);
                *control_flow = ControlFlow::WaitUntil(new_inst);
            }
        }
    });
    
}