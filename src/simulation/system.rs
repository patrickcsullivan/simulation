use super::component::{Position, Velocity};
use super::resource::ElapsedFramesCount;
use specs::{Read, ReadStorage, System, WriteStorage};

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
        Read<'a, ElapsedFramesCount>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (delta, vel, mut pos) = data;
        let delta = delta.0;
        use specs::Join;
        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x += vel.x * delta as f32;
            pos.y += vel.y * delta as f32;
        }
    }
}
