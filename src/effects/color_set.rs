use rand::{seq::SliceRandom, Rng};
use smart_leds::RGB8;

pub struct ColorSet<'a, R: Rng> {
    colors: &'a [RGB8],
    // random number generator
    rng: &'a mut R,
}

impl<'a, R: Rng> ColorSet<'a, R> {
    pub fn new(colors: &'a [RGB8], rng: &'a mut R) -> Self {
        Self { colors, rng }
    }

    pub fn mutate(&mut self, leds: &mut [RGB8]) {
        for led in leds.iter_mut() {
            *led = *self.colors.choose(&mut self.rng).unwrap();
        }
    }
}
