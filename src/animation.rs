// TODO: what would it look like for this struct to manage the loop internally?
pub struct Animation {
    frame_cnt: usize,
    frames_displayed: usize,
}

impl Animation {
    pub fn new(frame_cnt: usize) -> Self {
        Self {
            frame_cnt,
            frames_displayed: 0,
        }
    }

    pub fn frames_displayed(&self) -> usize {
        self.frames_displayed
    }

    // this is meant to be used at the end of a loop;
    // if loops were managed internally this would become private
    pub fn next(&mut self) -> &Self {
        if self.frames_displayed < self.frame_cnt - 1 {
            self.frames_displayed += 1;
        } else {
            self.frames_displayed = 0;
        }

        self
    }
}
