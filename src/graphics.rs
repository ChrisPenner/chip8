use sdl2::rect::Point;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub struct Screen {
    pixels: [bool; WIDTH * HEIGHT],
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            pixels: [false; WIDTH * HEIGHT],
        }
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

    pub fn draw_sprite(&mut self, x: u8, y: u8, sprite: &[u8]) -> (bool, Vec<(Point, bool)>) {
        let mut collision = false;
        let x: usize = x as usize;
        let y: usize = y as usize;
        let mut update_vec = vec![];
        for (i, n) in sprite.into_iter().enumerate() {
            let y = y + i;
            let bin: [bool; 8] = u8_to_binary(*n);
            let offset = x + (y * WIDTH);
            collision |= self.pixels[offset..offset + 8]
                .into_iter()
                .zip(bin.into_iter())
                .any(|(before, after)| *before && !after);
            self.pixels[offset..offset + 8].copy_from_slice(&bin);
            let mut draw_ops: Vec<(Point, bool)> = bin
                .into_iter()
                .enumerate()
                .map(|(j, b)| (Point::new((x + j) as i32, y as i32), b))
                .collect();
            update_vec.append(&mut draw_ops)
        }
        return (collision, update_vec);
    }
}

fn u8_to_binary(n: u8) -> [bool; 8] {
    let mut arr = [false; 8];
    arr[0] = (n & 0b10000000) == 0b10000000;
    arr[1] = (n & 0b01000000) == 0b01000000;
    arr[2] = (n & 0b00100000) == 0b00100000;
    arr[3] = (n & 0b00010000) == 0b00010000;
    arr[4] = (n & 0b00001000) == 0b00001000;
    arr[5] = (n & 0b00000100) == 0b00000100;
    arr[6] = (n & 0b00000010) == 0b00000010;
    arr[7] = (n & 0b00000001) == 0b00000001;
    return arr;
}
