
const SIZE_OF_MEMORY: usize = 4096;


pub struct Memory {
    memory: [u8; SIZE_OF_MEMORY]
}


impl Memory {
    pub fn new () -> Memory {
        Memory {
            memory: [0; SIZE_OF_MEMORY],
        }
    }

    pub fn write_byte (&mut self, address: u16, value: u8) {
        //println!("Writing 0x{:02X} into address 0x{:04X}", value, address);
        self.memory[address as usize] = value;
    }

    pub fn read_byte (&mut self, address: u16) -> u8 {
        //println!("Reading from address 0x{:04X}", address);
        self.memory[address as usize] 
    }
}
