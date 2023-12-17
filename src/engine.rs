use glium::Frame;
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

    fn render(&self);
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
    pub fn load(&mut self) {
        //self.object = ...
    }

    pub fn save(&self) {
        let data = json!(self.objects);
        // Self. objects to json
    }
}

pub struct Settings {
    window_size: PhysicalSize<usize>,
    min_window_size: PhysicalSize<usize>,

    max_fps: usize
}

impl Settings {
    pub fn new(window_size: PhysicalSize<usize>, min_window_size: PhysicalSize<usize>, max_fps: usize) -> Self {
        Self {window_size, min_window_size, max_fps}
    }
}

impl Default for Settings {
    fn default() -> Self {
        let window_size = PhysicalSize::new(700, 500);
        let min_window_size = PhysicalSize::new(350, 250);

        Self { window_size, min_window_size, max_fps: 120 }
    }
}

pub struct Engine {
    pub camera: Camera,
    world: Option<World>,
    pub settings: Settings
}

impl Engine {
    pub fn new() -> &'static mut Self {
        let camera = Camera::new();
        let world = None;

        let settings = Settings::default();

        Box::leak(Box::new(Self {camera, world, settings}))
    }

    fn get_delta_time(&self, start_time: Instant) -> usize {
        let elapsed_time = Instant::now().duration_since(start_time).as_nanos() as usize;

        match 1_000_000_000 / self.settings.max_fps >= elapsed_time {
            true => 1_000_000_000 / self.settings.max_fps - elapsed_time,
            false => 0
        }
    }

    fn get_fps(delta: usize) -> usize {
        match delta > 0 {
            true => 1_000_000_000 / delta,
            false => 0
        }
    }

    pub fn run(&'static mut self) {
        let event_loop = EventLoop::new();

        let (window, display) = SimpleWindowBuilder::new().build(&event_loop);

        event_loop.run(move |event, _, control_flow| {
            control_flow.set_wait();
            control_flow.set_poll();
            let start_time = Instant::now();

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => control_flow.set_exit(),
                _ => (),
            }

            match *control_flow {
                ControlFlow::Exit => (),
                _ => {
                    window.request_redraw(); // Запрос на отрисовку.
                    
                    let wait_nanos = self.get_delta_time(start_time) as u64;
                    println!("{}", wait_nanos);
                    let new_inst = start_time + std::time::Duration::from_nanos(wait_nanos);
                    *control_flow = ControlFlow::WaitUntil(new_inst); // Ожидание в наносекундах.
                }
            }
        });
    }

    pub fn set_world(&mut self, world: Option<World>) {
        self.world = world
    }

    pub fn get_world(&self) -> &Option<World> {
        &self.world
    }
}
