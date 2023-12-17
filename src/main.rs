mod engine;
use engine::{Engine, World};

fn main() {
    let engine = Engine::new();

    let main_world = World::new();

    engine.set_world(Some(main_world));

    engine.run();
}