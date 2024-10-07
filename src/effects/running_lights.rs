use smart_leds::RGB8;

// We are making a running pattern through the strip, whereby one LED (the head)
// shines bright, dragging a tail of diminishing brightness behind it
pub struct RunningLights {
    color: RGB8,
    head_index: usize,
    tail_length: usize,
}

impl<'a> RunningLights {
    pub fn new(color: &RGB8, tail_length: usize) -> Self {
        Self {
            color: *color,
            head_index: 0,
            tail_length,
        }
    }

    pub fn mutate(&mut self, leds: &mut [RGB8]) {
        if leds.len() <= self.tail_length {
            panic!("Number of LEDs too short to support configured tail length.");
        }

        // default everything off; RGB values will be assigned based on position relative to the head
        for led in leds.iter_mut() {
            *led = RGB8::default();
        }

        leds[self.head_index] = self.color;

        for index_in_effect in 1..=self.tail_length {
            let overall_index = match leds.get(self.head_index - index_in_effect) {
                Some(_) => self.head_index - index_in_effect,
                None => leds.len() + self.head_index - index_in_effect,
            };

            leds[overall_index] = self.dim(index_in_effect, self.tail_length);
        }

        // move the head one pixel up the strip
        if self.head_index < leds.len() - 1 {
            self.head_index += 1;
        } else {
            self.head_index = 0;
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
            r: self.color.r / total_pixels * brightness_factor,
            g: self.color.g / total_pixels * brightness_factor,
            b: self.color.b / total_pixels * brightness_factor,
        }
    }
}
