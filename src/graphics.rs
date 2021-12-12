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
}
