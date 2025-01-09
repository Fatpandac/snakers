use std::{thread, time::{Duration, SystemTime}};

#[derive(Debug)]
pub struct Fps {
    fps: u8,
    last_time: u64,
}

impl Fps {
    pub fn new(set_fps: u8) -> Self {
        Self {
            fps: set_fps,
            last_time: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn do_sleep_time(&self) {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let sleep_time = 1000 / self.fps as u64 - (now - self.last_time);
        if sleep_time > 0 {
            thread::sleep(Duration::from_millis(sleep_time));
        }
    }
}
