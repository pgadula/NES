use std::{cell::RefCell, rc::Rc};

use crate::{cartridge::Cartridge, ppu::PPU};

#[derive(Debug)]
pub struct MainBus {
    pub cpu_ram: [u8; 0x10000],
    pub cartridge: Option<Rc<RefCell<Cartridge>>>,
    pub ppu: Rc<RefCell<PPU>>,
}

impl MainBus {
    pub fn new(ppu: Rc<RefCell<PPU>>) -> MainBus {
        return MainBus {
            cpu_ram: [0; 0x10000],
            cartridge: None,
            ppu,
        };
    }
    pub fn dump(&self) {
        println!("{}", self);
    }

    pub fn load_cartridge(&mut self, cartridge: Rc<RefCell<Cartridge>>) {
        self.cartridge = Some(cartridge);
    }

    pub fn read(&mut self, address: u16) -> u8 {
        let addr = address as usize;

        if let Some(c) = self.cartridge.as_ref() {
            if let Ok(data) = c.borrow_mut().read(addr) {
                // println!("[INFO] reading from cartridge {:04x}", addr);

                return data;
            }
        }
        match address {
            0x0000..=0x1FFF => {
                // println!("\x1b[32m[INFO] reading from CPU\x1b[0m");
                return self.cpu_ram[address as usize];
            }
            0x2000..=0x3FFF => {
                let value = self.ppu.borrow_mut().cpu_read(address).unwrap_or(0);
                // println!(
                //     "\x1b[32m[INFO] reading from PPU\x1b[0m {:04x}:{:04x}",
                //     address, value
                // );
                value
            }
            0x4000..=0xFFFF => {
                eprintln!("reading from unknown device {:04x}", addr);
                return 0;
            }
        }
    }

    pub fn write(&mut self, address: usize, value: u8) {
        let addr = address as usize;
        if let Some(c) = self.cartridge.as_ref() {
            let mut borrowed = c.borrow_mut();
            match borrowed.write(addr, value) {
                Ok(_) => return,
                _ => {}
            }
        }
        match addr {
            0x0000..=0x1FFF => {
                // println!(
                //     "\x1b[32m[INFO] Writing to CPU RAM addr:{:04X} value {}\x1b[0m",
                //     addr, value
                // );
                self.cpu_ram[addr & 0x07FF] = value;
            }
            0x2000..=0x3FFF => {
                println!(
                    "\x1b[32m[INFO] Writing to PPU RAM addr:{:04X} value {}\x1b[0m",
                    addr, value
                );
                if addr > 0x200f {
                    panic!("Writing to ppu");
                }

                self.ppu.borrow_mut().cpu_write(addr as u16, value);
            }
            0x4000..=0x4017 => {
                println!(
                    "\x1b[32m[INFO] writing to APU addr:{:04X} value {}\x1b[0m",
                    addr, value
                );
            }
            _ => {
                eprintln!("Unahandled address {:04X}", addr);
            }
        }
    }

    pub fn write_bytes(&mut self, address: usize, value: &[u8]) {
        let mut start_address = address;
        for byte in value {
            self.write(start_address, *byte);
            start_address += 1;
        }
    }
}
