use std::{cell::RefCell, rc::Rc};

use crate::cartridge::{Cartridge, Mirroring};
const FALLBACK_PALETTE: [u32; 64] = [
    0xFF757575, 0xFF271B8F, 0xFF0000AB, 0xFF47009F, 0xFF8F0077, 0xFFA7004F, 0xFFA70000, 0xFF7F0B00,
    0xFF432F00, 0xFF004700, 0xFF005100, 0xFF003F17, 0xFF1B3F5F, 0xFF000000, 0xFF000000, 0xFF000000,
    0xFFBCBCBC, 0xFF0073EF, 0xFF233BEF, 0xFF8300F3, 0xFFBF00BF, 0xFFE7005B, 0xFFDB2B00, 0xFFCB4F0F,
    0xFF8B7300, 0xFF009700, 0xFF00AB00, 0xFF00933B, 0xFF00838B, 0xFF000000, 0xFF000000, 0xFF000000,
    0xFFFFFFFF, 0xFF3FBFFF, 0xFF5F73FF, 0xFFA78BFD, 0xFFF77BFF, 0xFFFF77B7, 0xFFFF7763, 0xFFFF9B3B,
    0xFFF3BF3F, 0xFF83D313, 0xFF4FDF4B, 0xFF58F898, 0xFF00EBDB, 0xFF757575, 0xFF000000, 0xFF000000,
    0xFFFFFFFF, 0xFFABE7FF, 0xFFC7D7FF, 0xFFD7CBFF, 0xFFFFC7FF, 0xFFFFC7DB, 0xFFFFBFB3, 0xFFFFDBAB,
    0xFFFFE7A3, 0xFFE3FFA3, 0xFFABF3BF, 0xFFB3FFCF, 0xFF9FFFF3, 0xFFD7D7D7, 0xFF000000, 0xFF000000,
];

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
    pub internal_palette: [u32; 64],
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

            internal_palette: FALLBACK_PALETTE,
        };
    }

    pub fn set_palette(&mut self, palette: &[u32; 64]) {
        self.internal_palette = palette.clone();
    }

    pub fn render(&mut self) {
        let pattern_base = if self.ppu_crtl & 0b0001_0000 != 0 {
            0x1000
        } else {
            0x0000
        };

        for y in 0..(240 / 8) {
            for x in 0..(256 / 8) {
                let nametable_idx = y * (256 / 8) + x;
                let vram_addr = self.get_nametable_addr(nametable_idx + 0x2000) as usize;
                let tile_id = self.vram[vram_addr] as usize;

                let pattern_addr: usize = pattern_base + tile_id * 16;

                let tile: [u8; 16] = {
                    let c = self.cartridge.borrow();
                    let data = c.chr_rom_data();
                    let mut buf = [0u8; 16];
                    buf.copy_from_slice(&data[pattern_addr..pattern_addr + 16]);
                    buf
                };
                let offset = (x * 8) + (y * 8 * 256);
                self.render_sprite(&tile, offset as usize);
            }
        }
    }

    fn render_sprite(&mut self, planes: &[u8], offset: usize) {
        for row in 0..8 {
            let plane0 = planes[row];
            let plane1 = planes[row + 8];

            for bit in 0..8 {
                let hi = plane0 >> (7 - bit) & 1;
                let lo = plane1 >> (7 - bit) & 1;
                let palette_index = (hi << 1) | lo;
                let color_index = self.palette[palette_index as usize];

                let fb = (row * 256) + offset + bit;
                self.framebuffer[fb] = self.internal_palette[color_index as usize % 64];
            }
        }
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
        let mirroring = &self.cartridge.borrow().mirroring;
        match mirroring {
            Mirroring::Horizontal => {
                match addr {
                    0x2000..=0x23FF => addr - 0x2000, // NT0
                    0x2400..=0x27FF => addr - 0x2400, // NT1
                    0x2800..=0x2BFF => addr - 0x2000, // mirror of NT0
                    0x2C00..=0x2FFF => addr - 0x2400, // mirror of NT1
                    _ => panic!("Invalid range {addr}"),
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
            0x2007 => Some(self.ppu_data),
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
}
