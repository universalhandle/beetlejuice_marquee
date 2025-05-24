use core::ops::RangeInclusive;

use rand::{rngs::SmallRng, Rng, SeedableRng};
use smart_leds_animations::animate::{AnimateFrames, FrameManager, ManageFrames, Transition};

pub struct Glitch<'a, ColorFormat> {
    colors: &'a [ColorFormat],
    current_color_index: usize,
    frame_manager: FrameManager,
    pixel_range: RangeInclusive<usize>,
    rng: SmallRng,
}

impl<'a, ColorFormat> Glitch<'a, ColorFormat> {
    const BRIGHTNESS_LEVELS_CNT: usize = 10;

    pub fn new(colors: &'a [ColorFormat], pixel_range: RangeInclusive<usize>) -> Self {
        // ensures safe access to colors[0]
        if colors.len() < 1 {
            panic!("At least one color is required");
        }

        Self {
            colors,
            current_color_index: 0,
            frame_manager: FrameManager::new(150),
            pixel_range,
            rng: SmallRng::seed_from_u64(361279846193),
        }
    }

    fn current_color(&self) -> &'a ColorFormat {
        &self.colors[self.current_color_index]
    }

    fn next_color(&mut self) -> &'a ColorFormat {
        self.current_color_index += 1;
        match self.colors.get(self.current_color_index) {
            Some(color) => &color,
            None => {
                self.current_color_index = 0;
                self.current_color()
            }
        }
    }
}

impl<'a, ColorFormat> AnimateFrames for Glitch<'a, ColorFormat>
where
    ColorFormat: Default + Transition,
{
    type ColorFormat = ColorFormat;

    fn frame_manager(&self) -> &FrameManager {
        &self.frame_manager
    }

    fn set_frame_manager(&mut self, frame_manager: FrameManager) -> () {
        self.frame_manager = frame_manager;
    }

    fn pixel_range(&self) -> RangeInclusive<usize> {
        self.pixel_range.clone()
    }

    fn render_frame(&mut self, canvas: &mut [Self::ColorFormat]) -> () {
        let off = ColorFormat::default();

        // ensure consistent color assignment across frames by resetting the index
        self.current_color_index = 0;

        for (i, pixel) in canvas.iter_mut().enumerate() {
            // Assign colors from our palette to the pixels in a seemingly random but idempotent way.
            // Actual random assignment is challenging because the application would need to store
            // both the original value of the pixel as well as its current state (e.g., off, dimmed 25%),
            // doubling the memory requirement. Idempotent assignment ensures that each pixel will be
            // able to return to its original state, as opposed to being lit red one cycle, dark the
            // next, and yellow after that.
            let base_color = if i % 3 == 0 || i % 5 == 0 {
                *self.current_color()
            } else {
                *self.next_color()
            };

            // 0 (i.e., the base_color) is the brightest; 9 is off
            let brightness = if i % 2 == 0 // even-numbered pixels are unwavering
                // all pixels are solid at the beginning of the Animation
                || self.index() < 20
                // ... and again after a short period of glitching
                || (30 < self.index() && self.index() <= 45)
            {
                0
            } else if 75 <= self.index() && self.index() < 125 {
                // randomizing the brightness every frame in this window creates the glitching effect
                // (for odd pixels only)
                self.rng.gen_range(0..Self::BRIGHTNESS_LEVELS_CNT)
            } else {
                // the odd pixels are off at all other times
                9
            };

            *pixel = base_color.transition(off, brightness, Self::BRIGHTNESS_LEVELS_CNT);
        }
    }
}
