type OpCode = u16;

use crate::graphics;

// short is actually just 4 bits.
type short = u8;

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
    let mut compy = Compy::new();
    println!("hi");

    loop {
        // Emulate one cycle
        compy.step();

        // If the draw flag is set, update the screen
        if compy.draw_flag {
            // drawGraphics();
        }

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
    pub fn step(&mut self) {
        let pc = self.pc as usize;
        // Fetch Opcode
        let opcode: OpCode = ((self.memory[pc] as u16) << 8) | (self.memory[pc + 1] as u16);
        self.run_op(opcode);
        // Decode Opcode
        // Execute Opcode

        // Update timers
        ()
    }
    fn clear_display(&mut self) {
        self.gfx.clear();
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
            self.step();
            self.step();
        } else {
            self.step();
        }
    }

    fn subroutine(&mut self, addr: u16) {
        self.stack[self.sp] = self.pc;
        self.pc = addr;
    }

    pub fn run_op(&mut self, opcode: OpCode) {
        let shorts: (short, short, short, short) = (
            ((opcode & 0xF000) >> 12) as short,
            ((opcode & 0x0F00) >> 8) as short,
            ((opcode & 0x00F0) >> 4) as short,
            (opcode & 0x000F) as short,
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
                self.step();
            }
            // Vx += NN
            (0x7, x, n1, n2) => {
                self.reg[x as usize] += val(n1, n2);
                self.step();
            }
            // Vx = Vy
            (0x8, x, y, 0x0) => {
                self.reg[x as usize] = self.reg[y as usize];
                self.step();
            }
            // Bitwise OR, Vx |= Vy
            (0x8, x, y, 0x1) => {
                self.reg[x as usize] |= self.reg[y as usize];
                self.step();
            }
            // Bitwise AND, Vx &= Vy
            (0x8, x, y, 0x2) => {
                self.reg[x as usize] &= self.reg[y as usize];
                self.step();
            }
            // Bitwise XOR, Vx ^= Vy
            (0x8, x, y, 0x3) => {
                self.reg[x as usize] ^= self.reg[y as usize];
                self.step();
            }
            // Vx += Vy
            (0x8, x, y, 0x4) => {
                self.reg[x as usize] += self.reg[y as usize];
                self.step();
            }
            // Vx -= Vy
            (0x8, x, y, 0x5) => {
                self.reg[x as usize] -= self.reg[y as usize];
                self.step();
            }
            // Vx >>= 1
            (0x8, x, y, 0x6) => {
                self.reg[x as usize] >>= self.reg[y as usize];
                self.step();
            }
            // Vx = Vy - Vx
            (0x8, x, y, 0x7) => {
                self.reg[x as usize] = self.reg[y as usize] - self.reg[x as usize];
                self.step();
            }
            // Vx <<= 1
            (0x8, x, y, 0xE) => {
                self.reg[x as usize] <<= self.reg[y as usize];
                self.step();
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
            // TODO: impl
            (0xD, x, y, n) => (),
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

pub fn addr(n1: short, n2: short, n3: short) -> u16 {
    ((n1 as u16) << 8) & ((n2 as u16) << 4) & (n3 as u16)
}

pub fn val(n1: short, n2: short) -> u8 {
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
