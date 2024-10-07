#![no_std]
#![no_main]

mod effects;
mod tick;

use arduino_hal::spi;
use effects::running_lights::RunningLights;
use panic_halt as _;
use smart_leds::{gamma, SmartLedsWrite, RGB8};
use tick::Tick;
use ws2812_spi::Ws2812;

// global configurations
const LED_CNT: usize = 15;
// does not include the head of the running lights
const TAIL_CNT: usize = 7;
const TICKS_PER_SEC: u16 = 100;

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

    let running_color = RGB8 {
        r: 255,
        g: 207,
        b: 57,
    };
    let run_in_reverse = false;
    let mut effect = RunningLights::new(&running_color, run_in_reverse, TAIL_CNT);

    // define the strip with the LEDs initialized in the "off" setting
    let mut strip = [RGB8::default(); LED_CNT];

    let mut tick = Tick::new(TICKS_PER_SEC);
    loop {
        // let's select all LEDs except the first four to use for the running effect
        let running_leds = &mut strip[4..];
        if tick.elapsed() % 10 == 0 {
            effect.mutate(running_leds);
        }

        ws.write(gamma(strip.iter().cloned())).unwrap();
        arduino_hal::delay_ms(1_000 / TICKS_PER_SEC);

        tick.tock();
    }
}
