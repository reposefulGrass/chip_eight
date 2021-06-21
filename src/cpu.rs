
use crate::memory::Memory;


const MAP_ROM_BEGIN: u16 = 0x200;


pub struct Cpu {
    ram: Memory,
}


impl Cpu {
    pub fn new () -> Cpu {
        Cpu {
            ram: Memory::new()
        }
    }

    pub fn load_rom (&mut self, rom: &Vec<u8>) {
        for (i, &byte) in rom.iter().enumerate() {
            let offset = MAP_ROM_BEGIN;
            self.ram.write_byte(offset + i as u16, byte);             
        }
    }
}
