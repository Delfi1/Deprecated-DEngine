mod engine;
use engine::{Engine, World, Cube, Object};

fn main() {
    let engine = Engine::new();

    let main_world = World::new();

    let cube1 = Cube::new(&main_world, "Cube1");
    main_world.add_object(cube1);

    main_world.save("Test");

    engine.set_world(Some(main_world));

    engine.run();
}