///! ECS resources for the simulation.
pub mod continuum_crowds;

use std::time::Duration;

/// The number of frames that have passed since the last simulation.
#[derive(Clone, Copy, Debug, Default)]
pub struct ElapsedFramesCount(pub u64);

/// The amount of time that has passed since the last frame of the simulation.
#[derive(Clone, Copy, Debug, Default)]
pub struct DurationSinceLastFrame(pub Duration);
