use std::{
    fs::File,
    io::{self, BufRead},
    num::ParseIntError,
};

use m6502::{cpu::PFlag, helpers::CpuState};

#[cfg(test)]
mod tests {
    use std::{
        cell::RefCell,
        fs::File,
        io::{self, BufRead},
        path::Path,
        rc::Rc,
    };
 
    use m6502::{
        cartridge::Cartridge,
        cpu::{self, MainBus, Mos6502},
        helpers::cpu_dump_state,
        opcodes::Opcode,
    };

    use crate::{compare_cpu_state, read_file_and_parse};

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
        let mut logs =
            read_file_and_parse("/Users/pgadula/Programming/NES/m6502/resources/nestest.log")
                .unwrap()
                .into_iter();
        let mut cpu = Mos6502::new(bus);
        cpu.pc = 0xC000;
        let mut n_step = 8991;
        let mut running = true;
        let mut line: i32 = 0;
        while running {
            match cpu.fetch() {
                Ok(instruction) => {
                    line = line + 1;
                    let log = logs.next().unwrap();
                    println!(
                        "[{line}] Fetched: {:?} {:?}\t Log: {}",
                        instruction.0, instruction.1, log.instruction
                    );
                    cpu.dump();
                    let emu_state = cpu_dump_state(&cpu);
                    if let Err(error) = compare_cpu_state(&emu_state, &log.cpu_state) {
                        cpu.dump();
                        assert!(false, "CPU state mismatch: {}", error);
                    }

                    cpu.execute(instruction);
                    n_step = n_step - 1;

                    let result = cpu.bus.read(0x6000);
                    if result > 0 {
                        println!("Error {}", result);
                    }
                    running = if instruction.0 == Opcode::BRK {
                        false
                    } else {
                        true
                    };
                    if n_step <= 0 {
                        running = false;
                    }
                }
                Err(e) => {
                    
                    eprintln!("Invalid instruction! {:?}", e);
                    cpu.dump();
                    panic!(".")
                }
            }
        }
        assert!(true);
    }
}

fn read_file_and_parse(file_path: &str) -> io::Result<Vec<InstructionLine>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut program = Vec::new();

    for line in reader.lines() {
        let line = line?;
        program.push(parse_line(&line).unwrap());
    }
    Ok(program)
}

fn parse_line(line: &str) -> Result<InstructionLine, String> {
    let addr_str = &line[0..4];
    let address = u16::from_str_radix(addr_str, 16).map_err(|e| e.to_string())?;
    let bytes_part = &line[6..14].trim();
    let bytes = bytes_part
        .split_whitespace()
        .map(|b| u8::from_str_radix(b, 16))
        .collect::<Result<Vec<_>, ParseIntError>>()
        .map_err(|e| e.to_string())?;

    let cpu_start = line.find("A:").ok_or("No CPU state found")?;
    let instruction = line[15..cpu_start].trim().to_string();

    let cpu_str = &line[cpu_start..];

    let mut a = 0;
    let mut x = 0;
    let mut y = 0;
    let mut p = 0;
    let mut sp = 0;
    let mut ppu = (0, 0);
    let mut cyc = 0;

    for part in cpu_str.split_whitespace() {
        if part.starts_with("A:") {
            a = u8::from_str_radix(&part[2..], 16).map_err(|e| e.to_string())?;
        } else if part.starts_with("X:") {
            x = u8::from_str_radix(&part[2..], 16).map_err(|e| e.to_string())?;
        } else if part.starts_with("Y:") {
            y = u8::from_str_radix(&part[2..], 16).map_err(|e| e.to_string())?;
        } else if part.starts_with("P:") {
            p = u8::from_str_radix(&part[2..], 16).map_err(|e| e.to_string())?;
        } else if part.starts_with("SP:") {
            sp = u8::from_str_radix(&part[3..], 16).map_err(|e| e.to_string())?;
        } else if part.starts_with("PPU:") {
            let ppu_parts: Vec<&str> = cpu_str.split(&[' ', ','][..]).collect();
            if ppu_parts.len() > 3 {
                ppu.0 = ppu_parts[1].parse().unwrap_or(0);
                ppu.1 = ppu_parts[2].parse().unwrap_or(0);
            }
        } else if part.starts_with("CYC:") {
            cyc = part[4..].parse().unwrap_or(0);
        }
    }

    let cpu_state = CpuState {
        a,
        x,
        y,
        p,
        sp,
        ppu,
        cyc,
    };

    Ok(InstructionLine {
        address,
        bytes,
        instruction,
        cpu_state,
    })
}

#[derive(Debug)]
struct InstructionLine {
    address: u16,
    bytes: Vec<u8>,
    instruction: String,
    cpu_state: CpuState,
}

fn compare_cpu_state(c1: &CpuState, c2: &CpuState) -> Result<(), String> {
    if c1.a != c2.a {
        return Err(format!("A register mismatch: {} != {}", c1.a, c2.a));
    }
    if c1.x != c2.x {
        return Err(format!("X register mismatch: {} != {}", c1.x, c2.x));
    }
    if c1.y != c2.y {
        return Err(format!("Y register mismatch: {} != {}", c1.y, c2.y));
    }
    if c1.p != c2.p {
        let cpu_p = PFlag::from_bits(c1.p);
        let expected_flags = PFlag::from_bits(c2.p);
        return Err(format!(
            "P register mismatch: {:02X} != {:02X}\n expected: {:?} != result {:?}",
            c1.p, c2.p, expected_flags, cpu_p
        ));
    }
    if c1.sp != c2.sp {
        return Err(format!("SP register mismatch: {} != {}", c1.sp, c2.sp));
    }
    Ok(())
}
