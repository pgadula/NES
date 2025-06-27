#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{self, BufRead}, path::Path,
    };

    use m6502::{cartridge::Cartridge, helpers::hex_dump};

    #[test]
    fn nestests() {
        let c = Cartridge::load_rom(Path::new("./resources/nestest.nes")).unwrap();
        // hex_dump(c.bytes[16..124].to_vec());

        println!("{:?}", c.prg_size); 
        println!("{:?}", c.flag_7); 
        println!("{:?}", c.flag_6); 
        // let mut rom = File::open("./resources/sm.nes").unwrap();
        // // log_iter();
        // let mut buff = Vec::new();
        // rom.read_to_end(&mut buff).unwrap();
        // hex_dump(buff);
        // hex_dump(c.prg_rom_data().to_vec());
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
