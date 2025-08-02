use std::{cell::RefCell, rc::Rc};

use crate::cartridge::{Cartridge, Mirroring};

#[derive(Debug)]
pub struct PPU {
    pub cartridge: Rc<RefCell<Cartridge>>,
    pub vram: [u8; 2048],
    pub palette: [u8; 32],

    //registers
    pub ppu_crtl: u8,
    pub ppu_mask: u8,
    pub ppu_status: u8,
    pub oam_addr: u8,
    pub oam_data: u8,
    pub ppu_scroll: u8,
    pub ppu_addr: u16,
    pub ppu_data: u8,
    pub oam_dma: u8,

    cycle: u16,
    pub scanline: u16,

    w: bool, //latch
    t: u16,  //temporary address
    v: u16,  //internal register for vram addressing

    pub framebuffer: [u32; 256 * 240],
}

impl PPU {
    pub fn new(cartridge: Rc<RefCell<Cartridge>>) -> PPU {
        return PPU {
            cartridge,
            vram: [0; 2048],
            palette: [0; 32],
            oam_addr: 0,
            oam_data: 0,
            oam_dma: 0,
            ppu_addr: 0,
            ppu_crtl: 0,
            ppu_data: 0,
            ppu_mask: 0,
            ppu_scroll: 0,
            ppu_status: 0,

            cycle: 0,
            scanline: 0,
            w: false,
            t: 0,
            v: 0,
            framebuffer: [0; 256 * 240],
        };
    }
    pub fn read_chr_rom(&self, address: u16) -> u8 {
        self.cartridge.borrow().chr_rom_data()[address as usize]
    }

    pub fn get_incr(&self) -> u16 {
        return match (self.ppu_crtl & 0b0000_0100) != 0 {
            true => 32,
            false => 1,
        };
    }

    pub fn get_nametable_addr(&self, addr: u16) -> u16 {
        let mirroring = self.cartridge.borrow().mirroring;
        match mirroring {
            Mirroring::Horizontal => {
                match addr {
                    0x2000..=0x23FF => addr - 0x2000, // NT0
                    0x2400..=0x27FF => addr - 0x2400, // NT1
                    0x2800..=0x2BFF => addr - 0x2000, // mirror of NT0
                    0x2C00..=0x2FFF => addr - 0x2400, // mirror of NT1
                    _ => panic!("Invalid range {addr}")
                }
            }
            Mirroring::Vertical => {
                match addr {
                    0x2000..=0x23FF => addr - 0x2000, // NT0
                    0x2400..=0x27FF => addr - 0x2400, // NT1
                    0x2800..=0x2BFF => addr - 0x2800, // NT0 again
                    0x2C00..=0x2FFF => addr - 0x2C00, // NT1 again
                    _ => panic!("Invalid range {addr}"),
                }
            }
        }
    }

    pub fn get_nametable(&self, n: u8) -> &[u8] {
        match n {
            0 => &self.vram[0x000..0x3C0], // first nametable (960 bytes)
            1 => &self.vram[0x400..0x7C0], // second nametable (960 bytes)
            _ => panic!("Invalid nametable num {}", n),
        }
    }

    pub fn cpu_read(&mut self, address: u16) -> Option<u8> {
        let addr = match address {
            0x2000..=0x3FFF => 0x2000 + (address % 8),
            _ => address,
        };
        match addr {
            0x2000..=0x2001 => {
                //eprintln!("Cannot read from addr {:04x}", addr);
                None
            }
            0x2002 => {
                self.w = false;
                let value = self.ppu_status;
                self.ppu_status &= !0x80;
                Some(value)
            }
            0x2003 => {
                //eprintln!("Cannot read from addr {:04x}", addr);
                None
            }
            0x2004 => Some(self.oam_data),
            0x2005 => {
                //   eprintln!("Cannot read from addr {:04x}", addr);
                None
            }
            0x2006 => {
                //    eprintln!("Cannot read from addr {:04x}", addr);
                None
            }
            0x2007 => {
                Some(self.ppu_data)
            },
            0x4014 => {
                eprintln!("Cannot read from addr {:04x}", addr);
                None
            }
            _ => {
                eprintln!("[Error] addr:{:04x} out of boundary.", addr);
                None
            }
        }
    }

    pub fn cpu_write(&mut self, address: u16, value: u8) {
        let addr = match address {
            0x2000..=0x3FFF => 0x2000 + (address & 0x0007),
            _ => address,
        };
        match addr {
            0x2000 => self.ppu_crtl = value,
            0x2001 => self.ppu_mask = value,
            0x2002 => {
                //eprintln!("[Error] cannot write to addr 0x2002")
            }
            0x2003 => self.oam_addr = value,
            0x2004 => self.oam_data = value,
            0x2005 => self.ppu_scroll = value,
            0x2006 => {
                if self.w == false {
                    self.t = 0;
                    self.t = ((value as u16) & 0x3F) << 8;
                    self.w = true;
                } else {
                    self.t |= value as u16;
                    self.v = self.t & 0x3FFF;
                    self.w = false;
                }
            }
            0x2007 => {
                let addr = self.v;
                if addr >= 0x2000 && addr <= 0x3EFF {
                    let mapped_addr = self.get_nametable_addr(addr);
                    self.vram[mapped_addr as usize] = value;
                } else if addr >= 0x3F00 && addr <= 0x3FFF {
                    self.palette[(addr % 32) as usize] = value;
                }

                self.v = self.v.wrapping_add(self.get_incr());
            }
            0x4014 => self.oam_dma = value,
            _ => {
                eprintln!("[Error] addr:{:04x} out of boundary.", addr);
            }
        }
    }

    pub fn tick(&mut self, nmi: Option<&mut dyn FnMut() -> ()>) {
        let _ = nmi;
        self.cycle += 1;
        if self.cycle >= 341 {
            self.cycle = 0;
            self.scanline += 1;

            if self.scanline == 241 {
                if self.ppu_crtl & 0b1000_0000 != 0 {
                    if let Some(nmi_fn) = nmi {
                        nmi_fn();
                    }
                }
                self.ppu_status |= 0x80
            }
            if self.scanline >= 262 {

                self.scanline = 0;
                self.ppu_status &= !0x80
            }
        }
    }
    pub fn dump(&self) {
        println!("PPU State Dump:");
        println!("  Cycle: {}", self.cycle);
        println!("  Scanline: {}", self.scanline);
        println!("  PPUCTRL:  0x{:02X}", self.ppu_crtl);
        println!("  PPUMASK:  0x{:02X}", self.ppu_mask);
        println!("  PPUSTATUS:0x{:02X}", self.ppu_status);
        println!("  OAMADDR:  0x{:02X}", self.oam_addr);
        println!("  OAMDATA:  0x{:02X}", self.oam_data);
        println!("  PPUSCROLL:0x{:02X}", self.ppu_scroll);
        println!("  PPUADDR:  0x{:02X}", self.v);
        println!("  PPUDATA:  0x{:02X}", self.ppu_data);
        println!("  OAMDMA:   0x{:02X}", self.oam_dma);

        // Optionally dump a small part of VRAM:
        println!("  VRAM[0x2000..0x200F]: {:?}", &self.vram[0x0000..0x0010]);

        // Palette sample:
        println!("  Palette[0..8]: {:?}", &self.palette[0..8]);
    }

    pub fn generate_framebuffer(&mut self, nes_palette_color: &[u32; 64]) {
    for tile_y in 0..30 {
        for tile_x in 0..32 {
            // Compute nametable address (first nametable, 0x2000–0x23BF)
            let nametable_index = 0x2000 + tile_y * 32 + tile_x;
            // Get tile ID directly from nametable
            let tile_id = self.get_nametable_addr(nametable_index);
            // Pattern table address (16 bytes per tile)
            let pattern_addr = (tile_id as u16) * 16;

            // Attribute table lookup (0x23C0–0x23FF for first nametable)
            let attr_index = 0x23C0 + (tile_y / 4) * 8 + (tile_x / 4);
            let attr_addr = self.get_nametable_addr(attr_index);
            let attr_byte = self.vram[attr_addr as usize];

            // Determine which 2x2 tile quadrant we're in (0-3)
            let quadrant = ((tile_y % 4) / 2) * 2 + ((tile_x % 4) / 2);

            // Extract palette bits (2 bits for this quadrant)
            let palette_bits = match quadrant {
                0 => attr_byte & 0b00000011,
                1 => (attr_byte >> 2) & 0b00000011,
                2 => (attr_byte >> 4) & 0b00000011,
                3 => (attr_byte >> 6) & 0b00000011,
                _ => 0,
            };

            // Each palette is 4 colors, skip universal background color
            let palette_base = 1 + (palette_bits * 4);

            // Draw 8x8 tile
            for row in 0..8 {
                let lo = self.read_chr_rom(pattern_addr + row);
                let hi = self.read_chr_rom(pattern_addr + row + 8);

                for col in 0..8 {
                    let bit = 7 - col;
                    let pixel = ((lo >> bit) & 1) | (((hi >> bit) & 1) << 1);

                    // Pixel 0 is always universal background color
                    let palette_idx = if pixel == 0 {
                        self.palette[0]
                    } else {
                        self.palette[(palette_base + pixel) as usize]
                    };

                    let screen_x = tile_x * 8 + col;
                    let screen_y = tile_y * 8 + row;

                    if screen_x < 256 && screen_y < 240 {
                        self.framebuffer[(screen_y * 256 + screen_x) as usize] =
                            nes_palette_color[palette_idx as usize % 64];
                    }
                }
            }
        }
    }
}

  }
