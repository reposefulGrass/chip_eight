
#![allow(unused_variables)]

mod memory;
mod cpu;

use std::fs;
use cpu::Cpu;


fn main () {
    let data = fs::read("roms/games/Space Invaders [David Winter].ch8")
        .unwrap();

    let mut cpu = Cpu::new();
    cpu.load_rom(&data);
}

