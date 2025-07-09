use std::{cell::RefCell, rc::Rc};

use crate::cartridge::Cartridge;

#[derive(Debug)]
pub struct MainBus {
    pub cpu_ram: [u8; 0x10000],
    pub cartridge: Option<Rc<RefCell<Cartridge>>>,
}

impl MainBus {
    pub fn new() -> MainBus {
        return MainBus {
            cpu_ram: [0; 0x10000],
            cartridge: None,
        };
    }
    pub fn dump(&self) {
        println!("{}", self);
    }

    pub fn load_cartridge(&mut self, cartridge: Rc<RefCell<Cartridge>>) {
        self.cartridge = Some(cartridge);
    }

    pub fn read(&self, address: u16) -> u8 {
        let addr = address as usize;
        if let Some(c) = self.cartridge.as_ref() {
            let borrowed = c.borrow_mut();
            return match borrowed.read(addr) {
                Ok(data) => data,
                Err(_) => self.cpu_ram[addr],
            };
        }
        return 0;
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
                println!(
                    "[INFO] writing to CPU RAM addr:{:04X} value {}",
                    addr, value
                );
                self.cpu_ram[addr & 0x07FF] = value;
            }
            0x4000..=0x4017 => {
                println!("[INFO] Ignoring APU write: ${:04X} = {}", addr, value);
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
