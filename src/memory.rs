
const SIZE_OF_MEMORY: usize = 4096;


pub struct Memory {
    memory: [u8; SIZE_OF_MEMORY]
}


impl Memory {
    pub fn new () -> Memory {
        let mut ram = Memory { memory: [0; SIZE_OF_MEMORY] };

        // An ascii representation using bit-level encoding
        let sprites: [[u8; 5]; 16] = [
            [0xF0, 0x90, 0x90, 0x90, 0xF0], // 0
            [0x20, 0x60, 0x20, 0x20, 0x70], // 1
            [0xF0, 0x10, 0xF0, 0x80, 0xF0], // 2
            [0xF0, 0x10, 0xF0, 0x10, 0xF0], // 3
            [0x90, 0x90, 0xF0, 0x10, 0x10], // 4
            [0xF0, 0x80, 0xF0, 0x10, 0xF0], // 5
            [0xF0, 0x80, 0xF0, 0x90, 0xF0], // 6
            [0xF0, 0x10, 0x20, 0x40, 0x40], // 7
            [0xF0, 0x90, 0xF0, 0x90, 0xF0], // 8
            [0xF0, 0x90, 0xF0, 0x10, 0xF0], // 9
            [0xF0, 0x90, 0xF0, 0x90, 0x90], // A
            [0xE0, 0x90, 0xE0, 0x90, 0xE0], // B
            [0xF0, 0x80, 0x80, 0x80, 0xF0], // C
            [0xE0, 0x80, 0x90, 0x90, 0xE0], // D
            [0xF0, 0x80, 0xF0, 0x80, 0xF0], // E
            [0xF0, 0x80, 0xF0, 0x80, 0x80], // F
        ];

        // Load the sprites in (0x0 - 0x200)
        let offset = 0x0;
        let mut i: u16 = 0;

        for sprite in sprites.iter() {
            for &byte in sprite {
                ram.write_byte(offset + i, byte);
                i += 1;
            }
        }

        ram
    }

    pub fn write_byte (&mut self, address: u16, value: u8) {
        if !self.within_bounds(address) {
            panic!("Address not within RAM boundary.");    
        }

        println!("Writing 0x{:02X} into address 0x{:04X}", value, address);
        self.memory[address as usize] = value;
    }

    pub fn read_byte (&mut self, address: u16) -> u8 {
        if !self.within_bounds(address) {
            panic!("Address not within RAM boundary.");    
        }

        //println!("Reading from address 0x{:04X}", address);
        self.memory[address as usize] 
    }

    fn within_bounds (&self, address: u16) -> bool {
        address <= (SIZE_OF_MEMORY as u16 - 1)
    }
}
