use smart_leds::RGB8;

// We are making a running pattern through the strip, whereby one LED (the head)
// shines bright, dragging a tail of diminishing brightness behind it
pub struct RunningLights {
    color: RGB8,
    // If true, the pattern will not repeat until the tail has completely exited; i.e.,
    // if true, the last frame of the animation will be comprised of all LEDs turned off.
    // If false, the head will appear to chase the tail.
    tail_must_exit_before_restart: bool,
    run_in_reverse: bool,
    color_at_terminus: RGB8,
    // can potentially be outside the range of LEDs assigned the effect, if tail_must_exit_before_restart == true
    head_position: usize,
    tail_length: usize,
}

impl RunningLights {
    pub fn new(
        color: &RGB8,
        tail_must_exit_before_restart: bool,
        run_in_reverse: bool,
        tail_length: usize,
    ) -> Self {
        Self {
            color: *color,
            color_at_terminus: RGB8::default(),
            tail_must_exit_before_restart,
            head_position: 0,
            run_in_reverse,
            tail_length,
        }
    }

    pub fn color_at_terminus(&self) -> RGB8 {
        self.color_at_terminus
    }

    pub fn mutate(&mut self, leds: &mut [RGB8]) {
        if !self.tail_must_exit_before_restart && leds.len() <= self.tail_length {
            panic!("Number of LEDs too short to support configured tail length.");
        }

        // default everything off; RGB values will be assigned based on position relative to the head
        for led in leds.iter_mut() {
            *led = RGB8::default();
        }

        if let Some(_) = leds.get(self.head_position) {
            leds[self.head_position] = self.color;
        }

        for index_in_effect in 1..=self.tail_length {
            let index_in_strip = match leds.get(self.head_position - index_in_effect) {
                Some(_) => Some(self.head_position - index_in_effect),
                None => {
                    if self.tail_must_exit_before_restart == true {
                        None
                    } else {
                        Some(leds.len() + self.head_position - index_in_effect)
                    }
                }
            };

            if let Some(i) = index_in_strip {
                leds[i] = self.dim(index_in_effect, self.tail_length);
            }
        }

        self.color_at_terminus = leds[leds.len() - 1];

        if self.run_in_reverse {
            leds.reverse();
        }

        // the number of frames in the animation; can be longer than the number of LEDs depending on
        // self.tail_must_exit_before_restart and self.tail_length
        let frame_cnt = if self.tail_must_exit_before_restart {
            leds.len() + self.tail_length
        } else {
            leds.len()
        };

        // advance the head
        if self.head_position < frame_cnt - 1 {
            self.head_position += 1;
        } else {
            self.head_position = 0;
        }
    }

    fn dim(&self, segments_behind_head: usize, cnt_tail_pixels: usize) -> RGB8 {
        // add one because only the head should be at 100% brightness
        let total_pixels = u8::try_from(cnt_tail_pixels + 1)
            .and_then(|n| Ok(n))
            .unwrap();
        let brightness_factor = total_pixels
            - u8::try_from(segments_behind_head)
                .and_then(|n| Ok(n))
                .unwrap();

        RGB8 {
            r: (self.color.r / total_pixels).saturating_mul(brightness_factor),
            g: (self.color.g / total_pixels).saturating_mul(brightness_factor),
            b: (self.color.b / total_pixels).saturating_mul(brightness_factor),
        }
    }
}
