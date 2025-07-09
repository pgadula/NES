use std::{io::Error, path::Path};

use m6502::{cartridge::Cartridge, helpers::hex_dump};

fn main() -> Result<(), Error> {
    let c = Cartridge::load_rom(Path::new("resources/sm.nes"))?;
    // let cartridge = Rc::new(RefCell::new(Cartridge::load_rom(Path::new("resources/sm.nes"))?));
    // let mut bus = MainBus::new();
    // bus.load_cartridge(cartridge.clone());
    // let mut cpu = Mos6502::new(bus);
    println!("{}", c.chr_rom_data().len());
   let background = c.chr_rom_data()[0..4096].to_vec();
    background.chunks(16).enumerate().for_each(|(index, tile)| {
        for row in 0..8 {
           let plane0 = tile[row];
           let plane1 = tile[row+8];
            for bit in 0..8{
                let hi = (plane0 >> (7 - bit)) & 1;
                let lo = (plane1 >> (7 - bit)) & 1;
                let color_index = (hi << 1) | lo;
                print!("{}", color_from_index(color_index));
            }
            println!("\x1b[0m"); 
        }
        println!();
    });

    Ok(())
}
fn color_from_index(index: u8) -> &'static str {
    match index {
        0 => "\x1b[40m  ",  // Transparent (black)
        1 => "\x1b[41m  ",  // Red
        2 => "\x1b[43m  ",  // Brown/yellow
        3 => "\x1b[103m  ", // Tan (bright yellow)
        _ => "\x1b[0m  ",   // Reset
    }
}