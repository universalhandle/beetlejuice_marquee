use core::cmp::Ordering;
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
        for (index, led) in leds.iter_mut().enumerate() {
            *led = match index.cmp(&self.head_index) {
                Ordering::Greater => RGB8::default(), // off
                Ordering::Equal => self.color,
                Ordering::Less => {
                    let segments_behind_head = self.head_index - index;

                    if segments_behind_head > self.tail_length {
                        RGB8::default()
                    } else {
                        self.dim(segments_behind_head, self.tail_length)
                    }
                }
            };
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
