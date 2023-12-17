use glium::{Frame, Surface};
use glium::{
    backend::glutin::SimpleWindowBuilder,
    uniforms::AsUniformValue
};
// Open GL Wrapper

extern crate serde;
use serde::{Serialize, Deserialize};
use serde_json::json;
use erased_serde::serialize_trait_object;

use winit::dpi::PhysicalSize;
use winit::event::VirtualKeyCode;
use winit::window::Fullscreen;
use winit::{window, event_loop};
use winit::{
    event::Event,
    event::WindowEvent,
    event_loop::ControlFlow,
    event_loop::EventLoop
};

use std::marker::PhantomData;
use std::ptr::null;
use std::{
    fmt,
    fs,
    io, default
};

use std::f32::consts::PI;
use std::time::Instant;

use self::input::Key;

#[path ="../src/input.rs"]
mod input;

#[derive(Clone, Copy, Default, Serialize, Deserialize)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32
}

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self {x, y, z}
    }

    fn get(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    fn from(&mut self, vector: [f32; 3]) -> Self {
        Self {x: vector[0], y: vector[1], z: vector[2]}
    }
}

pub struct Camera {
    pub position: Vec3,
    direction: Vec3,

    fov: f32
}

impl Camera {
    pub fn new() -> Self {
        let position = Vec3::default();
        let direction = Vec3::default();

        let fov = 60.0 * PI / 180.0;

        Self {position, direction, fov}
    }

    // Degrees -> Radians
    pub fn set_fov(&mut self, degrees: f32) {
        let radians = degrees * PI / 180.0;
        self.fov = radians;
    }

    pub fn get_fov(&self) -> f32 {
        self.fov
    }
}

// Object Trait

pub trait Object: erased_serde::Serialize {
    fn set_position(&mut self, positiob: Vec3);
    fn get_position(&self) -> Vec3;

    fn set_indicies(&mut self); // 
    fn set_normales(&mut self); // 

    fn draw(&self);
}

// Objects

#[derive(Serialize, Deserialize)]
struct Cube {
    position: Vec3,
    rotation: Vec3,
    size: Vec3,

}

// Serealize Object Trait
serialize_trait_object!(Object);

pub struct World {
    objects: Vec<&'static dyn Object>
}

impl World {
    pub fn new() -> &'static mut Self {
        Box::leak(Box::new( Self { objects: Vec::new() }))
    }

    fn draw(&mut self, frame: &mut Frame) {

    }

    pub fn load(&mut self) {
        //self.object = ...
    }

    pub fn save(&self) {
        let data = json!(self.objects);
        // Self. objects to json
    }

    pub fn add_object(&mut self, object: &'static dyn Object) {
        self.objects.push(object)
    }
}

pub struct Settings {
    title: &'static str,

    window_size: PhysicalSize<u32>,
    min_window_size: PhysicalSize<u32>,

    max_fps: u32
}

impl Settings {
    pub fn new(title: &'static str, window_size: PhysicalSize<u32>, min_window_size: PhysicalSize<u32>, max_fps: u32) -> Self {
        Self {title, window_size, min_window_size, max_fps}
    }
}

impl Default for Settings {
    fn default() -> Self {
        let window_size = PhysicalSize::new(700, 500);
        let min_window_size = PhysicalSize::new(350, 250);

        Self {title: "DEngine", window_size, min_window_size, max_fps: 120 }
    }
}

pub struct Engine {
    pub camera: Camera,
    world: Option<&'static mut World>,
    pub settings: Settings
}

impl Engine {
    pub fn new() -> &'static mut Self {
        let camera = Camera::new();
        let world = None;

        let settings = Settings::default();

        Box::leak(Box::new(Self {camera, world, settings}))
    }

    fn get_delta_time(&self, start_time: Instant) -> u32 {
        let elapsed_time = Instant::now().duration_since(start_time).as_nanos() as u32;

        match 1_000_000_000 / self.settings.max_fps >= elapsed_time {
            true => 1_000_000_000 / self.settings.max_fps - elapsed_time,
            false => 0
        }
    }

    fn get_fps(&self, delta: u32) -> u32 {
        match delta > 0 {
            true => 1_000_000_000 / delta,
            false => 0
        }
    }

    pub fn run(&'static mut self) {
        let event_loop = EventLoop::new();

        let (window, display) = SimpleWindowBuilder::new().build(&event_loop);

        // Настройка окна
        window.set_title(self.settings.title);
        window.set_inner_size(self.settings.window_size);
        window.set_min_inner_size(Some(self.settings.min_window_size));

        let f11_key = Key::new(0.3, VirtualKeyCode::F11);

        event_loop.run(move |event, _, control_flow| {
            control_flow.set_wait();
            control_flow.set_poll();
            let start_time = Instant::now();

            // Смена полноэкранного режима 
            if f11_key.is_pressed(&event) {
                if window.fullscreen().is_none() {
                    let mode = window.current_monitor().unwrap().video_modes().next().unwrap();
                    window.set_fullscreen(Some(Fullscreen::Exclusive(mode)));
                } else {
                    window.set_fullscreen(None);
                }
            }

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    println!("Закрытие программы...");
                    control_flow.set_exit()
                },
                Event::RedrawRequested(_) => {
                    // Создание кадра
                    let mut frame = display.draw();
                    frame.clear_color(1.0, 1.0, 1.0, 1.0);

                    if self.world.is_some() {
                        self.world.as_mut().unwrap().draw(&mut frame);
                    }
                    // Завершение отрисовки кадра.
                    frame.finish().unwrap();
                }
                _ => (),
            }

            match *control_flow {
                ControlFlow::Exit => (),
                _ => {
                    window.request_redraw(); // Запрос на отрисовку.
                    
                    let delta = self.get_delta_time(start_time);
                    
                    let new_inst = start_time + std::time::Duration::from_nanos(delta as u64);
                    *control_flow = ControlFlow::WaitUntil(new_inst); // Ожидание в наносекундах.
                }
            }
        });
    }

    pub fn set_world(&mut self, world: Option<&'static mut World>) {
        self.world = world
    }

    pub fn get_world(&self) -> &Option<&'static mut World> {
        &self.world
    }
}
