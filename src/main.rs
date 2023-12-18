mod engine;
use engine::{Engine, World, Object, Cube};

fn main() {
    let engine = Engine::new();

    let main_world = World::new("Test World");

    for i in 1..11 {
        let name = Box::leak(format!("Cube{}", i).into_boxed_str());
        let cube = Cube::new(&main_world, name);
        main_world.add_object(cube);
    }

    main_world.save();

    engine.set_world(Some(main_world));

    engine.run();
}