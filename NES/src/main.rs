use std::{io::Error, path::Path};

use m6502::{cartridge::Cartridge, cpu::{Bus, Mos6502}};


fn main()->Result<(), Error> {
    let bus = Bus::new();
    let mut cpu = Mos6502::new(bus);
    // adc(&mut cpu);
    // println!("{}", cpu);
    // println!("{}", cpu.bus);
    // cpu.pc.to_le_bytes()
    // cpu.reset();

    // println!("{}", cpu.bus)  ;
    let c = Cartridge::load_rom(Path::new("/Users/pgadula/Programming/m6502/m6502/resources/nestest.nes"))?;

    Ok(())
}
