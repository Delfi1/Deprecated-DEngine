use std::time::Instant;

extern crate glium;
use glium::{
    backend::glutin::SimpleWindowBuilder,
    Surface,
    Display,
    glutin::{surface::WindowSurface, api::egl::display}
};

extern crate winit;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, ControlFlow, self, EventLoopBuilder},
    window::{Window, self}, dpi::PhysicalSize
};

const TARGET_FPS: u64 = 120;

fn main() {
    // Основной цикл для программы
    let event_loop = EventLoop::new();
    // Создаем базовое окно.
    let (mut window, display) = SimpleWindowBuilder::new().build(&event_loop);

    // Настраиваем Окно
    window.set_title("DEngine");
    window.set_inner_size(PhysicalSize::new(700, 500)); // Размер окна
    window.set_min_inner_size(Some(PhysicalSize::new(350, 250))); // Минимальный размер окна

    // Запускаем основной цикл.
    event_loop.run(move |event, _, control_flow| {
        let start_time = Instant::now(); 
        control_flow.set_poll(); 
        control_flow.set_wait();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("Закрытие окна...");
                control_flow.set_exit();
            },
            // Отрисовка окна
            Event::RedrawRequested(_) => {


            },
            _ => ()
        };

        // Установка задержки для отрисовки (FPS)
        match *control_flow {
            ControlFlow::Exit => (),
            _ => {
                window.request_redraw(); // Запрос на отрисовку.
                let elapsed_time = Instant::now().duration_since(start_time).as_nanos() as u64;

                let wait_nanos = match 1_000_000_000 / TARGET_FPS >= elapsed_time {
                    true => 1_000_000_000 / TARGET_FPS - elapsed_time,
                    false => 0
                };
                
                let new_inst = start_time + std::time::Duration::from_nanos(wait_nanos);
                *control_flow = ControlFlow::WaitUntil(new_inst); // Ожидание в наносекундах.
            }
        }
    });
}