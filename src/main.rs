mod spu;
mod rom;
mod ram;

#[cfg(test)]
mod tests;

use spu::SPU;
use rom::Rom;
use ram::Ram;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2{
        println!("Usage: {} <rom_file>", args[0]);
        return;
    }
    let mut rom: Rom = Rom::new();
    rom.load_from_file(&args[1]);
    let ram: Ram = Ram::new();
    let mut spu: SPU = SPU::new(rom, ram);
    spu.execute(None);
}
