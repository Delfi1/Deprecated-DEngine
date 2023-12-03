#[path = "../src/teapot.rs"]
mod teapot;

use glium::{Surface, glutin::{display, surface::WindowSurface}, DrawParameters, GlObject};

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
}
implement_vertex!(Vertex, position, normal);

pub struct Scene {
    camera: Camera,
    objects: Vec<Box<dyn Object3D>>
}

impl Scene {
    pub fn new(camera: Camera) -> Self {
        Scene { camera, objects: Vec::new() }
    }

    fn add_object(&mut self, object: Box<dyn Object3D>) {
        self.objects.push(object);
    }

    pub fn get_object(&self, id: i32) -> Result<&Box<dyn Object3D>, String> {
        for object in &self.objects {
            if object.get_id() == id {
                return Ok(&object);
            }
        }
        Err("Object not found".to_string())
    }

    pub fn render(&self) {
        for object in &self.objects {
            object.render(&self);
        }
    }
}

pub trait Object3D {    
    fn get_id(&self) -> i32;

    fn get_position(&self) -> [f32; 3];
    fn set_position(&mut self, position: [f32; 3]);

    fn get_rotation(&self) -> [f32; 3];
    fn set_rotation(&mut self, rotation: [f32; 3]);   

    fn render(&self, parent_scene: &Scene);
}

pub struct Camera {
    position: [f32; 3],
    direction: [f32; 3],
    up: [f32; 3],

    fov: f32,
    zfar: f32,
    znear: f32
}

impl Camera {
    pub fn new(position: [f32; 3], direction: [f32; 3], up: [f32; 3], fov: f32, zfar: f32, znear: f32) -> Self {
        Camera { position, direction, up, fov, zfar, znear }
    }
}

pub struct Cube {
    id: i32,

    position: [f32; 3],
    rotation: [f32; 3],
    size: [f32; 3]
}

impl Cube {
    pub fn new(scene: &mut Scene, position: [f32; 3], rotation: [f32; 3], size: [f32; 3]) {
        let _id = scene.objects.len() as i32;

        let cube = Cube {id: _id, position, rotation, size};

        scene.add_object(Box::new(cube));
    }
}

impl Object3D for Cube {
    fn get_id(&self) -> i32 {
        self.id
    }
    fn get_position(&self) -> [f32; 3] {
        self.position
    }
    fn get_rotation(&self) -> [f32; 3] {
        self.rotation
    }

    fn set_position(&mut self, position: [f32; 3]) {
        self.position = position
    }
    fn set_rotation(&mut self, rotation: [f32; 3]) {
        self.rotation = rotation
    }

    fn render(&self, parent_scene: &Scene) {
        println!("Test");
    }
}