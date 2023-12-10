use glium::{Frame, glutin::surface::WindowSurface, Display, Surface};

#[derive(Clone, Copy, Default)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32
}

pub struct Scene {
    objects: Vec<Box<dyn Object3D>>
}

impl Scene {
    pub fn render_scene(&mut self, display: &Display<WindowSurface>) {
        let mut frame = display.draw();
        frame.clear_color(22.0 / 255.0, 22.0 / 255.0, 29.0 / 255.0, 1.0);
        
        for object in &self.objects {
            object.render(self, &mut frame);
        }

        frame.finish().unwrap();
    }

    fn add_object(&mut self, object: Box<dyn Object3D>) {
        self.objects.push(object);
    }
}

impl Default for Scene {
    fn default() -> Self {
        Scene { objects: Vec::new() }
    }
}

trait Object3D {
    //fn new(parent_scene: Scene) -> Self where Self: Sized;
    fn render(&self, _parent_scene: &Scene, _frame: &mut Frame);
}

pub struct Cube {
    id: u32,

    position: Vec3,
    rotation: Vec3,
    size: f32,
}

impl Cube {
    fn new(parent_scene: Scene, position: Vec3, rotation: Vec3, size: f32) -> Self {
        let id = parent_scene.objects.len() as u32;
        Self { id, position, rotation, size }
    }
}

impl Object3D for Cube {
    fn render(&self, _parent_scene: &Scene, _frame: &mut Frame) {
        //
    }
}