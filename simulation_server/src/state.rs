use simulation::{
    component::{Position, Velocity},
    frame::Frame,
    systems,
};
use specs::prelude::*;

pub struct State<'a, 'b> {
    pub world: World,
    pub dispatcher: Dispatcher<'a, 'b>,
    pub frame: Option<Frame>,
}

impl State<'_, '_> {
    pub fn new() -> Self {
        // Register components.
        let mut world = World::new();
        world.register::<Position>();
        world.register::<Velocity>();

        let _ = world
            .create_entity()
            .with(Position { x: 4.0, y: 7.0 })
            .with(Velocity { x: 0.1, y: 0.2 })
            .build();

        // Set up dispatcher and systems.
        let mut dispatcher = DispatcherBuilder::new()
            .with(systems::SayHello, "say_hello", &[])
            .with(systems::UpdatePos, "update_pos", &["say_hello"])
            .with(systems::SayHello, "hello_updated", &["update_pos"])
            .build();
        dispatcher.setup(&mut world);

        State {
            world,
            dispatcher,
            frame: None,
        }
    }
}
