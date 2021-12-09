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
            pc: 0,
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
            (_, _, _, _) => (),
        }
    }
}
