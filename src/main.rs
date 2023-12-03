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

    let global_camera = Camera::new([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 1.0, 1.0, 1.0);

    let mut test_scene = scenes::Scene::new(global_camera);

    scenes::Cube::new(&mut test_scene, [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);

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
                frame.clear_color_and_depth(rgba_to_float(22, 22, 29, 1.0), 0.8);

                test_scene.render();
                
                // Окончание отрисовки кадра
                frame.finish().unwrap();
            },
            _ => ()
        }
    });
    
}