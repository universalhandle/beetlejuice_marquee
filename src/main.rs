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
    let run_in_reverse = true;
    let mut effect_run_up = RunningLights::new(&running_color, !run_in_reverse, TAIL_CNT);
    let mut effect_run_down = RunningLights::new(&running_color, run_in_reverse, TAIL_CNT + 1);

    // define the strip with the LEDs initialized in the "off" setting
    let mut strip = [RGB8::default(); LED_CNT];

    let mut tick = Tick::new(TICKS_PER_SEC);
    loop {
        let (leds_arrow_left, leds_unalloc) = strip.split_at_mut(10);
        let (led_arrow_point, leds_arrow_right) = leds_unalloc.split_at_mut(1);

        if tick.elapsed() % 10 == 0 {
            effect_run_up.mutate(leds_arrow_left);
            effect_run_down.mutate(leds_arrow_right);
            led_arrow_point[0] = effect_run_up.color_at_terminus();
            led_arrow_point[0].r = (led_arrow_point[0].r / 9).saturating_mul(10);
            led_arrow_point[0].g = (led_arrow_point[0].g / 9).saturating_mul(10);
            led_arrow_point[0].b = (led_arrow_point[0].b / 9).saturating_mul(10);
        }

        ws.write(gamma(strip.iter().cloned())).unwrap();
        arduino_hal::delay_ms(1_000 / TICKS_PER_SEC);

        tick.tock();
    }
}
