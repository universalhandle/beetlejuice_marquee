// adapted from https://github.com/smart-leds-rs/smart-leds-samples/blob/master/avr-examples/examples/avr_ws2812_blink_spi_pre.rs,
// targeting the Arduino Uno (via the generic HAL) rather than the Arduino Leonardo

#![no_std]
#![no_main]

use panic_halt as _;

use arduino_hal::spi;
use smart_leds::{SmartLedsWrite, RGB8};
use ws2812_spi::Ws2812;

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

    let mut data: [RGB8; 3] = [RGB8::default(); 3];
    let empty: [RGB8; 3] = [RGB8::default(); 3];
    let mut ws = Ws2812::new(spi);

    loop {
        data[0] = RGB8 {
            r: 0,
            g: 0,
            b: 0x10,
        };
        data[1] = RGB8 {
            r: 0,
            g: 0x10,
            b: 0,
        };
        data[2] = RGB8 {
            r: 0x10,
            g: 0,
            b: 0,
        };
        ws.write(data.iter().cloned()).unwrap();
        arduino_hal::delay_ms(1000 as u16);
        ws.write(empty.iter().cloned()).unwrap();
        arduino_hal::delay_ms(1000 as u16);
    }
}
