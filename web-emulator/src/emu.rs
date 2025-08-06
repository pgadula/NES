use m6502::cpu::Mos6502;
use m6502::{bus::MainBus, cartridge::Cartridge, ppu::PPU};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;

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
        let lo = bus_rc.borrow_mut().read(0xFFFC);
        let hi = bus_rc.borrow_mut().read(0xFFFD);

        cpu_rc.borrow_mut().pc = ((hi as u16) << 8) | (lo as u16);
        Ok(WebEmu {
            ppu: ppu_rc,
            bus: bus_rc,
            cpu: cpu_rc,
        })
    }

    #[wasm_bindgen(js_name = "step")]
    pub fn step(&mut self) {
        let mut cc = self.cpu.borrow_mut();
        match cc.fetch() {
            Ok(instr) => {
                cc.execute(instr);
            }
            Err(err) => {
                panic!("{:?}", err);
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
    pub fn cpu_registers(&mut self) -> Vec<u16> {
        let cpu = self.cpu.borrow();
        let registers = vec![
            cpu.pc,
            cpu.p.bits() as u16,
            cpu.a as u16,
            cpu.x as u16,
            cpu.y as u16,
            cpu.sp as u16,
        ];
        return registers;
    }

    #[wasm_bindgen(js_name = "chrRom")]
    pub fn chr_rom_data(&mut self) -> Vec<u8> {
        let binding = self.ppu.borrow();
        let c = binding.cartridge.borrow();

        return c.chr_rom_data().to_vec();
    }

    #[wasm_bindgen(js_name = "nametable")]
    pub fn nametable(&mut self, n:u8) -> Vec<u8> {
        return self.ppu.borrow_mut().get_nametable(0).to_vec();

    }

    #[wasm_bindgen(js_name = "ramDump")]
    pub fn ram_dump(&mut self, addr: usize) -> Vec<u8> {
        let end_addr:usize = addr + (16 * 32);
        return self.bus.borrow().cpu_ram[addr..end_addr].to_vec();
    }
}
