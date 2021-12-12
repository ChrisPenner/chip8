type OpCode = u16;

// https://multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/

struct Compy {
    memory: [u8; 4096],
    regsters: [u8; 16],
    i: u16,
    pc: u16,
    gfx: [bool; 64 * 32],
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; 16],
    sp: u16,
    key: [bool; 16],

    draw_flag: bool,
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
    fn new() -> Compy {
        return Compy {
            memory: [0; 4096],
            regsters: [0; 16],
            i: 0,
            pc: 0x200,
            gfx: [false; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            key: [false; 16],
            draw_flag: false,
        };
    }
    fn step(&mut self) {
        let pc = self.pc as usize;
        // Fetch Opcode
        let opcode: OpCode = ((self.memory[pc] as u16) << 8) | (self.memory[pc + 1] as u16);
        self.run_op(opcode);
        // Decode Opcode
        // Execute Opcode

        // Update timers
        ()
    }

    fn run_op(&mut self, opcode: OpCode) {
        let shorts: (u8, u8, u8, u8) = (
            ((opcode & 0xF000) >> 12) as u8,
            ((opcode & 0x0F00) >> 8) as u8,
            ((opcode & 0x00F0) >> 4) as u8,
            (opcode & 0x000F) as u8,
        );
        match shorts {
            // Call machine code (optional)
            (0x0, x, y, z) => (),
            // clear display
            (0x0, 0x0, 0xE, 0x0) => (),
            // RETURN
            (0x0, 0x0, 0xE, 0xE) => (),
            // GOTO NNN
            (0x1, n1, n2, n3) => (),
            // Call Subroutine at NNN
            (0x2, n1, n2, n3) => (),
            // Skip when (Vx == NN)
            (0x3, x, n1, n2) => (),
            // Skip when (Vx != NN)
            (0x4, x, n1, n2) => (),
            // Skip when (Vx == Vy)
            (0x5, x, y, 0x0) => (),
            // Vx = NN
            (0x6, x, n1, n2) => (),
            // Vx += NN
            (0x7, x, n1, n2) => (),
            // Vx = Vy
            (0x8, x, y, 0x0) => (),
            // Bitwise OR, Vx |= Vy
            (0x8, x, y, 0x1) => (),
            // Bitwise AND, Vx &= Vy
            (0x8, x, y, 0x2) => (),
            // Bitwise XOR, Vx ^= Vy
            (0x8, x, y, 0x3) => (),
            // Vx += Vy
            (0x8, x, y, 0x4) => (),
            // Vx -= Vy
            (0x8, x, y, 0x5) => (),
            // Vx >>= 1
            (0x8, x, y, 0x6) => (),
            // Vx = Vy - Vx
            (0x8, x, y, 0x7) => (),
            // Vx <<= 1
            (0x8, x, y, 0xE) => (),
            // Skip when Vx != Vy
            (0x8, x, y, 0xE) => (),
            // I = NNN
            (0xA, n1, n2, n3) => (),
            // PC = V0 + NNN
            (0xB, n1, n2, n3) => (),
            // draw_sprite(Vx, Vy, N)
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
