use smart_leds::RGB8;

pub struct ColorSet<'a> {
    colors: &'a [RGB8],
    current_color_index: usize,
}

impl<'a> ColorSet<'a> {
    pub fn new(colors: &'a [RGB8]) -> Self {
        if colors.len() < 2 {
            panic!("At least two colors are required.");
        }

        Self {
            colors,
            current_color_index: 0,
        }
    }

    pub fn mutate(&mut self, leds: &mut [RGB8]) {
        for (i, led) in leds.iter_mut().enumerate() {
            if i % 5 == 0 || i % 3 == 0 {
                *led = *self.current_color();
            } else {
                *led = *self.next_color();
            }
        }
    }

    fn current_color(&mut self) -> &'a RGB8 {
        &self.colors[self.current_color_index]
    }

    fn next_color(&mut self) -> &'a RGB8 {
        self.current_color_index += 1;
        match self.colors.get(self.current_color_index) {
            Some(color) => color,
            None => {
                self.current_color_index = 0;
                &self.colors[0]
            }
        }
    }
}
