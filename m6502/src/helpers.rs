use crate::{cpu::{self}, opcodes::{resolve_opcode, AddressingMode}};
use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};


pub fn ppm(file_name: &str, width: u32, height: u32, bytes: &Vec<u32>) -> io::Result<()> {
    let mut file = File::create(file_name)?;
    let data = width * height * 3; //three chanells per color
    let header = format!("P6\n{} {}\n 255", width, height);
    let mut writer = BufWriter::with_capacity((data + (header.len() as u32)) as usize, file);
    println!("preallocate buffer with size: {}", writer.capacity());
     writeln!(writer, "{}", header);
    for byte in bytes{
        let r = ((byte >> 16) & 0xFF) as u8;
        let g = ((byte >> 8 ) & 0xFF) as u8;
        let b = ((byte >> 0 ) & 0xFF) as u8;
        writer.write_all(&[r,g,b]);
    }

    writer.flush();
    println!("File {} has been save", file_name);
    Ok(())
}

pub fn hex_dump(buff: &[u8]) {
    for (i, chunk) in buff.chunks(16).enumerate() {
        print!("{:04X}  ", i * 16);
        for byte in chunk {
            print!("{:02X} ", byte);
        }

        print!(" |");
        for byte in chunk {
            let ch = *byte as char;
            print!("{}", if ch.is_ascii_graphic() { ch } else { '.' });
        }
        println!("|");
    }
}

pub fn cpu_dump_state(cpu: &cpu::Mos6502)->CpuState{
    return CpuState { a: cpu.a, x: cpu.x, y: cpu.y, p: cpu.p.bits(), sp: cpu.sp, ppu: (0,0), cyc: 0 } 
}
#[derive(Debug)]
pub struct CpuState {
   pub a: u8,
   pub x: u8,
   pub y: u8,
   pub p: u8,
   pub sp: u8,
   pub ppu: (u32, u32),
   pub cyc: u32,
}

pub fn disassembler(cpu: &mut cpu::Mos6502, n_instruction:u16) {
    let mut i: u16 = 0;
    while i < n_instruction {
        let addr = cpu.pc + i;
        let opbyte = cpu.bus.borrow_mut().read(addr);
        let (mnemonic, mode) = resolve_opcode(opbyte).unwrap();
        let operand_len = AddressingMode::get_bytes(mode) as u16;
        print!("{:?}", mnemonic);

        for i in 1..operand_len {
            let byte = cpu.bus.borrow_mut().read(cpu.pc + i + i);
            print!(" {:02x}", byte);
        }
        print!("\t ##{:?}", mode);
        i += operand_len;
    }
    println!();
}
