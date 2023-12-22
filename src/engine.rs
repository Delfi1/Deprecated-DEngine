use glium::DrawParameters;
// Open GL Wrapper
use glium::glutin::surface::WindowSurface;
use glium::index::NoIndices;
use glium::{Frame, Surface, Display, backend::glutin::SimpleWindowBuilder, uniform};

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
use input::Key;

#[path ="../src/teapot.rs"]
mod teapot;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: (f32, f32, f32),
    normal: (f32, f32, f32)
}

glium::implement_vertex!(Vertex, position, normal);

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

    pub fn get_tuple(&self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }

    pub fn get_matrix(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    pub fn from(vector: [f32; 3]) -> Self {
        Self {x: vector[0], y: vector[1], z: vector[2]}
    }
}

pub struct Camera {
    pub position: Vec3,
    pub direction: Vec3,

    fov: f32
}

pub fn radians(x: f32) -> f32 {
    x * PI / 180.0
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
        self.fov = radians(degrees);
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

        [
            [  f / aspect_ratio   ,   0.0,               0.0              ,   0.0],
            [         0.0         ,     f ,              0.0              ,   0.0],
            [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
            [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
        ]
    }

    pub fn get_view(&self) -> [[f32; 4]; 4] {
        let f = {
            let d = self.direction.get_tuple();
            let len = d.0 * d.0 + d.1 * d.1 + d.2 * d.2;
            let len = len.sqrt();
            (d.0 / len, d.1 / len, d.2 / len)
        };

        let up = (0.0, 1.0, 0.0);

        let s = (f.1 * up.2 - f.2 * up.1,
                 f.2 * up.0 - f.0 * up.2,
                 f.0 * up.1 - f.1 * up.0);

        let s_norm = {
            let len = s.0 * s.0 + s.1 * s.1 + s.2 * s.2;
            let len = len.sqrt();
            (s.0 / len, s.1 / len, s.2 / len)
        };

        let u = (s_norm.1 * f.2 - s_norm.2 * f.1,
                 s_norm.2 * f.0 - s_norm.0 * f.2,
                 s_norm.0 * f.1 - s_norm.1 * f.0);

        let position = self.position.get_tuple();
        
        let p = (-position.0 * s.0 - position.1 * s.1 - position.2 * s.2,
                 -position.0 * u.0 - position.1 * u.1 - position.2 * u.2,
                 -position.0 * f.0 - position.1 * f.1 - position.2 * f.2);

        [
            [s_norm.0, u.0, f.0, 0.0],
            [s_norm.1, u.1, f.1, 0.0],
            [s_norm.2, u.2, f.2, 0.0],
            [p.0, p.1,  p.2, 1.0],
        ]
    }
}

// World 
#[derive(Default)]
pub struct World {
    pub name: &'static str,
    objects: Vec<&'static mut dyn Object>,
    global_light: Vec3,

    ambient_color: (f32, f32, f32, f32)
}

impl World {
    pub fn new(name: &'static str) -> &'static mut Self {
        let global_light = Vec3::new(-1.0, 0.4, 0.9);
        Box::leak(Box::new(Self {name, global_light, ..Default::default()}))
    }

    fn draw_axis(&self, frame: &mut Frame, camera: &Camera, display: &glium::Display<WindowSurface>) {
        let perspective = camera.get_perspective(frame);

    }

    fn compile_shaders(&mut self, display: &glium::Display<WindowSurface>) {
        for obj in self.objects.as_mut_slice() {
            if obj.get_program().is_none() {
                obj.compile_shader(display);
            }
        }
    }

    // Draw all objects
    fn draw_objects(&mut self, camera: &Camera, frame: &mut Frame, draw_parameters: &DrawParameters<'_>, display: &Display<WindowSurface>) {
        for obj in &self.objects {
            obj.draw(&self, camera, frame, draw_parameters, display);
        }
    }

    // Adding Objects 
    pub fn add_object(&mut self, object: &'static mut dyn Object) {
        self.objects.push(object)
    }

    pub fn get_objects(&mut self) -> &Vec<&'static mut dyn Object> {
        &self.objects
    }

    // Clear Screen
    fn clear(&self, frame: &mut Frame) {
        let color = self.ambient_color;
        frame.clear_color_and_depth((color.0, color.1, color.2, color.3), 1.0)
    }
}

// Objects
pub trait Object {
    fn new(name: &'static str) -> &'static mut Self where Self: Sized;

    fn get_program(&self) -> &Option<glium::Program>;
    fn compile_shader(&mut self, display: &Display<WindowSurface>);

    fn draw(&self, parent_world: &World, camera: &Camera, frame: &mut Frame, draw_parameters: &DrawParameters<'_>, display: &Display<WindowSurface>);
}

const TEST_VS: &str = r#"
    #version 150

    in vec3 position;
    in vec3 normal;

    out vec3 v_normal;

    uniform mat4 perspective;
    uniform mat4 view;
    uniform mat4 model;
    uniform vec3 global_position;

    void main() {
        mat4 modelview = view * model;
        
        vec3 render_position = position + global_position;
        v_normal = transpose(inverse(mat3(modelview))) * normal;
        gl_Position = perspective * modelview * vec4(render_position, 1.0);
    }
"#;

const TEST_FS: &str = r#"
    #version 150

    in vec3 v_normal;
    out vec4 color;
    uniform vec3 light;

    void main() {
        float brightness = dot(normalize(v_normal), normalize(light));
        vec3 dark_color = vec3(0.6, 0.0, 0.0);
        vec3 regular_color = vec3(1.0, 0.0, 0.0);
        color = vec4(mix(dark_color, regular_color, brightness), 1.0);
    }
"#;

#[derive(Default)]
pub struct Cuboid {
    name: &'static str,
    program: Option<glium::Program>,

    pub position: Vec3,
    pub rotation: Vec3,
    pub size: Vec3
}

impl Object for Cuboid {
    fn new(name: &'static str) -> &'static mut Self where Self: Sized {
        Box::leak(Box::new(Self {name, ..Default::default()}))
    }

    fn compile_shader(&mut self, display: &Display<WindowSurface>) {
        self.program = Some(glium::Program::from_source(display, TEST_VS, TEST_FS, None).unwrap())
    }

    fn get_program(&self) -> &Option<glium::Program> {
        &self.program
    }

    fn draw(&self, parent_world: &World, camera: &Camera, frame: &mut Frame, draw_parameters: &DrawParameters<'_>, display: &Display<WindowSurface>) {
        //print!("Drawing {} ", self.name)
        let perspective = camera.get_perspective(frame);
        let view = camera.get_view();

        let model = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ];

        if self.program.is_some() {
            //frame.draw(_, _, &self.program.as_ref().unwrap(), uniforms, draw_parameters)
        };
    }
}

#[derive(Default)]
pub struct Teapot {
    name: &'static str,
    program: Option<glium::Program>,

    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3
}

impl Object for Teapot {
    fn new(name: &'static str) -> &'static mut Self where Self: Sized {
        let scale = Vec3::new(1.0, 1.0, 1.0);
        Box::leak(Box::new(Self {name, scale, ..Default::default()}))
    }

    fn get_program(&self) -> &Option<glium::Program> {
        &self.program
    }

    fn compile_shader(&mut self, display: &Display<WindowSurface>) {
        self.program = Some(glium::Program::from_source(display, TEST_VS, TEST_FS, None).unwrap())
    }

    fn draw(&self, parent_world: &World, camera: &Camera, frame: &mut Frame, draw_parameters: &DrawParameters<'_>, display: &Display<WindowSurface>) {
        //println!("self pos {:?}", self.position.get_tuple());
        let perspective = camera.get_perspective(frame);
        let view = camera.get_view();

        let positions = glium::VertexBuffer::new(display, &teapot::VERTICES).unwrap();
        let normals = glium::VertexBuffer::new(display, &teapot::NORMALS).unwrap();
        let indices = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList,
                                            &teapot::INDICES).unwrap();

        let model = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 2.0, 1.0f32]
        ];

        if self.program.is_some() {
            //println!("Drawing {} ", self.name);
            frame.draw((&positions, &normals), &indices, &self.program.as_ref().unwrap(),
            &uniform! { model: model, global_position: self.position.get_matrix(), view: view, perspective: perspective, light: parent_world.global_light.get_matrix() },
            draw_parameters).unwrap();
        };
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

        Self {title: "DEngine", window_size, min_window_size, max_fps: 1200 }
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

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            //backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };

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

                    self.camera.direction.x += radians(1.0);

                    // Clear screen
                    if self.world.is_some() {
                        self.world.as_mut().unwrap().clear(&mut frame);

                        // Draw Axis
                        self.world.as_mut().unwrap().draw_axis(&mut frame, &self.camera, &display);
                    } else {
                        frame.clear_color(0.0, 0.0, 0.0, 1.0);
                    }

                    // GUI?

                    // Draw world objects
                    if self.world.is_some() {
                        let start_drawing = Instant:: now();
                        self.world.as_mut().unwrap().compile_shaders(&display);
                        self.world.as_mut().unwrap().draw_objects(&self.camera, &mut frame, &params, &display);

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
