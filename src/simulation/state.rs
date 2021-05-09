use super::{component, frame::Frame, system};
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
        world.register::<component::Position>();
        world.register::<component::Velocity>();

        let _ = world
            .create_entity()
            .with(super::component::Position { x: 4.0, y: 7.0 })
            .with(super::component::Velocity { x: 0.1, y: 0.2 })
            .build();

        // Set up dispatcher and systems.
        let mut dispatcher = DispatcherBuilder::new()
            .with(system::SayHello, "say_hello", &[])
            .with(system::UpdatePos, "update_pos", &["say_hello"])
            .with(system::SayHello, "hello_updated", &["update_pos"])
            .build();
        dispatcher.setup(&mut world);

        State {
            world,
            dispatcher,
            frame: None,
        }
    }
}
