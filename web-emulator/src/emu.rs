use m6502::cartridge;
use m6502::cpu::Mos6502;
use m6502::{bus::MainBus, cartridge::Cartridge, helpers::hex_dump, helpers::ppm, ppu::PPU};
use std::fs::File;
use std::io::Read;
use std::{cell::RefCell, io::Error, path::Path, rc::Rc};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn start_emu() {

    //   let cartridge: Rc<RefCell<Cartridge>> = Rc::new(RefCell::new(Cartridge::load_rom(Path::new(
    //     "resources/sm.nes",
    // )).unwrap()));
    // let nes_palette:[u32; 32] = [0; 32];
    // let ppu = Rc::new(RefCell::new(PPU::new(cartridge.clone())));
    // let mut main_bus = MainBus::new(ppu.clone());
    // // println!("{:?}", cartridge.borrow().mirroring);
    // // main_bus.load_cartridge(cartridge.clone());
    // // let lo = main_bus.read(0xFFFC);
    // // let hi = main_bus.read(0xFFFD);
    // let mut cpu = m6502::cpu::Mos6502::new(main_bus);
    // // cpu.pc = ((hi as u16) << 8) | (lo as u16);
    // // let background = cartridge.clone().borrow_mut().chr_rom_data().to_vec();

    // let mut running = true;
    // let mut line: u64 = 0;
    // while running {
    //     match cpu.fetch() {
    //         Ok(instr) => {
    //             cpu.execute(instr);
    //         }
    //         Err(_) => {
    //             running = false;
    //         }
    //     }
    //     for _ in 0..(3) {
    //         let mut nmi_closure = || cpu.nmi();
    //         ppu.borrow_mut().tick(Some(&mut nmi_closure));
    //     }
    //     if line == 15905098 {
    //         running = false;
    //     }
    //     line += 1;
    // }
}

#[wasm_bindgen]
pub struct WebEmu {
    ppu: Rc<RefCell<PPU>>,
    bus: Rc<RefCell<MainBus>>,
    cpu: Rc<RefCell<Mos6502>>,
}

#[wasm_bindgen]
impl WebEmu {
    #[wasm_bindgen(js_name = "loadCartridge")]
    pub fn load_cartridge(bytes: &[u8]) -> Result<WebEmu, JsValue> {
        let cartridge =
            Cartridge::load_rom_from_bytes(bytes).map_err(|e| JsValue::from_str(&e.to_string()))?;

        let cartridge_rc = Rc::new(RefCell::new(cartridge));
        let ppu_rc = Rc::new(RefCell::new(PPU::new(cartridge_rc.clone())));
        let bus_rc = Rc::new(RefCell::new(MainBus {
            cpu_ram: [0; 0x10000],
            cartridge: Some(cartridge_rc.clone()),
            ppu: ppu_rc.clone(),
        }));
        let cpu_rc = Rc::new(RefCell::new(Mos6502::new(bus_rc.clone())));

        Ok(WebEmu {
            ppu: ppu_rc,
            bus: bus_rc,
            cpu: cpu_rc,
        })
    }

    #[wasm_bindgen(js_name = "step")]
    pub fn step(&mut self) {
        let lo = self.bus.borrow_mut().read(0xFFFC);
        let hi = self.bus.borrow_mut().read(0xFFFD);
        let mut cc = self.cpu.borrow_mut();
        cc.pc = ((hi as u16) << 8) | (lo as u16);
        match cc.fetch() {
            Ok(instr) => {
                cc.execute(instr);
            }
            Err(_) => {
                // Handle the error or simply do nothing
            }
        }
        for _ in 0..(3) {
            let mut nmi_closure = || cc.nmi();
            self.ppu.borrow_mut().tick(Some(&mut nmi_closure));
        }
    }

    #[wasm_bindgen(js_name = "getFramebuffer")]
    pub fn generate_framebuffer(&mut self) -> Vec<u32> {
        self.ppu.borrow_mut().render();
        let result = self.ppu.borrow().framebuffer.to_vec();
        return result;
    }

    #[wasm_bindgen(js_name = "ppuPalette")]
    pub fn ppu_palette(&mut self) -> Vec<u32> {
        return self.ppu.borrow_mut().internal_palette.to_vec();
    }

    #[wasm_bindgen(js_name = "cpuRegisters")]
    pub fn cpu_registers(&mut self) -> Vec<u32> {
        let cpu = self.cpu.borrow();
        
        return vec![cpu.a as u32, cpu.pc as u32]
    }
}
