#![no_std]
#![no_main]

mod animation;
mod effects;

use animation::Animation;
use arduino_hal::spi;
use core::cmp::Ordering;
use core::ops::{RangeFrom, RangeInclusive, RangeTo};
use effects::{arrow::Arrow, color_set::ColorSet, running_lights::RunningLights};
use panic_halt as _;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use smart_leds::{gamma, SmartLedsWrite, RGB8};
use ws2812_spi::Ws2812;

// global configurations
const ARROW_START: usize = 44;
const ARROW_END: usize = 91;
const LEFT_LEG_START: usize = 199;
const LEFT_LEG_END: usize = 212;
const RIGHT_LEG_START: usize = 176;
const RIGHT_LEG_END: usize = 189;
const GLITCH_FRAME_CNT: usize = 150;
const SPLIT_ARROW_FRAME_CNT: usize = 100;
const LED_CNT: usize = 300;
const TAIL_CNT: usize = 20; // does not include the head of the running lights
const TAIL_MUST_EXIT_BEFORE_RESTART: bool = true;
const YELLOW: RGB8 = RGB8 {
    r: 255,
    g: 180,
    b: 47,
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
const LEFT_LEG_LED_RANGE: RangeInclusive<usize> = LEFT_LEG_START..=LEFT_LEG_END;
const RIGHT_LEG_LED_RANGE: RangeInclusive<usize> = RIGHT_LEG_START..=RIGHT_LEG_END;
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

    let mut arrow = Arrow::new(&YELLOW, TAIL_CNT, TAIL_MUST_EXIT_BEFORE_RESTART);

    let mut color_set = ColorSet::new(&[RED, YELLOW]);

    let run_in_reverse = true;
    let mut left_leg = RunningLights::new(
        &YELLOW,
        TAIL_MUST_EXIT_BEFORE_RESTART,
        !run_in_reverse,
        TAIL_CNT,
    );
    let mut right_leg = RunningLights::new(
        &YELLOW,
        TAIL_MUST_EXIT_BEFORE_RESTART,
        run_in_reverse,
        TAIL_CNT,
    );

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
            // get glitchy
            for led in strip[GLITCH_LED_RANGE_1].iter_mut().step_by(2) {
                let random = rng.gen_range(1..=10);
                if random > 9 {
                    *led = RGB8::default();
                } else if random > 4 {
                    *led = dim(led, 2);
                }
            }
            for led in strip[GLITCH_LED_RANGE_2].iter_mut().step_by(2) {
                let random = rng.gen_range(1..=10);
                if random > 9 {
                    *led = RGB8::default();
                } else if random > 4 {
                    *led = dim(led, 2);
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

        // split arrow effect -- this repeats after 100 frames (a third as often as the overall animation);
        // to keep the conditions below a little simpler provide an effect-specific frame counter
        let split_arrow_frames_displayed =
            match animation.frames_displayed().cmp(&SPLIT_ARROW_FRAME_CNT) {
                Ordering::Less => animation.frames_displayed(),
                Ordering::Equal => 0,
                Ordering::Greater => animation
                    .frames_displayed()
                    .saturating_sub(SPLIT_ARROW_FRAME_CNT),
            };

        // assumes the legs are the same length -- the number of frames required for the running lights to
        // completely exit the legs of the arrow
        if split_arrow_frames_displayed < strip[LEFT_LEG_LED_RANGE].len() + TAIL_CNT {
            left_leg.mutate(&mut strip[LEFT_LEG_LED_RANGE]);
            right_leg.mutate(&mut strip[RIGHT_LEG_LED_RANGE]);
        // the number of frames required for the above leg animation (LEFT_LEG plus TAIL), plus the arrow head
        // animation (divide number of pixels by two because an arrow is essentially a set of parallel running
        // lights effects, plus the tail length)
        } else if split_arrow_frames_displayed
            < strip[LEFT_LEG_LED_RANGE].len() + strip[ARROW_LED_RANGE].len() / 2 + TAIL_CNT * 2
        {
            arrow.mutate(&mut strip[ARROW_LED_RANGE]);
        }

        ws.write(gamma(strip.iter().cloned())).unwrap();

        animation.next();
    }
}

fn dim(led: &RGB8, factor: usize) -> RGB8 {
    let factor = u8::try_from(factor).and_then(|n| Ok(n)).unwrap();

    RGB8 {
        r: (led.r / factor),
        g: (led.g / factor),
        b: (led.b / factor),
    }
}
