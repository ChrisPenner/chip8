use sdl2::rect::Point;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub struct Screen {
    pixels: [bool; WIDTH * HEIGHT],
}

impl Screen {
    pub fn new() -> Screen {
        let mut s = Screen {
            pixels: [true; WIDTH * HEIGHT],
        };
        s.pixels[10] = false;
        s.pixels[40] = false;
        s.pixels[44] = false;
        s.pixels[57] = false;
        s.pixels[300] = false;
        s
    }
    pub fn clear(&mut self) {
        self.pixels.fill(false);
    }
    pub fn pixel_sets(&self) -> (Vec<Point>, Vec<Point>) {
        let (lit, unlit): (Vec<(Point, bool)>, Vec<(Point, bool)>) = self
            .pixels
            .into_iter()
            .enumerate()
            .map(|(i, b)| {
                let x: i32 = (i % WIDTH) as i32;
                let y: i32 = (i / WIDTH) as i32;
                (Point::new(x, y), b)
            })
            .partition(|(_, b)| *b);
        (
            lit.into_iter().map(|(p, _)| p).collect(),
            unlit.into_iter().map(|(p, _)| p).collect(),
        )
    }

    pub fn draw_sprite(&mut self, x: u8, y: u8, sprite: &[u8]) {
        let x: usize = x as usize;
        let y: usize = y as usize;
        for (i, n) in sprite.into_iter().enumerate() {
            let offset = (x * y) + i;
            self.pixels[offset + 0] = (n & 0b10000000) == 0b10000000;
            self.pixels[offset + 1] = (n & 0b01000000) == 0b01000000;
            self.pixels[offset + 2] = (n & 0b00100000) == 0b00100000;
            self.pixels[offset + 3] = (n & 0b00010000) == 0b00010000;
            self.pixels[offset + 4] = (n & 0b00001000) == 0b00001000;
            self.pixels[offset + 5] = (n & 0b00000100) == 0b00000100;
            self.pixels[offset + 6] = (n & 0b00000010) == 0b00000010;
            self.pixels[offset + 7] = (n & 0b00000001) == 0b00000001;
        }
    }
}
