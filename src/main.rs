#![no_std]
#![no_main]

mod effects;
mod tick;

use arduino_hal::spi;
use core::ops::Range;
use effects::{arrow::Arrow, color_set::ColorSet};
use panic_halt as _;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use smart_leds::{gamma, SmartLedsWrite, RGB8};
use tick::Tick;
use ws2812_spi::Ws2812;

// global configurations
const ARROW_LED_CNT: usize = 21;
const ARROW_LED_RANGE: Range<usize> = 0..ARROW_LED_CNT;
const GLITCH_LED_CNT: usize = 5;
const GLITCH_LED_RANGE: Range<usize> = ARROW_LED_CNT..(ARROW_LED_CNT + GLITCH_LED_CNT);
const TAIL_CNT: usize = 5; // does not include the head of the running lights
const TICKS_PER_SEC: u16 = 100;
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
    let mut strip = [RGB8::default(); ARROW_LED_CNT + GLITCH_LED_CNT];

    let mut arrow = Arrow::new(&YELLOW, TAIL_CNT);

    let mut color_set = ColorSet::new(&[RED, YELLOW]);

    let mut tick = Tick::new(TICKS_PER_SEC);
    loop {
        // Assign colors from our palette to the LEDs in a seemingly random but idempotent way.
        // This has to happen inside the loop (and hence can't be actually randomly assigned)
        // because the Arduino doesn't have the memory capacity for the originally assigned
        // plus the currently assigned (e.g., turned off) color for every LED. The non-randomness
        // assures that, even though some LEDs will be turned off in some cycles, each LED will
        // have its original color assigned the next time through, as opposed to being lit red
        // one cycle, dark the next, and yellow after that.
        color_set.mutate(&mut strip[GLITCH_LED_RANGE]);

        // glitchy flash effect
        if tick.elapsed() < 10 || (19 < tick.elapsed() && tick.elapsed() <= 30) {
            // leave the lights on
        } else if 50 <= tick.elapsed() && tick.elapsed() < 90 {
            // the second half of a second, leave the lights on... with a 90% chance of glitching
            for (i, led) in strip[GLITCH_LED_RANGE].iter_mut().enumerate() {
                let random = rng.gen_range(1..=10);
                if i % 2 == 0 && random > 9 {
                    *led = RGB8::default();
                }
            }
        } else {
            // turn off every other light
            for (i, led) in strip[GLITCH_LED_RANGE].iter_mut().enumerate() {
                if i % 2 == 0 {
                    *led = RGB8::default();
                }
            }
        }

        // arrow effect
        if tick.elapsed() % 10 == 0 {
            arrow.mutate(&mut strip[ARROW_LED_RANGE]);
        }

        ws.write(gamma(strip.iter().cloned())).unwrap();
        arduino_hal::delay_ms(tick.len());

        tick.tock();
    }
}
