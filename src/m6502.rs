
use std::fmt::{self, write, Display, Formatter};

use bitflags::bitflags;

bitflags! {
    #[derive(Debug)]
    pub struct PFlag: u32 {
        const Carry =            0b00000001;
        const Zero =             0b00000010;
        const InterruptDisable = 0b00000100;
        const DecimalMode =      0b00001000;
        const BreakCommand =     0b00010000;
        const Overflow =         0b00100000;
        const Negative =         0b01000000;
    }
}

#[derive(Debug)]
pub struct Mos6502{
    pub bus: Bus,
    pub pc: u16,
    pub p: PFlag,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,

}

#[derive(Debug)]
pub struct Bus{
    memory:[u8; 1024*2]
}

impl Bus {
    pub fn new()->Bus{
       return Bus{
           memory: [0; 2048] 
       } 
    }

    pub fn read(& self, address: usize)->u8{
        return self.memory[address];
    }

    pub fn write(& mut self, address: usize, value: u8){
        self.memory[address] = value;
    }
}

impl Mos6502 {
    pub fn new(bus: Bus)->Mos6502{
        Mos6502{
         a: 0,
         p: PFlag::Carry,
         pc: 0,
         sp: 0,
         x: 0,
         y: 0,
         bus,
        } 
    }

    pub fn fetch(&mut self){

    }
}

impl Display for Mos6502 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // Header
        writeln!(f, "┌──────────────────────────────────┐")?;
        writeln!(f, "│           MOS 6502 CPU           │")?;
        writeln!(f, "├──────────────────────────────────┤")?;

        // Program counter & stack pointer
        writeln!(f, "│ PC: 0x{:04X}   SP: 0x{:02X}            │", self.pc, self.sp)?;

        // General‑purpose registers
        writeln!(
            f,
            "│  A: 0x{:02X}      X: 0x{:02X}    Y: 0x{:02X} │",
            self.a, self.x, self.y
        )?;

        // Processor status (as raw byte + decoded flags)
        writeln!(f, "│  P: 0b{:08b}  ({:?}) . │", self.p, self.p)?;

        // Footer
        writeln!(f, "└──────────────────────────────────┘")
    }
}

impl Display for Bus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "+---------------- 2 KiB RAM ----------------+")?;

        for (row, chunk) in self.memory.chunks(16).enumerate() {
            write!(f, "{:04X} |", row * 16)?;

            for byte in chunk {
                write!(f, " {:02X}", byte)?;
            }

            for _ in 0..(16 - chunk.len()) {
                write!(f, "   ")?;       
            }

            write!(f, " | ")?;
            for &byte in chunk {
                let ch = byte as char;
                write!(f, "{}", if ch.is_ascii_graphic() { ch } else { '.' })?;
            }

            writeln!(f)?;
        }
        writeln!(f, "+-------------------------------------------+")
    }
}