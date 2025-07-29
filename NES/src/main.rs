use std::{cell::RefCell, io::Error, path::Path, rc::Rc};

use m6502::{bus::MainBus, cartridge::Cartridge, helpers::hex_dump, ppu::PPU, helpers::ppm};

fn main() -> Result<(), Error> {
    let cartridge: Rc<RefCell<Cartridge>> = Rc::new(RefCell::new(Cartridge::load_rom(Path::new(
        "resources/nestest.nes",
    ))?));
    let ppu = Rc::new(RefCell::new(PPU::new(cartridge.clone())));
    let mut main_bus = MainBus::new(ppu.clone());
    println!("{:?}",cartridge.borrow().mirroring);
    main_bus.load_cartridge(cartridge.clone());
    let lo = main_bus.read(0xFFFC);
    let hi = main_bus.read(0xFFFD);
    let mut cpu = m6502::cpu::Mos6502::new(main_bus);
    cpu.pc = ((hi as u16) << 8) | (lo as u16);
    let background = cartridge.clone().borrow_mut().backgrounds().to_vec();
    for planes in background.chunks(16) {
        display_sprite(planes);
        println!()
    }
    hex_dump(&cartridge.borrow().prg_rom_data()[0..128]);
    println!();
    let mut running = true;
    let mut line: u64 = 0;
    while running {
        match cpu.fetch() {
            Ok(instr) => {
                cpu.execute(instr);
            },
            Err(_) => {
                running = false;
            }
        }
        for _ in 0..(3) {
            let mut nmi_closure = || cpu.nmi();
            ppu.borrow_mut().tick(Some(&mut nmi_closure));
        }
        if ppu.borrow().get_incr() > 32 {
            panic!("VALUE: {}", ppu.borrow().get_incr())
        }
        if ppu.borrow().scanline == 241 {}
        if line == 59999888 {
            running = false;
        }
        line += 1;
    }
    
   // for y in 0..240{
   //     for x in 0..256{
   //        let tile_x = x / 8;
   //        let tile_y = y / 8;
   //        let tile_idx =  
   //        print!("{}.{} ", tile_x, tile_y);
   //        dbg!();
   //     }
   //     println!();
   // }         

    println!("---------------------------");
    let mut framebuffer: [u32; 240*256] = [0; 240*256];
    for y in 0..(240/8){
        for x in 0..(256/8){
           let tile_idx = (x + y * (256/8));
           let addr = ppu.borrow().get_nametable_addr(0x2000 + tile_idx);
           let tile_id = ppu.borrow().vram[addr as usize];
           let offset:usize = ((x * 8) + (y * 8) * 256).into();
           display_sprite_from_idx(tile_id.into(), &background, offset, &mut framebuffer);
           //print!("{} ", tile_id);
        }
        println!();
    }         

    ppm("frame.ppm", 256, 240, framebuffer.to_vec());

    //    print_stable_colored_hex(&ppu.borrow().vram[0..1024 - 64]);
    //    println!();
    //    print_stable_colored_hex(&ppu.borrow().vram[1024..]);
    //    println!();
    //    print_stable_colored_hex(&ppu.borrow().vram[1024 - 64..(1024 - 64) + 64]);
    Ok(())
}

fn display_sprite_from_idx(tile_id: usize, planes: &[u8], offset:usize, framebuffer:&mut [u32; 240*256]) {
    for row in 0..8 {
        let base = tile_id * 16;
        let plane0 = planes[base + row];
        let plane1 = planes[base + row + 8] ;

        for bit in 0..8 {
            let hi = plane0 >> (7 - bit) & 1;
            let lo = plane1 >> (7 - bit) & 1;
            let color_index = (hi << 1) | lo;
            framebuffer[offset + row * 256 + bit] = color_hex_value_from_index(color_index as usize);
        }
    }
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

fn color_hex_value_from_index(index: usize) -> u32 {
    match index {
        0 => 0x000000, // Black
        1 => 0x0000FF, // Blue
        2 => 0x00FF00, // Green
        3 => 0xFF00FF, // Magenta
        4 => 0x00FFFF, // Cyan
        5 => 0xFF5555, // Light Red
        6 => 0xCCCCCC, // Light Grey
        7 => 0x333333, // Dark Grey
        _ => 0x000000, // Default Black
    }
}

fn print_stable_colored_hex(data: &[u8]) {
    let mut colors: Vec<String> = Vec::new();

    // Generate 128 colors (0–127)
    for i in 0..128 {
        colors.push(format!("\x1b[38;5;{}m", i));
    }
    let reset = "\x1b[0m";

    for (i, &byte) in data.iter().enumerate() {
        // Map byte value to a color index based on the byte itself (stable)
        let color_index = (byte as usize) % colors.len();
        let color = colors[color_index].clone();

        print!("{}{:02X}{}", color, byte, reset);

        if (i + 1) % 32 == 0 {
            println!();
        } else {
            print!(".");
        }
    }
    println!();
}
