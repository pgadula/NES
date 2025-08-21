use std::fs::File;
use std::io::{stdout, Write};
use std::{cell::RefCell, io::Error, path::Path, rc::Rc};

use nes_core::{bus::MainBus, cartridge::Cartridge, helpers::hex_dump, helpers::ppm, ppu::PPU};

fn load_pallete(file_path: &str) -> Result<[u32; 64], Error> {
    let file: Vec<u8> = std::fs::read(file_path)?;
    let mut palette: [u32; 64] = [0; 64];
    hex_dump(&file);
    for (idx, triple) in file.chunks(3).enumerate() {
        if idx == 64 {
            break;
        }
        let mut color: u32 = 0;

        color |= (triple[0] as u32) << 16;
        color |= (triple[1] as u32) << 8;
        color |= (triple[2] as u32) << 0;
        println!("{idx}");
        print!("{:06x} ", color);
        palette[idx as usize] = color;
    }
    Ok(palette)
}

fn main() -> Result<(), Error> {
    let cartridge: Rc<RefCell<Cartridge>> = Rc::new(RefCell::new(Cartridge::load_rom(Path::new(
        "resources/bf.nes",
    ))?));
    // let nes_palette = load_pallete("resources/ntscpalette.pal").unwrap();
    let ppu = Rc::new(RefCell::new(PPU::new(cartridge.clone())));
    let mut main_bus = Rc::new(RefCell::new(MainBus::new(ppu.clone())));
    println!("{:?}", cartridge.borrow().mirroring);
    main_bus.borrow_mut().load_cartridge(cartridge.clone());
    let lo = main_bus.borrow_mut().read(0xFFFC);
    let hi = main_bus.borrow_mut().read(0xFFFD);
    let mut cpu = nes_core::cpu::Mos6502::new(main_bus);
    cpu.pc = ((hi as u16) << 8) | (lo as u16);

    let mut running = true;
    let mut line: u64 = 0;
    while running {
        match cpu.fetch() {
            Ok(instr) => {
                cpu.execute(instr);
            }
            Err(_) => {
                running = false;
            }
        }
        for _ in 0..3 {
            let mut nmi_closure = || cpu.nmi();
            ppu.borrow_mut().tick(Some(&mut nmi_closure));
        }

        if line % 200_000 == 0 {
            render_terminal_scaled(&ppu, 1, 1)?;
        }
        line += 1;
    }

    // println!("Program stopped!");

    // let palette = ppu.borrow().palette;
    // hex_dump(&palette);

    // let mut palette_buffer: [u32; 32 * 32] = [0; 32 * 32];
    // for y in 0..32 {
    //     for x in 0..32 {
    //         let idx = y * 32 + x;
    //         let color_idx = palette[idx % 32];
    //         // let hex = nes_palette[color_idx as usize % 64];
    //         palette_buffer[idx] = hex;
    //         if y == 0 {
    //             print!("{:02} ", color_idx);
    //         }
    //     }
    // }

    // println!("------------------------------------\n");
    // println!("---------generating palette---------\n");
    // ppm("palette.ppm", 32, 32, &palette_buffer.to_vec());

    // println!("----------------------------------------");
    // println!("---------generating framebuffer---------");
    // let mut framebuffer: [u32; 240 * 256] = [0; 240 * 256];
    // let pu_ctrl = ppu.borrow().ppu_crtl;
    // let pattern_base = if ppu_ctrl & 0b0001_0000 != 0 {
    // 0x1000
    // } else {
    // 0x0000
    // };

    // for y in 0..(240 / 8) {
    //     for x in 0..(256 / 8) {
    //         let nametable_idx = y * (256 / 8) + x;
    //         let vram_addr = ppu.borrow().get_nametable_addr(nametable_idx + 0x2000) as usize;
    //         let tile_id = ppu.borrow().vram[vram_addr] as usize;

    //         let c = cartridge.borrow();
    //         let pattern_addr: usize = pattern_base + tile_id * 16;

    //         let tile = &c.chr_rom_data()[pattern_addr..pattern_addr + 16];
    //         let offset = (x * 8) + (y * 8 * 256);

    //         render_sprite(
    //             &tile,
    //             offset as usize,
    //             &mut framebuffer,
    //             &ppu.borrow(),
    //             &nes_palette,
    //         );
    //     }
    // }

    // for y in 0..(240 / 8) {
    //     for x in 0..(256 / 8) {
    //         let nametable_idx = y * (256 / 8) + x;
    //         let vram_addr = ppu.borrow().get_nametable_addr(nametable_idx + 0x2000) as usize;
    //         let tile_id = ppu.borrow().get_nametable(0)[(nametable_idx) as usize] as usize;
    //         let c = cartridge.borrow();
    //         let tile = &c.chr_rom_data()[tile_id..tile_id + 16];
    //         let offset = (x * 8) + (y * 8 * 256);
    //         let color_code = ansi_color((tile_id) as u8);
    //         print!("{}{:02x}.{}", color_code, tile_id, reset_color());
    //     }
    //     println!();
    // }

    // ppu.borrow().dump();
    ppu.borrow_mut().render_background();
    ppu.borrow_mut().render_sprite();
    ppm(
        "frame-f-ppu.ppm",
        256,
        240,
        &ppu.borrow().framebuffer.to_vec(),
    );

    // ppm("frame2.ppm", 256, 240, &framebuffer.to_vec());
    Ok(())
}
fn render_sprite(
    planes: &[u8],
    offset: usize,
    framebuffer: &mut [u32; 240 * 256],
    ppu: &PPU,
    nes_palette: &[u32; 64],
) {
    for row in 0..8 {
        let plane0 = planes[row];
        let plane1 = planes[row + 8];

        for bit in 0..8 {
            let hi = plane0 >> (7 - bit) & 1;
            let lo = plane1 >> (7 - bit) & 1;
            let palette_index = (hi << 1) | lo;
            let color_index = ppu.palette[palette_index as usize];

            let fb = (row * 256) + offset + bit;
            framebuffer[fb] = nes_palette[color_index as usize % 64];
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
        0 => "\x1b[40m",  // Black (Background)
        1 => "\x1b[44m",  // Blue (Sky/Water)
        2 => "\x1b[42m",  // Green (Grass/Environment)
        3 => "\x1b[45m",  // Magenta (Highlights or UI elements)
        4 => "\x1b[46m",  // Cyan (Tech/Metal)
        5 => "\x1b[101m", // Light Red (Character accents)
        6 => "\x1b[47m",  // Light Grey (Neutral background)
        7 => "\x1b[100m", // Dark Grey (Shadow/Depth)
        _ => "\x1b[0m",   // Reset
    }
}

fn color_hex_value_from_index(index: usize) -> u32 {
    match index {
        0 => 0x545454, // Dark gray (background-like)
        1 => 0x0018D8, // Deep blue
        2 => 0x38A800, // NES green
        3 => 0xD82800, // Bright red
        4 => 0xFC9838, // Orange
        5 => 0xFCFCFC, // White
        6 => 0x7C7C7C, // Medium gray
        7 => 0xB8B8F8, // Light blue
        _ => 0x000000, // Fallback
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

pub fn render_terminal_scaled(
    ppu: &Rc<RefCell<PPU>>,
    sx: usize,
    sy: usize,
) -> std::io::Result<()> {
    //[TODO] It would be cool to change this to double-buffered version. I think that will eliminate the problem with flickering of screen.
    const W: usize = 256;
    const H: usize = 240;

    {
        let mut p = ppu.borrow_mut();
        p.render_background();
        p.render_sprite();
    }

    let fb = ppu.borrow();
    let fb: &[u32] = &fb.framebuffer;

    let mut out = std::io::stdout().lock();
    write!(out, "\x1b[?25l\x1b[2J\x1b[H")?;

    for y in (0..H).step_by(2 * sy) {
        let y2 = (y + sy).min(H - 1);
        for x in (0..W).step_by(sx) {
            let top = fb[y * W + x];
            let bot = fb[y2 * W + x];

            let (rt, gt, bt) = (((top >> 16) & 0xFF) as u8,
                                ((top >>  8) & 0xFF) as u8,
                                ((top >>  0) & 0xFF) as u8);
            let (rb, gb, bb) = (((bot >> 16) & 0xFF) as u8,
                                ((bot >>  8) & 0xFF) as u8,
                                ((bot >>  0) & 0xFF) as u8);

            write!(out, "\x1b[38;2;{rt};{gt};{bt}m\x1b[48;2;{rb};{gb};{bb}m▀")?;
        }
        write!(out, "\x1b[0m\n")?;
    }

    write!(out, "\x1b[0m\x1b[?25h")?;
    out.flush()
}