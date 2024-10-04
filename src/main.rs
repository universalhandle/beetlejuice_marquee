#![no_std]
#![no_main]

use arduino_hal::spi;
use core::cmp::Ordering;
use panic_halt as _;
use smart_leds::{gamma, SmartLedsWrite, RGB8};
use ws2812_spi::Ws2812;

const LED_CNT: usize = 15;
const TAIL_CNT: usize = 7;

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

    let mut strip = [RGB8::default(); LED_CNT];

    // We are making a running pattern through the strip, whereby one LED (the head)
    // shines bright, dragging a tail of diminishing brightness behind it
    let mut head_index: usize = 0;

    loop {
        for (index, led) in strip.iter_mut().enumerate() {
            *led = match index.cmp(&head_index) {
                Ordering::Greater => RGB8::default(), // off
                Ordering::Equal => RGB8 {
                    r: 255,
                    g: 207,
                    b: 57,
                },
                Ordering::Less => {
                    let segments_behind_head = head_index - index;

                    if segments_behind_head > TAIL_CNT {
                        RGB8::default()
                    } else {
                        dim(led, segments_behind_head, TAIL_CNT);
                        *led
                    }
                }
            };
        }

        // increment the leader for the next cycle
        if head_index < LED_CNT - 1 {
            head_index += 1;
        } else {
            head_index = 0;
        }

        ws.write(gamma(strip.iter().cloned())).unwrap();
        arduino_hal::delay_ms(100 as u16);
    }
}

fn dim(color: &mut RGB8, current_tail_pixel: usize, cnt_tail_pixels: usize) {
    // add one because only the head should be at 100% brightness
    let total_pixels = u8::try_from(cnt_tail_pixels + 1)
        .and_then(|n| Ok(n))
        .unwrap();
    let brightness_factor = total_pixels
        - u8::try_from(current_tail_pixel)
            .and_then(|n| Ok(n))
            .unwrap();

    color.r = color.r / total_pixels * brightness_factor;
    color.g = color.g / total_pixels * brightness_factor;
    color.b = color.b / total_pixels * brightness_factor;
}
