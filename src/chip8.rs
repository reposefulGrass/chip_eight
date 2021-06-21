
use crate::memory::Memory;
use crate::cpu::Cpu;


pub const MAP_ROM_BEGIN: u16 = 0x200;


pub struct Chip8 {
    cpu: Cpu,
    ram: Memory,
}


impl Chip8 {
    pub fn new () -> Chip8 {
        Chip8 {
            cpu: Cpu::new(),
            ram: Memory::new(),
        }
    }

    pub fn load_rom (&mut self, rom: &Vec<u8>) {
        for (i, &byte) in rom.iter().enumerate() {
            let offset = MAP_ROM_BEGIN;
            self.ram.write_byte(offset + i as u16, byte);             
        }
    }

    pub fn step_instruction (&mut self) {
        self.cpu.execute(self.ram);
    }
}
