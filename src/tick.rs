pub struct Tick {
    ticks_per_sec: u16,
    elapsed: u16,
}

impl Tick {
    pub fn new(ticks_per_sec: u16) -> Self {
        Self {
            ticks_per_sec,
            elapsed: 0,
        }
    }

    pub fn len(&self) -> u16 {
        1_000 / self.ticks_per_sec
    }

    pub fn elapsed(&self) -> u16 {
        self.elapsed
    }

    // this is meant to be used at the end of a loop
    pub fn tock(&mut self) -> &Self {
        if self.elapsed < self.ticks_per_sec - 1 {
            self.elapsed += 1;
        } else {
            self.elapsed = 0;
        }

        self
    }
}
