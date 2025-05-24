#![no_std]
#![no_main]

use arduino_hal::{spi, Spi};
use panic_halt as _;
use smart_leds_animations::{
    animations::{Arrow, Snake},
    composition::{Compose, Parallel, Series},
    harness::*,
    smart_leds::RGB8,
};
use ws2812_spi::Ws2812;

mod animations;
use animations::Glitch;

/// The length of the tail for each Snake animation (does not include the head)
const TAIL_CNT: usize = 20;
const CHASE_TAIL: bool = false;
const WRAP_ARROW: bool = false;
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

#[arduino_hal::entry]
fn main() -> ! {
    let writer = set_up_writer();

    // Hardware setup, which is really the domain of the `arduino_hal` and `smart_leds` crates, is tucked away into the
    // `set_up_writer()` function, invoked above, so that `main()` may highlight the role of the `smart_leds_animations`
    // crate in creating the display pictured at https://github.com/universalhandle/smart_leds_animations/raw/main/example.gif.
    // In this project, a single, unbroken LED strip borders the marquee (a rectangle with an arrow going through it). In
    // many cases the visual effects are realized across noncontiguous pixels.

    let driver = DriverBuilder::new(writer)
        .enable_gamma_correction(true)
        .build();

    let mut director = DirectorBuilder::new(driver).build();

    // Define the strip with the LEDs initialized in the "off" setting.
    let mut pixels = [RGB8::default(); 300];

    // The pixels bordering the rectangle should use our custom `Glitch` animation. Since the pixels in question
    // are not contiguous on the LED strip, three separate `Glitch` animations are needed.
    let colors = [RED, YELLOW];
    let mut glitch1 = Glitch::new(&colors, 0..=43);
    let mut glitch2 = Glitch::new(&colors, 92..=175);
    let mut glitch3 = Glitch::new(&colors, 213..=299);

    // The next major component of the animation is an arrow broken by the rectangular section of the marquee.
    // First, we declare a pair of `Snake` animations to act as the fletching of the arrow.
    let run_in_reverse = true;
    let mut left_leg = Snake::new(YELLOW, CHASE_TAIL, 199..=212, !run_in_reverse, TAIL_CNT);
    let mut right_leg = Snake::new(YELLOW, CHASE_TAIL, 176..=189, run_in_reverse, TAIL_CNT);
    // These are grouped together in a `Parallel` animation because they need to be treated as a single unit in the
    // `Series` animation that represents the broken arrow as a whole.
    let mut fletching_parts: [&mut dyn Compose<RGB8>; 2] = [&mut left_leg, &mut right_leg];
    let mut fletching = Parallel::new(&mut fletching_parts);

    // Next, we declare an `Arrow` animation for the pointy part of the broken arrow.
    let mut arrow = Arrow::new(YELLOW, 44..=91, TAIL_CNT, WRAP_ARROW);

    // Now we compose the fletching and arrow into a single `Series` animation.
    let mut broken_arrow_parts: [(&mut dyn Compose<RGB8>, usize); 2] = [
        // The lights appear to run up the fletching and disappear behind the rectangle. Then, after the specified delay (in animation frames)…
        (&mut fletching, 0),
        // …the lights reappear and continue toward the point of the arrow. After the specifiied delay the compound animation restarts.
        (&mut arrow, 0),
    ];
    let mut broken_arrow = Series::new(&mut broken_arrow_parts);

    // The show starts the Director calls action. We just have to hand it the pixels and the animations we configured.
    director.action(
        &mut pixels,
        &mut [&mut glitch1, &mut glitch2, &mut broken_arrow, &mut glitch3],
    );
}

/// Configures the LED writer for this particular hardware.
///
/// In this case, I'm using a WS2812B LED strip with an Arduino Uno R3. This code is fairly boilerplate; I adapted it
/// from [the avr-hal-template](https://github.com/Rahix/avr-hal-template) and [one of the `smart_leds` examples](
/// https://github.com/smart-leds-rs/smart-leds-samples/blob/5bea1bb763fd4b068548c00197e211b31af41d18/avr-examples/examples/avr_ws2812_blink_spi_pre.rs).
fn set_up_writer() -> Ws2812<Spi> {
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);

    // spi: serial peripheral interface
    let (spi, _) = spi::Spi::new(
        peripherals.SPI,
        pins.d13.into_output(),
        pins.d11.into_output(),
        pins.d12.into_pull_up_input(),
        pins.d10.into_output(),
        spi::Settings::default(),
    );
    Ws2812::new(spi)
}
