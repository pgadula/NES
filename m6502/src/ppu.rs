use std::{cell::RefCell, rc::Rc};

use crate::cartridge::Cartridge;

#[derive(Debug)]
pub struct PPU {
    cartridge: Rc<RefCell<Cartridge>>,
    vram: [u8; 2048],    
    palette: [u8; 32],    
    
    //registers
    ppu_crtl: u8,
    ppu_mask: u8,
    ppu_status: u8,
    oam_addr: u8,
    oam_data: u8,
    ppu_scroll: u8,
    ppu_addr: u8,
    ppu_data: u8,
    oam_dma: u8,

    cycle:u16,
    pub scanline: u16
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

            cycle:0,
            scanline:0
        };
    }
    pub fn read_chr_rom(&self, address: u16) -> u8 {
        self.cartridge.borrow().chr_rom_data()[address as usize]
    }
    
    pub fn cpu_read(&mut self, address: u16) -> Option<u8> {
        let addr = match address {
            0x2000..=0x3FFF => 0x2000 + (address % 8),
            _ => address,
        };
        match addr {
            0x2000 => Some(self.ppu_crtl),
            0x2001 => Some(self.ppu_mask),
            0x2002 => {
                   self.ppu_status &= !0x80; // Clear VBlank

                Some(self.ppu_status)
            }
            0x2003 => Some(self.oam_addr),
            0x2004 => Some(self.oam_data),
            0x2005 => Some(self.ppu_scroll),
            0x2006 => Some(self.ppu_addr),
            0x2007 => Some(self.ppu_data),
            0x4014 => Some(self.oam_dma),
            _ => {
                eprintln!("[Error] addr:{:04x} out of boundary.", addr);
                None
            }
        }
    }

    pub fn cpu_write(&mut self, address: u16, value: u8) {
        let addr = match address {
            0x2000..=0x3FFF => 0x2000 + (address % 8),
            _ => address,
        };
        match addr {
            0x2000 => self.ppu_crtl = value,
            0x2001 => self.ppu_mask = value,
            0x2002 => {
               
                eprintln!("[Error] cannot write to addr 0x2002")
            }
            0x2003 => self.oam_addr = value,
            0x2004 => self.oam_data = value,
            0x2005 => self.ppu_scroll = value,
            0x2006 => self.ppu_addr = value,
            0x2007 => self.ppu_data = value,
            0x4014 => self.oam_dma = value,
            _ => {
                eprintln!("[Error] addr:{:04x} out of boundary.", addr);
            }
        }}
        
        pub fn nametable(&self, addr: u16)->u8{
           match addr{
               0x2000..=0x23ff=>{
                 return self.vram[addr as usize] 
               } 
               _=>{
                  eprintln!("[Error] reading from name table out of bounds! {:04x}", addr);
                  panic!("Fatal error");
               }  
            } 
        }   

        pub fn tick(&mut self) {
            self.cycle+=1;
            if self.cycle >= 341{
               self.cycle = 0;
               self.scanline +=1;

               if self.scanline == 241{
                  println!("[INFO] Hit scanline, switching ppu status"); 
                  self.ppu_status |= 0x80
               }
               if self.scanline >= 262{
                  println!("[INFO] new scaneline"); 

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
        println!("  PPUADDR:  0x{:02X}", self.ppu_addr);
        println!("  PPUDATA:  0x{:02X}", self.ppu_data);
        println!("  OAMDMA:   0x{:02X}", self.oam_dma);

        // Optionally dump a small part of VRAM:
        println!("  VRAM[0x2000..0x200F]: {:?}", &self.vram[0x0000..0x0010]);
        
        // Palette sample:
        println!("  Palette[0..8]: {:?}", &self.palette[0..8]);
    }
}


