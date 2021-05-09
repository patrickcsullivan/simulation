use std::time::{Duration, Instant};

// A counter that tracks the index and timing of a fixed length frame.
#[derive(Clone, Copy, Debug)]
pub struct Frame {
    /// Ideal duration of a fixed length frame in milliseconds.
    pub ideal_duration: Duration,

    /// The index of the frame.
    pub index: u64,

    /// The time at which the frame started.
    pub start_time: Instant,

    /// The ideal time at which the frame should have started. This is used to
    /// make sure sure that the start time of future frames don't cummulatively
    /// drift from ideal start times.
    pub ideal_start_time: Instant,
}

impl Frame {
    pub fn new(ideal_duration: Duration, start_time: Instant) -> Frame {
        Frame {
            ideal_duration,
            index: 0,
            start_time,
            ideal_start_time: start_time,
        }
    }

    pub fn next(&self, start_time: Instant) -> Frame {
        let elapsed_frames_count = ((start_time - self.ideal_start_time).as_millis()
            / self.ideal_duration.as_millis()) as u32;
        Frame {
            ideal_duration: self.ideal_duration,
            index: self.index + elapsed_frames_count as u64,
            start_time,
            ideal_start_time: self.ideal_start_time
                + self
                    .ideal_duration
                    .checked_mul(elapsed_frames_count)
                    .unwrap(),
        }
    }
}
