
#![allow(unused_variables)]
#![allow(dead_code)]

mod memory;
mod cpu;
mod chip8;
mod instruction;
mod types;

use std::fs;
use chip8::Chip8;


fn main () {
    let data = fs::read("../roms/games/Space Invaders [David Winter].ch8")
        .unwrap();

    let mut chip8 = Chip8::new();
    chip8.load_rom(&data);

    loop {
        chip8.step_instruction();
    }
}

