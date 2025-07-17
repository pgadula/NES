use crate::cpu::{self};

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