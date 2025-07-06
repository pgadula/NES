use std::{cell::RefCell, io::Error, path::Path, rc::Rc};

use m6502::{cartridge::Cartridge, cpu::{MainBus, Mos6502}, helpers::hex_dump};

fn main()->Result<(), Error> {
    let cartridge = Rc::new(RefCell::new(Cartridge::load_rom(Path::new("/Users/pgadula/Programming/NES/m6502/resources/sm.nes"))?));
    let mut bus = MainBus::new();
    bus.load_cartridge(cartridge.clone());
    let mut cpu = Mos6502::new(bus);
    
    Ok(())
}
