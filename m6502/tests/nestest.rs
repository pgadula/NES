use std::{
    fs::File,
    io::{self, BufRead},
};

use regex::Regex;

#[cfg(test)]
mod tests {
    use std::{
        borrow::Borrow,
        cell::RefCell,
        fs::File,
        io::{self, BufRead},
        path::Path,
        rc::Rc,
    };

    use m6502::{
        cartridge::Cartridge,
        cpu::{MainBus, PFlag},
        opcodes::Opcode,
    };

    use crate::read_file_and_parse;

    #[test]
    fn nestests() {
        let mut bus = MainBus::new();
        let cartridge = Rc::new(RefCell::new(
            Cartridge::load_rom(Path::new(
                "/Users/pgadula/Programming/NES/m6502/resources/nestest.nes",
            ))
            .unwrap(),
        ));
        // hex_dump(&cartridge.borrow_mut().bytes);
        bus.load_cartridge(cartridge);
        let logs =
            read_file_and_parse("/Users/pgadula/Programming/NES/m6502/resources/nestest.log")
                .unwrap();
        
        let mut cpu = m6502::cpu::Mos6502::new(bus);
        cpu.pc = 0xC000;
        cpu.p = PFlag::Unused | PFlag::InterruptDisable;
        let mut n_step = 5;
        let mut running = true;
        while running {
            cpu.dump();
            println!("steps left: {}", n_step);
            match cpu.fetch() {
                Ok(instruction) => {
                    match logs.iter().next() {
                        Some(log) => println!("{:?}", log),
                        None => {
                            eprintln!("Invalid log!");
                            running = false;
                        }
                    }
                    n_step = n_step - 1;
                    running = if instruction.0 == Opcode::BRK {
                        false
                    } else {
                        true
                    };
                    if n_step <= 0 {
                        running = false;
                    }
                    println!("{:?} {:?}", instruction.0, instruction.1);
                }
                Err(e) => {
                    eprintln!("Invalid instruction!");
                }
            }
        }

        // hex_dump(c.bytes[16..124].to_vec());
        // println!("{:?}", cartridge.prg_size);
        // println!("{:?}", cartridge.flag_7);
        // println!("{:?}", cartridge.flag_6);
        // let mut rom = File::open("./resources/sm.nes").unwrap();
        // // log_iter();
        // let mut buff = Vec::new();
        // rom.read_to_end(&mut buff).unwrap();
        // hex_dump(buff);

        // cpu.fetch();
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
fn read_file_and_parse(file_path: &str) -> io::Result<Vec<Instruction>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut program = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if let Some(instruction) = parse_instruction(&line) {
            program.push(instruction);
        }
    }

    Ok(program)
}
fn parse_instruction(line: &str) -> Option<Instruction> {
    // Regex to match each part of the instruction line
   let re = Regex::new(r"([0-9A-F]{4})\s([0-9A-F]{2})\s([0-9A-F]{2})\s([0-9A-F]{2})\s([A-Z]+)\s?\$([0-9A-F]{4})\s+.*A:([0-9A-F]{2})\sX:([0-9A-F]{2})\sY:([0-9A-F]{2})\sP:([0-9A-F]{2})\sSP:([0-9A-F]{2})\sPPU:\s([0-9]+),\s([0-9]+)\sCYC:([0-9]+)")
        .unwrap();

    if let Some(caps) = re.captures(line) {
        let address = u16::from_str_radix(&caps[1], 16).unwrap();
        let opcode = u8::from_str_radix(&caps[2], 16).unwrap();
        let immediate = if let Some(im) = caps.get(3) {
            Some(u8::from_str_radix(im.as_str(), 16).unwrap())
        } else {
            None
        };
        let memory_addr = Some(u16::from_str_radix(&caps[5], 16).unwrap());
        let a = u8::from_str_radix(&caps[7], 16).unwrap();
        let x = u8::from_str_radix(&caps[8], 16).unwrap();
        let y = u8::from_str_radix(&caps[9], 16).unwrap();
        let p = u8::from_str_radix(&caps[10], 16).unwrap();
        let sp = u8::from_str_radix(&caps[11], 16).unwrap();
        let ppu = caps[12].replace(',', "").parse::<u32>().unwrap();
        let cycles = caps[13].parse::<u32>().unwrap();

        Some(Instruction::new(
            address,
            opcode,
            immediate,
            memory_addr,
            a,
            x,
            y,
            p,
            sp,
            ppu,
            cycles,
        ))
    } else {
        None
    }
}

#[derive(Debug)]
struct Instruction {
    address: u16,
    opcode: u8,
    immediate: Option<u8>,
    memory_addr: Option<u16>,
    a: u8,
    x: u8,
    y: u8,
    p: u8,
    sp: u8,
    ppu: u32,
    cycles: u32,
}

impl Instruction {
    fn new(
        address: u16,
        opcode: u8,
        immediate: Option<u8>,
        memory_addr: Option<u16>,
        a: u8,
        x: u8,
        y: u8,
        p: u8,
        sp: u8,
        ppu: u32,
        cycles: u32,
    ) -> Self {
        Instruction {
            address,
            opcode,
            immediate,
            memory_addr,
            a,
            x,
            y,
            p,
            sp,
            ppu,
            cycles,
        }
    }
}
