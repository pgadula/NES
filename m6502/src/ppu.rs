use std::{cell::RefCell, rc::Rc};

use crate::cartridge::Cartridge;

#[derive(Debug)]
pub struct PPU{
   cartridge: Rc<RefCell<Cartridge>>,
   ppu_crtl: u8,
   ppu_mask: u8,
   ppu_status: u8,
   oam_addr: u8,
   oam_data: u8,
   ppu_scroll: u8,
   ppu_addr: u8,
   ppu_data: u8,
   oam_dma: u8
}

impl PPU {
   pub fn new(cartridge: Rc<RefCell<Cartridge>>,
)-> PPU{
      return PPU{
        cartridge,
        oam_addr:0,
        oam_data:0,
        oam_dma:0,
        ppu_addr:0,
        ppu_crtl:0,
        ppu_data:0,
        ppu_mask:0,
        ppu_scroll:0,
        ppu_status:0
    }
   }
   pub fn read_chr_rom(&self,  address: u16)->u8{
      self.cartridge.borrow().chr_rom_data()[address as usize]
   }
   pub fn read(&self, address: u16) -> Option<u8> {
      let addr = match address {
         0x2000..=0x3FFF => 0x2000 + (address % 8 ),
         _ => address,
      };
      match addr {
         0x2000 => Some(self.ppu_crtl),
         0x2001 => Some(self.ppu_mask),
         0x2002 => Some(self.ppu_status),
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
   pub fn write(& mut self, address: u16, value: u8){
      let addr = match address {
         0x2000..=0x3FFF => 0x2000 + (address % 8 ),
         _ => address,
      };
      match addr {
         0x2000 => {self.ppu_crtl = value},
         0x2001 => {self.ppu_mask = value},
         0x2002 => { eprintln!("[Error] cannot write to addr 0x2002")},
         0x2003 => {self.oam_addr = value},
         0x2004 => {self.oam_data = value},
         0x2005 => {self.ppu_scroll = value},
         0x2006 => {self.ppu_addr = value},
         0x2007 => {self.ppu_data = value},
         0x4014 => {self.oam_dma = value},
         _ => {
            eprintln!("[Error] addr:{:04x} out of boundary.", addr);
         }
      };
   }

}