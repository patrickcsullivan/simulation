///! ECS resources for the simulation.

/// The number of frames that have passed since the last simulation.
#[derive(Clone, Copy, Debug, Default)]
pub struct ElapsedFramesCount(pub u64);
