use std::time::{Instant};

pub struct Timer {
    last_time: Instant,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            last_time: Instant::now(),
        }
    }

    pub fn get_delta_time(&mut self) -> f32 {
        let now = Instant::now();
        let delta = now.duration_since(self.last_time);
        self.last_time = now;

        delta.as_secs_f32()
    }

    pub fn elapsed_time(&self) -> f32 {
        self.last_time.elapsed().as_secs_f32()
    }
}
