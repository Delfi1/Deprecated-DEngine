#[path = "../src/objects.rs"]
mod objects;

use objects::Object3D;

#[derive(Default)]
struct Scene {
    objects: Vec<dyn Object3D>
}

impl Scene {
    pub fn new() -> Self {
        return {..Default::default()}
    }

    pub fn render(&self) {
        self.objects.iter().render();
    }
}