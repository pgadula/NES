use std::{
    cell::RefCell,
    io::Error,
    path::Path,
    rc::Rc, thread, time::Duration,
};

use m6502::{
    bus::MainBus, cartridge::Cartridge, helpers::{disassembler, hex_dump}, opcodes::{resolve_opcode, AddressingMode}, ppu::PPU
};

fn main() -> Result<(), Error> {
    let c = Rc::new(RefCell::new(Cartridge::load_rom(Path::new(
        "resources/sm.nes",
    ))?));
    let ppu = Rc::new(RefCell::new(PPU::new(c.clone())));
    let mut main_bus = MainBus::new(ppu.clone());
    main_bus.load_cartridge(c.clone());
    let lo = main_bus.read(0xFFFC);
    let hi = main_bus.read(0xFFFD);
    let mut cpu = m6502::cpu::Mos6502::new(main_bus);
    cpu.pc = ((hi as u16) << 8) | (lo as u16);
    let background = c.clone().borrow_mut().backgrounds().to_vec();
    for planes in background.chunks(16) {
        display_sprite(planes);
        println!()
    }
    hex_dump(&c.borrow().prg_rom_data()[0..128]);
    println!();
    let mut running = true;
    let mut line: u64 = 0;
    while running {
        match cpu.fetch() {
            Ok(instr) => {
                // disassembler(&mut cpu, 1);
                println!("[{line}] Fetched {:?} {:?}", instr.0, instr.1);
                cpu.execute(instr);
                // cpu.dump();
                // thread::sleep(Duration::from_millis(50));
            }
            Err(_) => {
                running = false;
            }
        }
        for _ in 0..(3 * 3) {
            ppu.borrow_mut().tick();
        }
        println!("scanline: {}", ppu.borrow().scanline);
        if ppu.borrow().scanline == 241 {
            thread::sleep(Duration::from_millis(50));
            println!("\x1b[41mVBlank start detected\x1b[0m");
        }
        line += 1;
    }
    Ok(())
}



fn display_sprite(planes: &[u8]) {
    for row in 0..8 {
        let plane0 = planes[row];
        let plane1 = planes[row + 8];
        for bit in 0..8 {
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
        0 => "\x1b[40m  ",  // Black (Background)
        1 => "\x1b[44m  ",  // Blue (Sky/Water)
        2 => "\x1b[42m  ",  // Green (Grass/Environment)
        3 => "\x1b[45m  ",  // Magenta (Highlights or UI elements)
        4 => "\x1b[46m  ",  // Cyan (Tech/Metal)
        5 => "\x1b[101m  ", // Light Red (Character accents)
        6 => "\x1b[47m  ",  // Light Grey (Neutral background)
        7 => "\x1b[100m  ", // Dark Grey (Shadow/Depth)
        _ => "\x1b[0m  ",   // Reset
    }
}
