mod cpu;

use std::fs::File;
use std::io::prelude::*;

use cpu::CPU;

fn main() {
    println!("Welcome to the Virmic virtual microprocessor.");

    // Read program into 'memory'.
    let mut file = File::open("test").unwrap();
    let mut program: [u8; 15] = [0; 15];
    file.read(&mut program).unwrap();
    println!("{:?}", program);

    let mut cpu = CPU::new(&mut program);

    while cpu.is_running() {
        cpu.cycle();
    }

    println!("{:?}", cpu);
}
