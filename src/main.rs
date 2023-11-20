extern crate glium;
extern crate winit;

use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new();

    let _window = WindowBuilder::new()
        .with_inner_size(winit::dpi::Size::Logical(winit::dpi::LogicalSize::new(700.0, 500.0)))
        .with_title("Engine")
        .build(&event_loop)
        .unwrap();

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
                
            },
            _ => ()
        }
    });
    
}