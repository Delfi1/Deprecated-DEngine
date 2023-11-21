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

mod rendering;

fn main() {
    let event_loop = EventLoop::new(); // Главный цикл отрисовки окна

    let (_window, _display) = SimpleWindowBuilder::new()
        .with_inner_size(700, 500)
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
            Event::MainEventsCleared => {
                _window.request_redraw();
            },
            Event::RedrawRequested(_) => {
                // Отрисовка кадра
                let mut frame = _display.draw();
                frame.clear_color(0.0, 0.0, 0.0, 1.0);
                

                // Окончание отрисовки кадра
                frame.finish().unwrap();
            },
            _ => ()
        }
    });
    
}