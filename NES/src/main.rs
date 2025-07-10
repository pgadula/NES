use std::{io::Error, path::Path};

use m6502::cartridge::Cartridge;

fn main() -> Result<(), Error> {
    let c = Cartridge::load_rom(Path::new("resources/sm.nes"))?;
   let background = c.sprites().to_vec();
    for planes in background.chunks(16) {
       display_sprite(planes); 
       println!()
    }
    Ok(())
}

fn display_sprite(planes: &[u8]) {
    for row in 0..8{
            let plane0 = planes[row];
            let plane1 = planes[row + 8];
            for bit in 0..8{
                let hi = plane0 >> (7 - bit) & 1;
                let lo = plane1 >> (7 - bit) & 1;
                let color_index = (hi << 1) | lo;
                print!("{}", color_from_index(color_index));
            }
        println!("{}", color_from_index(0))
       }
}
fn color_from_index(index: u8) -> &'static str {
    match index {
        0 => "\x1b[40m  ",   // Black (Background)
        1 => "\x1b[44m  ",   // Blue (Sky/Water)
        2 => "\x1b[42m  ",   // Green (Grass/Environment)
        3 => "\x1b[45m  ",   // Magenta (Highlights or UI elements)
        4 => "\x1b[46m  ",   // Cyan (Tech/Metal)
        5 => "\x1b[101m  ",  // Light Red (Character accents)
        6 => "\x1b[47m  ",   // Light Grey (Neutral background)
        7 => "\x1b[100m  ",  // Dark Grey (Shadow/Depth)
        _ => "\x1b[0m  ",    // Reset
    }
}