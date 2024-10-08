use super::running_lights::RunningLights;
use smart_leds::RGB8;

pub struct Arrow {
    left: RunningLights,
    right: RunningLights,
}

impl Arrow {
    pub fn new(color: &RGB8, tail_length: usize) -> Self {
        let run_in_reverse = true;
        Self {
            left: RunningLights::new(color, !run_in_reverse, tail_length),
            right: RunningLights::new(color, run_in_reverse, tail_length),
        }
    }

    pub fn mutate(&mut self, leds: &mut [RGB8]) {
        if leds.len() < 4 {
            panic!("Arrow effect requires at least 3 LEDs");
        }

        let (leds_arrow_left, leds_unalloc) = leds.split_at_mut(leds.len() / 2);

        // in the case of an uneven number of LEDs, the point cannot be handled
        // in a RunningLights and must be managed manually
        let right_index = if leds_unalloc.len() > leds_arrow_left.len() {
            // Getting the color at the terminus of one of the arms *before* they've been mutated means we don't have
            // to transform the color in Arrow, e.g., if the head is currently at the terminus (the brightest state
            // for that pixel), then on the next tick the arrow point will be at max brightness and the termini will
            // be at their next-to-brightest state. Note that it doesn't matter whether we get the color from the left
            // or right arm, as they are equally sized and running at the same speed; they will always match.
            leds_unalloc[0] = self.left.color_at_terminus();
            1
        } else {
            0
        };

        self.left.mutate(leds_arrow_left);
        self.right.mutate(&mut leds_unalloc[right_index..]);
    }
}
