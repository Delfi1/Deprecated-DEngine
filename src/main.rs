mod engine;
use engine::{Engine, Cuboid, World, Vec3, Object, Teapot, radians};

fn main() {
    let engine = Engine::new();

    engine.camera.set_fov(80.0);
    engine.camera.position = Vec3::new(2.0, 1.0, 1.0);
    engine.camera.direction = Vec3::new(radians(10.0), radians(-200.0), radians(180.0));

    let main_world = World::new("Test World");

    let x = 10;
    for i in 1..x+1 {
        let teapot = Teapot::new(Box::leak(format!("Teapot{}", i).into_boxed_str()));
        teapot.position = Vec3::new((i * 150) as f32, 0.0, 0.0);
        main_world.add_object(teapot);
    }

    engine.set_world(Some(main_world));
    engine.run();
}