
use crate::chip8;
use crate::memory::Memory;


pub struct Cpu {
    // 16 8-bit general registers
    vx: [u8; 16],

    // 16-bit address register
    i: u16,

    // 16-bit program counter
    pc: u16,
}


impl Cpu {
    pub fn new () -> Cpu {
        Cpu {
            vx: [0; 16],
            i: 0x0,
            pc: chip8::MAP_ROM_BEGIN,
        }
    }

    pub fn execute (&mut self, ram: &mut Memory) {
        let low  = ram.read_byte(self.pc);
        let high = ram.read_byte(self.pc + 1);

        let instruction = (low as u16) << 8 | (high as u16);

        println!("pc = {}, instr = {:#X}", self.pc, instruction);

        self.pc += 2;
    }
}
