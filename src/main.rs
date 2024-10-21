#![no_std]
#![no_main]

mod animation;
mod effects;

use animation::Animation;
use arduino_hal::spi;
use core::cmp::Ordering;
use core::ops::{RangeFrom, RangeInclusive, RangeTo};
use effects::{arrow::Arrow, color_set::ColorSet};
use panic_halt as _;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use smart_leds::{gamma, SmartLedsWrite, RGB8};
use ws2812_spi::Ws2812;

// global configurations
const ARROW_START: usize = 100;
const ARROW_END: usize = 200;
const GLITCH_FRAME_CNT: usize = 150;
const LED_CNT: usize = 300;
const TAIL_CNT: usize = 15; // does not include the head of the running lights
const YELLOW: RGB8 = RGB8 {
    r: 255,
    g: 207,
    b: 57,
};
const RED: RGB8 = RGB8 {
    r: 253,
    g: 74,
    b: 65,
};

// somewhat derived configs...
const ARROW_LED_RANGE: RangeInclusive<usize> = ARROW_START..=ARROW_END;
const GLITCH_LED_RANGE_1: RangeTo<usize> = ..ARROW_START;
const GLITCH_LED_RANGE_2: RangeFrom<usize> = (ARROW_END + 1)..;
// end config

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let (spi, _) = spi::Spi::new(
        dp.SPI,
        pins.d13.into_output(),
        pins.d11.into_output(),
        pins.d12.into_pull_up_input(),
        pins.d10.into_output(),
        spi::Settings::default(),
    );
    let mut ws = Ws2812::new(spi);

    let mut rng = SmallRng::seed_from_u64(361279846193);

    // define the strip with the LEDs initialized in the "off" setting
    let mut strip = [RGB8::default(); LED_CNT];

    let mut arrow = Arrow::new(&YELLOW, TAIL_CNT);

    let mut color_set = ColorSet::new(&[RED, YELLOW]);

    let mut animation = Animation::new(LED_CNT);
    loop {
        // Assign colors from our palette to the LEDs in a seemingly random but idempotent way.
        // This has to happen inside the loop (and hence can't be actually randomly assigned)
        // because the Arduino doesn't have the memory capacity for the originally assigned
        // plus the currently assigned (e.g., turned off) color for every LED. The non-randomness
        // assures that, even though some LEDs will be turned off in some cycles, each LED will
        // have its original color assigned the next time through, as opposed to being lit red
        // one cycle, dark the next, and yellow after that.
        color_set.mutate(&mut strip);

        // glitchy flash effect -- this repeats after 150 frames (half as often as the overall animation);
        // to keep the conditions below a little simpler provide an effect-specific frame counter
        let glitch_frames_displayed = match animation.frames_displayed().cmp(&GLITCH_FRAME_CNT) {
            Ordering::Less => animation.frames_displayed(),
            Ordering::Equal => 0,
            Ordering::Greater => animation
                .frames_displayed()
                .saturating_sub(GLITCH_FRAME_CNT),
        };

        if glitch_frames_displayed < 20
            || (30 < glitch_frames_displayed && glitch_frames_displayed <= 45)
        {
            // leave the lights on
        } else if 75 <= glitch_frames_displayed && glitch_frames_displayed < 125 {
            // the second half of a second, leave the lights on... with a 90% chance of glitching
            for led in strip[GLITCH_LED_RANGE_1].iter_mut().step_by(2) {
                let random = rng.gen_range(1..=10);
                if random > 9 {
                    *led = RGB8::default();
                }
            }
            for led in strip[GLITCH_LED_RANGE_2].iter_mut().step_by(2) {
                let random = rng.gen_range(1..=10);
                if random > 9 {
                    *led = RGB8::default();
                }
            }
        } else {
            // turn off every other light
            for led in strip[GLITCH_LED_RANGE_1].iter_mut().step_by(2) {
                *led = RGB8::default();
            }
            for led in strip[GLITCH_LED_RANGE_2].iter_mut().step_by(2) {
                *led = RGB8::default();
            }
        }

        // arrow effect
        arrow.mutate(&mut strip[ARROW_LED_RANGE]);

        ws.write(gamma(strip.iter().cloned())).unwrap();

        animation.next();
    }
}
