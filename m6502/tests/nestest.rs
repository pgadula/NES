#[cfg(test)]
mod tests {
    use std::{
        cell::RefCell, fs::File, io::{self, BufRead}, path::Path, rc::Rc
    };

    use m6502::{cartridge::Cartridge, cpu::MainBus};

    #[test]
    fn nestests() {
        let mut bus = MainBus::new();
        let cartridge = Rc::new(RefCell::new(Cartridge::load_rom(Path::new("/Users/pgadula/Programming/NES/m6502/resources/dp.nes")).unwrap()));
         
        bus.load_cartridge(cartridge);
        let mut cpu = m6502::cpu::Mos6502::new(bus);
        cpu.pc = 0xC000; 
        // hex_dump(c.bytes[16..124].to_vec());
        // println!("{:?}", cartridge.prg_size); 
        // println!("{:?}", cartridge.flag_7); 
        // println!("{:?}", cartridge.flag_6); 
        // let mut rom = File::open("./resources/sm.nes").unwrap();
        // // log_iter();
        // let mut buff = Vec::new();
        // rom.read_to_end(&mut buff).unwrap();
        // hex_dump(buff);
        
        cpu.fetch();
        cpu.fetch();
        assert!(false);
    }
    
    fn log_iter() {
        let log = File::open("./resources/nestest.log").unwrap();
        let log_reader = io::BufReader::new(log);
        let lines = log_reader.lines();
        for l in lines.map_while(Result::ok) {
            println!("{l}");
        }
    }
}
