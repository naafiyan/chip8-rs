pub struct EmuTimer {
    time_left: usize,
}

impl EmuTimer {
    pub fn new(time_left: usize) -> EmuTimer {
        EmuTimer { time_left }
    }

    pub fn get_time_left(&self) -> usize {
        self.time_left
    }

    pub fn set_time_left(&mut self, time_left: usize) {
        self.time_left = time_left;
    }

    pub fn decr_time_left(&mut self) {
        if self.time_left > 0 {
            self.time_left -= 1
        }
    }

    pub fn incr_time_left(&mut self) {
        self.time_left += 1
    }
}
