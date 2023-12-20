mod engine;
use engine::{Engine, World, Vec3};

fn main() {
    let engine = Engine::new();

    let main_world = World::new("Test World");
    
    let x = 10000;

    for i in 1..x+1 {
        main_world.add_cube( Box::leak(format!("Cube{}", i).into_boxed_str()), Vec3::new(i as f32, 0.0, 0.0), Vec3::default(), Vec3::default());
    }

    engine.set_world(Some(main_world));
    engine.run();
}