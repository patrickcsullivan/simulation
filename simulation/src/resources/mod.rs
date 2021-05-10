///! ECS resources for the simulation.
pub mod continuum_crowds;

/// The number of frames that have passed since the last simulation.
#[derive(Clone, Copy, Debug, Default)]
pub struct ElapsedFramesCount(pub u64);
