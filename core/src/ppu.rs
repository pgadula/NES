use crate::cartridge::{Cartridge, Mirroring};
use bitflags::bitflags;
use std::{cell::RefCell, rc::Rc};
const FALLBACK_PALETTE: [u32; 64] = [
    0x545454FF, 0x001E74FF, 0x081090FF, 0x300088FF, 0x440064FF, 0x5C0030FF, 0x540400FF, 0x3C1800FF,
    0x202A00FF, 0x083A00FF, 0x004000FF, 0x003C00FF, 0x00323CFF, 0x000000FF, 0x000000FF, 0x000000FF,
    0x989698FF, 0x084CCCFF, 0x3032ECFF, 0x5C1EE4FF, 0x8814B0FF, 0xA01464FF, 0x981F20FF, 0x783C00FF,
    0x545A00FF, 0x287200FF, 0x087C00FF, 0x007628FF, 0x006684FF, 0x000000FF, 0x000000FF, 0x000000FF,
    0xECEEECFF, 0x4C9AEFFF, 0x787CECFF, 0xB062ECFF, 0xE454ECFF, 0xEC58B4FF, 0xEC6A64FF, 0xD48820FF,
    0xA0AA00FF, 0x74C400FF, 0x4CD020FF, 0x38CC6CFF, 0x38B4CCFF, 0x3C3C3CFF, 0x000000FF, 0x000000FF,
    0xECEEECFF, 0xA8CCECFF, 0xBCBCECFF, 0xD4B2ECFF, 0xECAFECFF, 0xECAFD4FF, 0xECB4B0FF, 0xE4C490FF,
    0xCCD278FF, 0xB4DE78FF, 0xA8E290FF, 0x98E2B4FF, 0xA0D6E4FF, 0xA0A2A0FF, 0x000000FF, 0x000000FF,
];

#[derive(Debug)]
pub struct PPU {
    pub cartridge: Rc<RefCell<Cartridge>>,
    pub vram: [u8; 2048],
    pub palette: [u8; 32],
    pub oam: [u8; 4 * 64],

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
    temp_value: u8,

    pub framebuffer: [u32; 256 * 240],
    pub internal_palette: [u32; 64],
}

impl PPU {
    pub fn new(cartridge: Rc<RefCell<Cartridge>>) -> PPU {
        return PPU {
            cartridge,
            vram: [0; 2048],
            palette: [0; 32],
            oam: [0; 4 * 64],
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
            temp_value: 0,
            framebuffer: [0; 256 * 240],

            internal_palette: FALLBACK_PALETTE,
        };
    }

    pub fn get_tile_palette(&self, tile_x: u8, tile_y: u8, nametable: u8) -> u8 {
        let attr_table = self.get_attribute(nametable);
        let attr_x = tile_x / 4;
        let attr_y = tile_y / 4;
        let attr_index = (attr_y as usize) * 8 + attr_x as usize; // 8 attribute bytes per row of 32 tiles
        let attr_byte = attr_table[attr_index];

        // Determine which quadrant in the attribute byte
        let quadrant_x = (tile_x % 4) / 2;
        let quadrant_y = (tile_y % 4) / 2;
        let shift = match (quadrant_x, quadrant_y) {
            (0, 0) => 0, // top-left
            (1, 0) => 2, // top-right
            (0, 1) => 4, // bottom-left
            (1, 1) => 6, // bottom-right
            _ => unreachable!(),
        };

        (attr_byte >> shift) & 0b11
    }

    pub fn set_palette(&mut self, palette: &[u32; 64]) {
        self.internal_palette = palette.clone();
    }

    pub fn render_background(&mut self) {
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
                let offset: usize = ((x * 8) + (y * 8 * 256)) as usize;
                let palette_num = self.get_tile_palette(x as u8, y as u8, 0);
                let palette_base = (palette_num * 4) as usize;
                for row in 0..8 {
                    let plane0 = tile[row];
                    let plane1 = tile[row + 8];

                    for bit in 0..8 {
                        let hi = plane0 >> (7 - bit) & 1;
                        let lo = plane1 >> (7 - bit) & 1;
                        let palette_index = (hi << 1) | lo;

                        let color_index = self.palette[palette_base + palette_index as usize];

                        let fb = (row * 256) + offset + bit;
                        self.framebuffer[fb] = self.internal_palette[color_index as usize];
                    }
                }
            }
        }
    }

    pub fn render_sprite(&mut self) {
        let sprite_data: Vec<[u8; 4]> = self
            .oam
            .chunks(4)
            .map(|chunk| {
                let mut arr = [0u8; 4];
                arr.copy_from_slice(chunk);
                arr
            })
            .collect();

        for data in sprite_data.iter() {
            let sprite = self.get_sprite(data);
            if sprite.attributes.contains(SpriteAttr::Priority){
                continue;
            }
            let (sx, sy) = sprite.get_cord();
            let start_addr = sprite.get_tile_index() as usize * 16;

            //0x10 is offset to sprite pall
            let palette_base = 0x10 + sprite.get_palette() * 4;
            let tile: [u8; 16] = {
                let c = self.cartridge.borrow();
                let data = c.chr_rom_data();
                let mut buf = [0u8; 16];
                buf.copy_from_slice(&data[start_addr..start_addr + 16]);
                buf
            };

            let ver_flipped = sprite.attributes.contains(SpriteAttr::Vertical);
            let hor_flipped = sprite.attributes.contains(SpriteAttr::Horizontal);
            for dy in 0..8 {
                let dy = if ver_flipped { 7 - dy } else { dy };
                let plane0 = tile[dy];
                let plane1 = tile[dy + 8];
                for dx in 0..8 {
                    let bit_index = if !hor_flipped { 7 - dx } else { dx };
                    let hi = plane0 >> bit_index & 1;
                    let lo = plane1 >> bit_index & 1;

                    let palette_index = (hi << 1) | lo;
                    if palette_index == 0 {
                        continue;
                    }
                    let color_index = self.palette[(palette_base + palette_index) as usize];
                    let index = ((dy as u32 + sy as u32) * 256 + (dx as u32 + sx as u32)) as usize;
                    if index >= self.framebuffer.len() {
                        continue;
                    }
                    self.framebuffer[index] = self.internal_palette[(color_index as usize)];
                }
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

    pub fn get_attribute(&self, n: u8) -> &[u8] {
        match n {
            0 => &self.vram[0x3C0..0x400],
            1 => &self.vram[0x7C0..0x800],
            _ => panic!("Invalid attribute num {}", n),
        }
    }

    pub fn cpu_read(&mut self, address: u16, readonly: bool) -> Option<u8> {
        if readonly {
            return Some(0);
        }
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
                let addr = self.v & 0x3FFF;
                let data = if addr >= 0x3F00 {
                    let value = self.ppu_read_byte(addr);
                    self.temp_value = self.ppu_read_byte(addr - 0x1000);
                    value
                } else {
                    let buffered = self.temp_value;
                    self.temp_value = self.ppu_read_byte(addr);
                    buffered
                };

                self.v = self.v.wrapping_add(self.get_incr());

                Some(data)
            }
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
                let mut addr = self.v;
                if addr >= 0x2000 && addr <= 0x3EFF {
                    let mapped_addr = self.get_nametable_addr(addr);
                    self.vram[mapped_addr as usize] = value;
                } else if addr >= 0x3F00 && addr <= 0x3FFF {
                    addr &= 0x001F;
                    if addr == 0x0010 {
                        addr = 0x0000;
                    }
                    if addr == 0x0014 {
                        addr = 0x0004;
                    }
                    if addr == 0x0018 {
                        addr = 0x0008;
                    }
                    if addr == 0x001C {
                        addr = 0x000C;
                    }
                    self.palette[addr as usize] = value;
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

    pub fn oam_dma(&mut self, memory: &[u8], page_addr: u8) {
        let start_addr: usize = (page_addr as usize) << 8;
        let end_addr = start_addr + 256;
        self.oam.copy_from_slice(&memory[start_addr..end_addr]);
    }

    pub fn get_sprite(&self, data: &[u8]) -> Sprite {
        return Sprite {
            y_position: data[0],
            tile_index: data[1],
            attributes: SpriteAttr::from_bits(data[2]).unwrap(),
            x_position: data[3],
        };
    }

    fn ppu_read_byte(&self, mut addr: u16) -> u8 {
        addr &= 0x3FFF;

        match addr {
            0x0000..=0x1FFF => self.read_chr_rom(addr),
            0x2000..=0x3EFF => {
                let mapped = self.get_nametable_addr(addr);
                self.vram[mapped as usize]
            }
            0x3F00..=0x3FFF => {
                let mut p = addr & 0x001F;
                if p == 0x0010 {
                    p = 0x0000;
                }
                if p == 0x0014 {
                    p = 0x0004;
                }
                if p == 0x0018 {
                    p = 0x0008;
                }
                if p == 0x001C {
                    p = 0x000C;
                }
                self.palette[p as usize]
            }
            _ => 0,
        }
    }
}

#[derive(Debug)]
pub struct Sprite {
    x_position: u8,
    y_position: u8,
    tile_index: u8,
    attributes: SpriteAttr,
}

impl Sprite {
    pub fn get_cord(&self) -> (u8, u8) {
        (self.x_position, self.y_position)
    }

    pub fn get_tile_index(&self) -> u8 {
        self.tile_index
    }

    pub fn get_palette(&self) -> u8 {
        self.attributes.bits() & SpriteAttr::Palette.bits()
    }
}

bitflags! {
    #[derive(Debug)]
       pub struct SpriteAttr: u8 {
       const Palette =     0b00000011;
       const NotUsed =     0b00111100;
       const Priority  =   0b00100000;
       const Horizontal  = 0b01000000;
       const Vertical =    0b10000000;
    }
}
