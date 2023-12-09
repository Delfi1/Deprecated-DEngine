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
    event::{Event, WindowEvent, KeyboardInput, VirtualKeyCode},
    event_loop::{EventLoop, ControlFlow, self, EventLoopBuilder},
    window::{Window, self, Fullscreen}, dpi::PhysicalSize, monitor::{MonitorHandle, VideoMode}
};

mod input;
use input::Key;

const TARGET_FPS: u64 = 120;

fn main() {
    // Основной цикл для программы
    let event_loop = EventLoop::new();
    // Создаем базовое окно.
    let (window, display) = SimpleWindowBuilder::new().build(&event_loop);

    // Настраиваем Окно
    window.set_title("DEngine");
    window.set_inner_size(PhysicalSize::new(700, 500)); // Размер окна
    window.set_min_inner_size(Some(PhysicalSize::new(350, 250))); // Минимальный размер окна

    // Кнопки
    let mut f11_bind = Key::new(0.5, VirtualKeyCode::F11);
    let mut f3_bind = Key::new(0.5, VirtualKeyCode::F3);

    // Запускаем основной цикл.
    event_loop.run(move |event, _target, control_flow| {
        let start_time = Instant::now(); 
        control_flow.set_poll(); 
        control_flow.set_wait();

        if f11_bind.check_event(&event) {
            if window.fullscreen().is_none() {
                let mode = window.current_monitor().unwrap().video_modes().next().unwrap();
                window.set_fullscreen(Some(Fullscreen::Exclusive(mode)));
            } else {
                window.set_fullscreen(None);
            }
        }

        if f3_bind.check_event(&event) {
            println!("OpenGL version: {:?}", display.get_opengl_version_string());
        }

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("Закрытие окна...");
                control_flow.set_exit_with_code(0);
            },
            
            
            // Отрисовка окна
            Event::RedrawRequested(_) => {
                let mut frame = display.draw(); // Отрисовка кадра
                frame.clear_color(22.0/255.0, 22.0/255.0, 29.0/255.0, 1.0);

                frame.finish().unwrap();
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
                //println!("{}", wait_nanos);
                let new_inst = start_time + std::time::Duration::from_nanos(wait_nanos);
                *control_flow = ControlFlow::WaitUntil(new_inst); // Ожидание в наносекундах.
            }
        }
    });
}