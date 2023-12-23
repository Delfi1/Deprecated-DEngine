mod engine;
use engine::{Engine, Cuboid, World, Vec3, Object, Teapot, radians};

fn main() {
    let engine = Engine::new();

    engine.camera.set_fov(80.0);
    engine.camera.position = Vec3::new(2.0, 1.0, 1.0);
    engine.camera.direction = Vec3::new(radians(10.0), radians(-200.0), radians(180.0));

    let main_world = World::new("Test World");

    let a = 10;
    for x in 1..a+1 {
        for z in 1..a+1 {
            let teapot = Teapot::new(Box::leak(format!("Teapot{}", x+x*z).into_boxed_str()));
            teapot.position = Vec3::new((x * 150) as f32, 0.0, (z * 100) as f32);
            main_world.add_object(teapot);
        }
    }

    engine.set_world(Some(main_world));
    engine.run();
}