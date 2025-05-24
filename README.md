# Beetlejuice Marquee: a Halloween decoration made with [`smart_leds_animations`]

This repository contains code for a Halloween decoration modeled after the marquee from the graveyard scene in
*Beetlejuice* (1988). It uses an Arduino Uno R3 to drive a 300-pixel WS2812B <acronym title="Light-Emitting Diode">LED</acronym>
strip bordering an irregularly shaped foam board. The code is specific to all of these details, which is to say that, if you are
reading this and you are not me, this code is probably useful to you only as a reference implementation of [`smart_leds_animations`].

<div style="float: right; margin: 0 0 10px 10px;">

![Photo of finished project; marquee reads: Beteleguese Betelegeuse][photo]

</div>

Pictured here is the finished product. A single, unbroken
<acronym title="Light-Emitting Diode">LED</acronym> strip borders the marquee, meaning that in many cases the visual effects are
realized across noncontiguous pixels. The two main animations are implemented as follows:

- **Broken arrow:** The main building block of this animation is the [`Snake`]; there are four of them in play. Each side
  of the arrow above the lettering is a [`Snake`]. These are grouped together in a [`Parallel`] animation because they need
  to be treated as a single unit in the [`Series`] animation that represents the broken arrow as a whole. The other component
  in the [`Series`] is an [`Arrow`], which is little more than two converging [`Snake`]s with a little extra logic to handle
  some edge cases.
- **Glitch:** I have taken to calling the rectangle of pixels around the lettering a [`Glitch`] effect. Because I thought it too
  bespoke for a general-purpose library, it is implemented as a custom animation in this project. Hopefully it's a useful example
  of how users of [`smart_leds_animations`] might use the [`AnimateFrames`] trait.

<br style="clear:both" />

## Build Instructions

1. Install prerequisites as described in the [`avr-hal` README] (`avr-gcc`, `avr-libc`, `avrdude`, [`ravedude`]).

2. Run `cargo build` to build the firmware.

3. Run `cargo run` to flash the firmware to a connected board.  If `ravedude`
   fails to detect your board, check its documentation at
   <https://crates.io/crates/ravedude>.

4. `ravedude` will open a console session after flashing where you can interact
   with the UART console of your board.

[`avr-hal` README]: https://github.com/Rahix/avr-hal#readme
[`ravedude`]: https://crates.io/crates/ravedude

## License

Distributed under the terms of both the [MIT license](./LICENSE-MIT)
and the [Apache License (Version 2.0)](./LICENSE-APACHE).

Any contribution intentionally submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Contributing

Issues, pull requests, feature requests, and constructive criticism are welcome.

[`AnimateFrames`]: <https://docs.rs/smart_leds_animations/latest/smart_leds_animations/animate/trait.AnimateFrames.html>
[`Arrow`]: <https://docs.rs/smart_leds_animations/latest/smart_leds_animations/animations/struct.Arrow.html>
[`Glitch`]: <./src/animations/glitch.rs>
[`Parallel`]: <https://docs.rs/smart_leds_animations/latest/smart_leds_animations/composition/struct.Parallel.html>
[photo]: <https://raw.githubusercontent.com/universalhandle/smart_leds_animations/refs/heads/main/example.gif>
[`Series`]: <https://docs.rs/smart_leds_animations/latest/smart_leds_animations/composition/struct.Series.html>
[`smart_leds_animations`]: <https://github.com/universalhandle/smart_leds_animations>
[`Snake`]: <https://docs.rs/smart_leds_animations/latest/smart_leds_animations/animations/struct.Snake.html>
