mod channel;
mod error;
mod network;
mod state;

use channel::Senders;
use error::Result;
use futures::future;
use futures::pin_mut;
use simulation::{
    component::{Position, Velocity},
    frame::Frame,
    resource::ElapsedFramesCount,
    system::{SayHello, UpdatePos},
};
use specs::{Builder, DispatcherBuilder, World, WorldExt};
use state::State;
use std::{
    env,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tokio::{net::TcpListener, time::sleep};

const FRAME_DURATION: Duration = Duration::from_millis(32u64);

#[tokio::main]
async fn main() -> Result<()> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let senders = Arc::new(Mutex::new(Senders::new()));
    let listener = TcpListener::bind(&addr).await?;
    let listen = network::listen(listener, senders);
    let run_sim = run();
    pin_mut!(listen, run_sim);
    future::select(listen, run_sim).await;

    Ok(())
}

async fn step(state: &mut State<'_, '_>) -> Result<()> {
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

pub async fn run() -> Result<()> {
    let mut state = State::new();
    let sim_loop = async { while let Ok(()) = step(&mut state).await {} };
    sim_loop.await;
    Ok(())
}
