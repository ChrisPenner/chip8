type OpCode = u16;

use std::fs;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use crate::font;
use crate::graphics;
use rand::Rng;
use sdl2::keyboard::Keycode;

// Short is actually just 4 bits.
type Short = u8;

// https://multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/

pub struct Compy {
    pub memory: [u8; 4096],
    pub reg: [u8; 16],
    pub i: u16,
    pub pc: u16,
    pub gfx: graphics::Screen,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub stack: [u16; 16],
    pub sp: usize,
    pub keys: [bool; 16],
    pub key_buffer: Option<u8>,

    pub start_time_micros: u128,
    pub latest_time_micros: u128,
    pub draw_flag: bool,
    rng: rand::prelude::ThreadRng,
}

impl Compy {
    pub fn new() -> Compy {
        let start_micros = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_micros();
        let mut compy = Compy {
            memory: [0; 4096],
            reg: [0; 16],
            i: 0,
            pc: 0x200,
            gfx: graphics::Screen::new(),
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            keys: [false; 16],
            draw_flag: false,
            key_buffer: None,
            start_time_micros: start_micros,
            latest_time_micros: start_micros,
            rng: rand::thread_rng(),
        };
        compy.memory[0x50..0x50 + font::FONTSET.len()].copy_from_slice(&font::FONTSET);
        return compy;
    }
    pub fn load_rom(&mut self, filename: &str) {
        let bytes = fs::read(filename).expect("Error reading rom");
        // println!("{:#?}", bytes);
        self.memory[0x200..0x200 + bytes.len()].copy_from_slice(&bytes);
        // self.memory
        //     .into_iter()
        //     .enumerate()
        //     .for_each(|(i, b)| println!("{:#x}: {:#x}", i, b));
    }
    fn select_char(&mut self, which_char: u8) {
        self.i = 0x50 + (which_char as u16 * 5);
    }
    pub fn single_cycle(&mut self) {
        let pc = self.pc as usize;
        // Fetch Opcode
        let opcode: OpCode = ((self.memory[pc] as u16) << 8) | (self.memory[pc + 1] as u16);

        println!("Running opcode: {:#x}", opcode);
        self.run_op(opcode);
        self.print_state();
        self.key_buffer = None;

        // Update timers
        let now_micros = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_micros();
        let frames_since_start = (now_micros - self.start_time_micros) / 60_000_000;
        let frames_since_latest = (now_micros - self.latest_time_micros) / 60_000_000;
        let new_frames = frames_since_latest - frames_since_start;
        self.latest_time_micros = now_micros;

        self.sound_timer = self.sound_timer.saturating_sub(new_frames as u8);
        self.delay_timer = self.delay_timer.saturating_sub(new_frames as u8);
        ()
    }
    fn clear_display(&mut self) {
        self.gfx.clear();
    }

    // Keypad                   Keyboard
    // +-+-+-+-+                +-+-+-+-+
    // |1|2|3|C|                |1|2|3|4|
    // +-+-+-+-+                +-+-+-+-+
    // |4|5|6|D|                |Q|W|E|R|
    // +-+-+-+-+       =>       +-+-+-+-+
    // |7|8|9|E|                |A|S|D|F|
    // +-+-+-+-+                +-+-+-+-+
    // |A|0|B|F|                |Z|X|C|V|
    // +-+-+-+-+                +-+-+-+-+
    pub fn set_key_state(&mut self, state: bool, keycode: Keycode) {
        if let Some(keynum) = keynum_for_keycode(keycode) {
            self.keys[keynum as usize] = state;
            self.key_buffer = Some(keynum);
        }
    }

    fn return_from_sub(&mut self) {
        self.pc = self.stack[self.sp];
        self.sp -= 1;
    }

    fn step_pc(&mut self) {
        self.pc += 2;
    }

    fn skip_when(&mut self, cond: bool) {
        if cond {
            self.step_pc();
            self.step_pc();
        } else {
            self.step_pc();
        }
    }

    fn subroutine(&mut self, addr: u16) {
        self.stack[self.sp] = self.pc;
        self.pc = addr;
    }

    // Get a slice of memory starting from I
    fn mem_slice(&self, size: usize) -> &[u8] {
        let i: usize = self.i as usize;
        &self.memory[i..i + size]
    }

    pub fn print_state(&self) {
        println!("Reg: {:?}", self.reg);
        println!("PC: {:#x}, I: {:#x}", self.pc, self.i);
    }
    pub fn run_op(&mut self, opcode: OpCode) {
        let shorts: (Short, Short, Short, Short) = (
            ((opcode & 0xF000) >> 12) as Short,
            ((opcode & 0x0F00) >> 8) as Short,
            ((opcode & 0x00F0) >> 4) as Short,
            (opcode & 0x000F) as Short,
        );
        match shorts {
            // clear display
            (0x0, 0x0, 0xE, 0x0) => {
                self.clear_display();
                self.step_pc();
            }
            // RETURN
            (0x0, 0x0, 0xE, 0xE) => {
                self.return_from_sub();
            }
            // Call machine code (optional)
            (0x0, _x, _y, _z) => {
                panic!("Machine code procedures not implemented.");
            }
            // GOTO NNN
            (0x1, n1, n2, n3) => {
                println!("ADDR: {:#x}", addr(n1, n2, n3));
                self.pc = addr(n1, n2, n3);
            }
            // Call Subroutine at NNN
            (0x2, n1, n2, n3) => self.subroutine(addr(n1, n2, n3)),
            // Skip when (Vx == NN)
            (0x3, x, n1, n2) => {
                println!("{} == {}", self.reg[x as usize], val(n1, n2));
                self.skip_when(self.reg[x as usize] == val(n1, n2));
            }
            // Skip when (Vx != NN)
            (0x4, x, n1, n2) => self.skip_when(self.reg[x as usize] != val(n1, n2)),
            // Skip when (Vx == Vy)
            (0x5, x, y, 0x0) => self.skip_when(self.reg[x as usize] == self.reg[y as usize]),
            // Vx = NN
            (0x6, x, n1, n2) => {
                self.reg[x as usize] = val(n1, n2);
                self.step_pc();
            }
            // Vx += NN
            (0x7, x, n1, n2) => {
                let (r, _) = self.reg[x as usize].overflowing_add(val(n1, n2));
                self.reg[x as usize] = r;
                self.step_pc();
            }
            // Vx = Vy
            (0x8, x, y, 0x0) => {
                self.reg[x as usize] = self.reg[y as usize];
                self.step_pc();
            }
            // Bitwise OR, Vx |= Vy
            (0x8, x, y, 0x1) => {
                self.reg[x as usize] |= self.reg[y as usize];
                self.step_pc();
            }
            // Bitwise AND, Vx &= Vy
            (0x8, x, y, 0x2) => {
                self.reg[x as usize] &= self.reg[y as usize];
                self.step_pc();
            }
            // Bitwise XOR, Vx ^= Vy
            (0x8, x, y, 0x3) => {
                self.reg[x as usize] ^= self.reg[y as usize];
                self.step_pc();
            }
            // Vx += Vy
            (0x8, x, y, 0x4) => {
                let vx = self.reg[x as usize];
                let vy = self.reg[y as usize];
                let (r, carry) = vx.overflowing_add(vy);
                self.reg[x as usize] = r;
                self.reg[0xf] = carry as u8;
                self.step_pc();
            }
            // Vx -= Vy
            (0x8, x, y, 0x5) => {
                let vx = self.reg[x as usize];
                let vy = self.reg[y as usize];
                // TODO: double-check this logic;
                let (r, did_borrow) = vx.borrowing_sub(vy, false);
                self.reg[x as usize] = r;
                self.reg[0xf] = !did_borrow as u8;
                self.step_pc();
            }
            // Vx >>= 1
            (0x8, x, y, 0x6) => {
                let vx = self.reg[x as usize];
                self.reg[0xf] = vx & 0x1;
                self.reg[x as usize] >>= self.reg[y as usize];
                self.step_pc();
            }
            // Vx = Vy - Vx
            (0x8, x, y, 0x7) => {
                let vx = self.reg[x as usize];
                let vy = self.reg[y as usize];

                let (r, did_borrow) = vy.borrowing_sub(vx, false);
                self.reg[x as usize] = r;
                self.reg[0xf] = !did_borrow as u8;
                self.step_pc();
            }
            // Vx <<= 1
            (0x8, x, y, 0xE) => {
                let vx = self.reg[x as usize];
                self.reg[0xf] = vx >> 7;
                self.reg[x as usize] <<= self.reg[y as usize];
                self.step_pc();
            }
            // Skip when Vx != Vy
            (0x9, x, y, 0x0) => self.skip_when(self.reg[x as usize] != self.reg[y as usize]),
            // I = NNN
            (0xA, n1, n2, n3) => {
                self.i = addr(n1, n2, n3);
                self.step_pc();
            }
            // PC = V0 + NNN
            // TODO: do we store a stack pointer here?
            (0xB, n1, n2, n3) => self.pc = self.reg[0] as u16 + addr(n1, n2, n3),

            (0xC, x, n1, n2) => {
                self.reg[x as usize] = self.rng.gen::<u8>() & val(n1, n2);
                self.step_pc()
            }
            // draw_sprite(Vx, Vy, N)
            (0xD, x, y, n) => {
                let vx = self.reg[x as usize];
                let vy = self.reg[y as usize];
                let sprite = self.mem_slice(n as usize).to_vec();
                let collision = self.gfx.draw_sprite(vx, vy, &sprite);
                self.reg[0xf] = collision as u8;
                self.step_pc();
            }
            // Skip if (key(Vx))
            (0xE, x, 0x9, 0xE) => {
                let keycode = self.reg[x as usize];
                self.skip_when(self.keys[keycode as usize])
            }
            // Skip if (!key(Vx))
            (0xE, x, 0xA, 0x1) => {
                let keycode = self.reg[x as usize];
                self.skip_when(!self.keys[keycode as usize])
            }
            // Vx = delay_timer()
            (0xF, x, 0x0, 0x7) => {
                self.reg[x as usize] = self.delay_timer;
                self.step_pc();
            }
            // Vx = wait_for_key()
            (0xF, x, 0x0, 0xA) => {
                if let Some(keynum) = self.key_buffer {
                    self.reg[x as usize] = keynum;
                    self.step_pc();
                }

                // TODO: check if timers still need to count down or not while blocking for a
                // key?
            }
            // delay_timer = Vx
            (0xF, x, 0x1, 0x5) => {
                self.delay_timer = self.reg[x as usize];
                self.step_pc();
            }
            // sound_timer = Vx
            (0xF, x, 0x1, 0x8) => {
                self.sound_timer = self.reg[x as usize];
                self.step_pc();
            }
            // I += Vx
            (0xF, x, 0x1, 0xE) => {
                self.i += self.reg[x as usize] as u16;
                self.step_pc();
            }
            // I = select_char(Vx)
            (0xF, x, 0x2, 0x9) => {
                self.select_char(self.reg[x as usize]);
                self.step_pc();
            }
            // Binary Coded Decimal
            // set_BCD(Vx)
            //   *(I+0) = BCD(3)
            //   *(I+1) = BCD(2)
            //   *(I+2) = BCD(1);
            (0xF, x, 0x3, 0x3) => {
                let n = self.reg[x as usize];
                let i = self.i as usize;
                // TODO: double check this logic.
                self.memory[i] = n / 100;
                self.memory[i + 1] = (n % 100) / 10;
                self.memory[i + 2] = n % 10;
                self.step_pc();
            }
            // Dump V0-Vx (inclusive) into memory at I.
            (0xF, x, 0x5, 0x5) => {
                let reg_data = self.reg[0..=(x as usize)].to_vec();
                let i: usize = self.i as usize;
                self.memory[i..=i + (x as usize)].copy_from_slice(&reg_data);
                self.step_pc();
            }
            // Fill V0 -> Vx (inclusive) from memory at I.
            (0xF, x, 0x6, 0x5) => {
                let reg_data = self.mem_slice(x as usize).to_vec();
                self.reg[0..=(x as usize)].copy_from_slice(&reg_data);
                self.step_pc();
            }
            _ => panic!("Bad op-code. {:#x}", opcode),
        }
    }
}

pub fn addr(n1: Short, n2: Short, n3: Short) -> u16 {
    ((n1 as u16) << 8) | ((n2 as u16) << 4) | (n3 as u16)
}

pub fn val(n1: Short, n2: Short) -> u8 {
    ((n1 as u8) << 4) | (n2 as u8)
}

pub fn keynum_for_keycode(keycode: Keycode) -> Option<u8> {
    match keycode {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xc),
        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::R => Some(0xd),
        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::F => Some(0xe),
        Keycode::Z => Some(0xa),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xb),
        Keycode::V => Some(0xf),
        _ => None,
    }
}
