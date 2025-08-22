mod gb;
use crate::gb::Gameboy;

use std::env;
use std::fs::read;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run path/to/rom");
        return;
    }

    let mut gb = Gameboy::new();

    let buffer: Vec<u8> = read(&args[1]).expect("Unable to open file");
    gb.load_rom(&buffer);

    for _x in 1..12 {
        gb.tick();
    }
}
