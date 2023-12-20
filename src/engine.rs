use glium::glutin::surface::WindowSurface;
use glium::index::NoIndices;
// Open GL Wrapper
use glium::{Frame, Surface, program};
use glium::{
    backend::glutin::SimpleWindowBuilder,
    uniforms::AsUniformValue
};

extern crate typetag;
use serde::{Serialize, Deserialize};

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

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::f32::consts::PI;
use std::time::Instant;

#[path ="../src/input.rs"]
mod input;
use self::input::Key;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: (f32, f32, f32)
}

glium::implement_vertex!(Vertex, position);

#[derive(Clone, Copy, Default, Serialize, Deserialize)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {x, y, z}
    }

    pub fn get(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    pub fn from(&mut self, vector: [f32; 3]) -> Self {
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

    pub fn get_perspective(&self, frame: &Frame) -> [[f32; 4]; 4] {
        let aspect_ratio = frame.get_dimensions().0 as f32 / frame.get_dimensions().1 as f32;

        let fov = self.fov;
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        // note: remember that this is column-major, so the lines of code are actually columns
        [
            [  f / aspect_ratio   ,   0.0,               0.0              ,   0.0],
            [         0.0         ,     f ,              0.0              ,   0.0],
            [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
            [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
        ]
    }
}

// World 
#[derive(Default)]
pub struct World {
    pub name: &'static str,
    obejcts: Vec<&'static dyn Object>,

    ambient_color: (f32, f32, f32, f32)
}

impl World {
    pub fn new(name: &'static str) -> &'static mut Self {
        Box::leak(Box::new(Self {name, ..Default::default()}))
    }

    fn draw_axis(&self, frame: &mut Frame, camera: &Camera, display: &glium::Display<WindowSurface>) {
        let perspective = camera.get_perspective(frame);
        
        let vertex_buffer = glium::VertexBuffer::new(display, &[
            Vertex {position: (0.0, 0.0, 0.0)},
            Vertex {position: (0.0, 0.0, 0.0)}
        ]).unwrap();

        //

        //frame.draw(vertex_buffer, NoIndices, , );
    }

    // Draw all objects
    fn draw_objects(&self, camera: &Camera, frame: &mut Frame) {
        for obj in &self.obejcts {
            obj.draw(camera, frame);
        }
    }


    // Adding Objects 
    fn add_object(&mut self, object: &'static dyn Object) {
        self.obejcts.push(object)
    }

    pub fn get_objects(&mut self) -> &Vec<&'static dyn Object> {
        &self.obejcts
    }

    pub fn add_cube(&mut self, name: &'static str, position: Vec3, rotation: Vec3, size: Vec3) {
        let cube = Cube::new(name);
        cube.position = position;
        cube.rotation = rotation;
        cube.size = size;

        self.add_object(cube)
    }

    // Clear Screen
    fn clear(&self, frame: &mut Frame) {
        let color = self.ambient_color;
        frame.clear_color(color.0, color.1, color.2, color.3)
    }
}

// Objects

pub trait Object {
    fn new(name: &'static str) -> &'static mut Self where Self: Sized;

    fn get_id(&self) -> usize;

    fn draw(&self, camera: &Camera, frame: &mut Frame);
}

#[derive(Default)]
pub struct Cube {
    id: usize,
    name: &'static str,

    pub position: Vec3,
    pub rotation: Vec3,
    pub size: Vec3
}

const CUBE_VERT: [Vertex; 1] = [
    Vertex {position: (0.0, 0.0, 0.0)},
    
];

impl Object for Cube {
    fn new(name: &'static str) -> &'static mut Self where Self: Sized {
        Box::leak(Box::new(Self {name, ..Default::default()}))
    }

    fn get_id(&self) -> usize {
        self.id
    }

    fn draw(&self, camera: &Camera, frame: &mut Frame) {
        //print!("Drawing {} ", self.name)
        let perspective = camera.get_perspective(frame);

    }
}

// Engine Settings
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

                    // Clear screen
                    if self.world.is_some() {
                        self.world.as_mut().unwrap().clear(&mut frame);

                        // Draw Axis
                        self.world.as_mut().unwrap().draw_axis(&mut frame, &self.camera, &display);
                    } else {
                        frame.clear_color(0.0, 0.0, 0.0, 1.0);
                    }

                    // GUI?

                    if self.world.is_some() {
                        let start_drawing = Instant:: now();
                        self.world.as_mut().unwrap().draw_objects(&self.camera, &mut frame);

                        let draw_time = Instant::now().duration_since(start_drawing).as_secs_f64();
                        println!("World drawing time: {}", draw_time)
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
