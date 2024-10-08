#![no_std]
#![no_main]

mod effects;
mod tick;

use arduino_hal::spi;
use effects::arrow::Arrow;
use panic_halt as _;
use smart_leds::{gamma, SmartLedsWrite, RGB8};
use tick::Tick;
use ws2812_spi::Ws2812;

// global configurations
const LED_CNT: usize = 21;
// does not include the head of the running lights
const TAIL_CNT: usize = 5;
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
    let mut arrow = Arrow::new(&running_color, TAIL_CNT);

    // define the strip with the LEDs initialized in the "off" setting
    let mut strip = [RGB8::default(); LED_CNT];

    let mut tick = Tick::new(TICKS_PER_SEC);
    loop {
        if tick.elapsed() % 10 == 0 {
            arrow.mutate(&mut strip);
        }

        ws.write(gamma(strip.iter().cloned())).unwrap();
        arduino_hal::delay_ms(1_000 / TICKS_PER_SEC);

        tick.tock();
    }
}
