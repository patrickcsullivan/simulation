mod component;
mod frame;
mod resource;
mod state;
mod system;

use component::{Position, Velocity};
use frame::Frame;
use resource::ElapsedFramesCount;
use specs::{Builder, DispatcherBuilder, World, WorldExt};
use state::State;
use std::time::{Duration, Instant};
use system::{SayHello, UpdatePos};
use tokio::time::sleep;

const FRAME_DURATION: Duration = Duration::from_millis(32u64);

// pub fn run() -> Result<(), String> {
//     let mut world = World::new();
//     world.register::<Position>();
//     world.register::<Velocity>();

//     let _ = world
//         .create_entity()
//         .with(Position { x: 4.0, y: 7.0 })
//         .with(Velocity { x: 0.1, y: 0.2 })
//         .build();

//     let mut dispatcher = DispatcherBuilder::new()
//         .with(SayHello, "say_hello", &[])
//         .with(UpdatePos, "update_pos", &["say_hello"])
//         .with(SayHello, "hello_updated", &["update_pos"])
//         .build();

//     dispatcher.dispatch(&mut world);
//     world.maintain();

//     Ok(())
// }

async fn step(state: &mut State<'_, '_>) -> Result<(), String> {
    if let Some(frame) = state.frame {
        let duration_since_ideal_start = Instant::now() - frame.ideal_start_time;
        if duration_since_ideal_start < frame.ideal_duration {
            // Wait until it's time for the next frame to start.
            sleep(frame.ideal_duration - duration_since_ideal_start).await;
        }
        let next_frame = frame.next(Instant::now());
        state.frame = Some(next_frame);
        state
            .world
            .insert(ElapsedFramesCount(next_frame.index - frame.index))
    } else {
        state.frame = Some(Frame::new(FRAME_DURATION, Instant::now()));
        state.world.insert(ElapsedFramesCount(0));
    }

    // Executate a frame of the simulation.
    state.dispatcher.dispatch(&state.world);
    state.world.maintain();

    Ok(())
}

pub async fn run() -> Result<(), String> {
    let mut state = State::new();
    let sim_loop = async { while let Ok(()) = step(&mut state).await {} };
    sim_loop.await;
    Ok(())
}
