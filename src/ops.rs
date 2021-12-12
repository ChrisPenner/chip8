type OpCode = u16;

use crate::graphics;

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
    pub key: [bool; 16],

    pub draw_flag: bool,
}

pub fn machine() {
    // 0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
    // 0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
    // 0x200-0xFFF - Program ROM and work RAM
    println!("hi");

    loop {
        // Emulate one cycle

        // If the draw flag is set, update the screen
        // drawGraphics();

        // Store key press state (Press and Release)
        // myChip8.setKeys();
    }
}

impl Compy {
    pub fn new() -> Compy {
        return Compy {
            memory: [0; 4096],
            reg: [0; 16],
            i: 0,
            pc: 0x200,
            gfx: graphics::Screen::new(),
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            key: [false; 16],
            draw_flag: false,
        };
    }
    pub fn single_cycle(&mut self) {
        let pc = self.pc as usize;
        // Fetch Opcode
        let opcode: OpCode = ((self.memory[pc] as u16) << 8) | (self.memory[pc + 1] as u16);
        self.run_op(opcode);

        // Update timers
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
        match keycode {
            Keycode::Num1 => self.key[0x1] = state,
            Keycode::Num2 => self.key[0x2] = state,
            Keycode::Num3 => self.key[0x3] = state,
            Keycode::Num4 => self.key[0xc] = state,
            Keycode::Q => self.key[0x4] = state,
            Keycode::W => self.key[0x5] = state,
            Keycode::E => self.key[0x6] = state,
            Keycode::R => self.key[0xd] = state,
            Keycode::A => self.key[0x7] = state,
            Keycode::S => self.key[0x8] = state,
            Keycode::D => self.key[0x9] = state,
            Keycode::F => self.key[0xe] = state,
            Keycode::Z => self.key[0xa] = state,
            Keycode::X => self.key[0x0] = state,
            Keycode::C => self.key[0xb] = state,
            Keycode::V => self.key[0xf] = state,
            _ => (),
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
            // subroutine NNN
            (0x1, n1, n2, n3) => self.pc = addr(n1, n2, n3),
            // Call Subroutine at NNN
            (0x2, n1, n2, n3) => self.subroutine(addr(n1, n2, n3)),
            // Skip when (Vx == NN)
            (0x3, x, n1, n2) => {
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
                self.reg[x as usize] += val(n1, n2);
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
                self.reg[x as usize] += self.reg[y as usize];
                self.step_pc();
            }
            // Vx -= Vy
            (0x8, x, y, 0x5) => {
                self.reg[x as usize] -= self.reg[y as usize];
                self.step_pc();
            }
            // Vx >>= 1
            (0x8, x, y, 0x6) => {
                self.reg[x as usize] >>= self.reg[y as usize];
                self.step_pc();
            }
            // Vx = Vy - Vx
            (0x8, x, y, 0x7) => {
                self.reg[x as usize] = self.reg[y as usize] - self.reg[x as usize];
                self.step_pc();
            }
            // Vx <<= 1
            (0x8, x, y, 0xE) => {
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
            // draw_sprite(Vx, Vy, N)
            (0xD, x, y, n) => {
                let sprite = self.mem_slice(n as usize * 8).to_vec();
                self.gfx.draw_sprite(x, y, &sprite);
                self.step_pc();
            }
            // Skip if (key(Vx))
            (0xE, x, 0x9, 0xE) => (),
            // Skip if (!key(Vx))
            (0xE, x, 0xA, 0x1) => (),
            // Vx = delay_timer()
            (0xF, x, 0x0, 0x7) => (),
            // Vx = wait_for_key()
            (0xF, x, 0x0, 0xA) => (),
            // delay_timer = Vx
            (0xF, x, 0x1, 0x5) => (),
            // sound_timer = Vx
            (0xF, x, 0x1, 0x8) => (),
            // I += Vx
            (0xF, x, 0x1, 0xE) => (),
            // I = select_char(Vx)
            (0xF, x, 0x2, 0x9) => (),
            // Binary Coded Decimal
            // set_BCD(Vx)
            //   *(I+0) = BCD(3)
            //   *(I+1) = BCD(2)
            //   *(I+2) = BCD(1);
            (0xF, x, 0x3, 0x3) => (),
            // Store V0-Vx (inclusive) in memory from I.
            (0xF, x, 0x5, 0x5) => (),
            // Fill V0 -> Vx (inclusive) from memory at I.
            (0xF, x, 0x6, 0x5) => (),
            _ => (),
        }
    }
}

pub fn addr(n1: Short, n2: Short, n3: Short) -> u16 {
    ((n1 as u16) << 8) & ((n2 as u16) << 4) & (n3 as u16)
}

pub fn val(n1: Short, n2: Short) -> u8 {
    ((n1 as u8) << 4) & (n2 as u8)
}

// (0x0, 0x0, 0xE, 0x0) => self.cls(),
// (0x0, 0x0, 0xE, 0xE) => self.ret(),
// // 0NNN = sys addr : ignore
// (0x1, _, _, _) => self.jump_addr(op & 0x0FFF),
// (0x2, _, _, _) => self.call_addr(op & 0x0FFF),
// (0x3, x, _, _) => self.se_vx_nn(x, (op & 0x00FF) as u8),
// (0x4, x, _, _) => self.sne_vx_nn(x, (op & 0x00FF) as u8),
// (0x5, x, y, 0x0) => self.se_vx_vy(x, y),
// (0x6, x, _, _) => self.ld_vx_nn(x, (op & 0x00FF) as u8),
// (0x7, x, _, _) => self.add_vx_nn(x, (op & 0x00FF) as u8),
// (0x8, x, y, 0x0) => self.ld_vx_vy(x, y),
// (0x8, x, y, 0x1) => self.or_vx_vy(x, y),
// (0x8, x, y, 0x2) => self.and_vx_vy(x, y),
// (0x8, x, y, 0x3) => self.xor_vx_vy(x, y),
// (0x8, x, y, 0x4) => self.add_vx_vy(x, y),
// (0x8, x, y, 0x5) => self.sub_vx_vy(x, y),
// (0x8, x, y, 0x6) => self.shr_vx_vy(x, y),
// (0x8, x, y, 0x7) => self.subn_vx_vy(x, y),
// (0x8, x, y, 0xE) => self.shl_vx_vy(x, y),
// (0x9, x, y, 0x0) => self.sne_vx_vy(x, y),
