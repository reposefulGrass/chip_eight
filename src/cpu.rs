
use crate::chip8;


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

    pub fn execute (&mut self, ram: &mut Vec<u8>) {

    }
}
