use std::time::Instant;

pub struct FPSCounter {
    last_update: Instant,
    frame_count: u32,
}

impl FPSCounter {
    pub fn new() -> Self {
        Self {
            frame_count: 0,
            last_update: Instant::now(),
        }
    }

    pub fn update(&mut self) -> Option<f64> {
        let now = Instant::now();

        self.frame_count += 1;

        let elapsed = (now - self.last_update).as_secs_f64();
        if elapsed >= 1.0 {
            let mean_frame_duration = elapsed / (self.frame_count as f64);
            self.frame_count = 0;
            self.last_update = now;

            Some(mean_frame_duration)
        } else {
            None
        }
    }
}
