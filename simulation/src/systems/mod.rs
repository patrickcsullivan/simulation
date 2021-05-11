///! ECS systems for the simulation.
pub mod continuum_crowds;

use crate::component::{Position, Velocity};
use crate::resources::DurationSinceLastFrame;
use specs::{Read, ReadExpect, ReadStorage, System, WriteStorage};

pub struct SayHello;

impl<'a> System<'a> for SayHello {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, position: Self::SystemData) {
        use specs::Join;

        for position in position.join() {
            println!("Hello, {:?}", &position);
        }
    }
}

pub struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (
        Read<'a, DurationSinceLastFrame>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (delta_t, vel, mut pos) = data;
        let delta_t = delta_t.0;
        use specs::Join;
        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x += vel.x * delta_t.as_secs_f32();
            pos.y += vel.y * delta_t.as_secs_f32();
        }
    }
}
