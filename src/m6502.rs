use std::fmt::{self, write, Display, Formatter};

use bitflags::bitflags;

use crate::opcodes::{resolve_opcode, Instruction, Opcode};

pub const STACK_BASE: u8 = 0x01;
pub const VECTOR_BASE: u8 = 0xFF;
pub const IRQ_BRK_VECTOR: u8 = 0xFE;
pub const RESET_VECTOR: u8 = 0xFC;
pub const NMI_VECTOR: u8 = 0xFA;

#[derive(Debug)]
pub struct Mos6502 {
    pub bus: Bus,
    pub pc: u16,
    pub p: PFlag,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,

    //
    pub operand: u8,
    pub abs_addr: u16,
}

#[derive(Debug)]
pub struct Bus {
    memory: [u8; 1024 * 2],
}

impl Bus {
    pub fn new() -> Bus {
        return Bus { memory: [0; 2048] };
    }
    pub fn dump(&self) {
        println!("{}", self);
    }

    pub fn read_u16(&mut self, hi: u8, le: u8) -> u8 {
        let mut adr: u16 = le as u16;
        adr |= (hi as u16) << 8;
        println!("{:04X}", adr);
        self.read(adr)
    }

    pub fn read(&self, address: u16) -> u8 {
        return self.memory[address as usize];
    }

    pub fn write(&mut self, address: usize, value: u8) {
        self.memory[address] = value;
    }

    pub fn write_bytes(&mut self, address: usize, value: &[u8]) {
        let mut start_address = address;
        for byte in value {
            self.write(start_address, *byte);
            start_address += 1;
        }
    }
}

impl Mos6502 {
    pub fn new(bus: Bus) -> Mos6502 {
        Mos6502 {
            a: 0,
            p: PFlag::Carry,
            pc: 0,
            sp: 0,
            x: 0,
            y: 0,
            bus,
            //
            operand: 0,
            abs_addr: 0,
        }
    }
    pub fn reset(&mut self) {
        let address = ((VECTOR_BASE as u16) << 8) | RESET_VECTOR as u16;
        let le = self.bus.read(address);
        let hi = self.bus.read(address + 1);
        self.pc = Mos6502::get_address_from_bytes(hi, le);
    }

    pub fn get_address_from_bytes(hi: u8, lo: u8) -> u16 {
        u16::from(lo) + (u16::from(hi) << 8usize)
    }

    pub fn fetch(&mut self) {
        let opcode = self.bus.read(self.pc);
        println!("0x{:02X}", opcode);
        let resolved = resolve_opcode(opcode);
        match resolved {
            Some(instruction) => {
                println!("{:?}\n", instruction);
                self.execute(instruction);
            }
            None => println!("None\n"),
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        self.pc += 1;
        match instruction.0 {
            Opcode::ADC => {
                instruction.1.ex(self);
                
            }
            Opcode::AND => todo!(),
            Opcode::ASL => todo!(),
            Opcode::BCC => todo!(),
            Opcode::BCS => todo!(),
            Opcode::BEQ => todo!(),
            Opcode::BIT => todo!(),
            Opcode::BMI => todo!(),
            Opcode::BNE => todo!(),
            Opcode::BPL => todo!(),
            Opcode::BRK => todo!(),
            Opcode::BVC => todo!(),
            Opcode::BVS => todo!(),
            Opcode::CLC => todo!(),
            Opcode::CLD => todo!(),
            Opcode::CLI => todo!(),
            Opcode::CLV => todo!(),
            Opcode::CMP => todo!(),
            Opcode::CPX => todo!(),
            Opcode::CPY => todo!(),
            Opcode::DEC => todo!(),
            Opcode::DEX => todo!(),
            Opcode::DEY => todo!(),
            Opcode::EOR => todo!(),
            Opcode::INC => todo!(),
            Opcode::INX => todo!(),
            Opcode::INY => todo!(),
            Opcode::JMP => todo!(),
            Opcode::JSR => todo!(),
            Opcode::LDA => {
                instruction.1.ex(self);
                println!("{}\n", self.abs_addr);
                let value = self.bus.read(self.abs_addr);
                println!("value: {}\n ", value);
            }
            Opcode::LDX => todo!(),
            Opcode::LDY => todo!(),
            Opcode::LSR => todo!(),
            Opcode::NOP => todo!(),
            Opcode::ORA => todo!(),
            Opcode::PHA => todo!(),
            Opcode::PHP => todo!(),
            Opcode::PLA => todo!(),
            Opcode::PLP => todo!(),
            Opcode::ROL => todo!(),
            Opcode::ROR => todo!(),
            Opcode::RTI => todo!(),
            Opcode::RTS => todo!(),
            Opcode::SBC => todo!(),
            Opcode::SEC => todo!(),
            Opcode::SED => todo!(),
            Opcode::SEI => todo!(),
            Opcode::STA => todo!(),
            Opcode::STX => todo!(),
            Opcode::STY => todo!(),
            Opcode::TAX => todo!(),
            Opcode::TAY => todo!(),
            Opcode::TSX => todo!(),
            Opcode::TXA => todo!(),
            Opcode::TXS => todo!(),
            Opcode::TYA => todo!(),
            Opcode::AHX => todo!(),
            Opcode::ALR => todo!(),
            Opcode::ANC => todo!(),
            Opcode::ARR => todo!(),
            Opcode::AXS => todo!(),
            Opcode::DCP => todo!(),
            Opcode::ISC => todo!(),
            Opcode::KIL => todo!(),
            Opcode::LAS => todo!(),
            Opcode::LAX => todo!(),
            Opcode::RLA => todo!(),
            Opcode::RRA => todo!(),
            Opcode::SAX => todo!(),
            Opcode::SHX => todo!(),
            Opcode::SHY => todo!(),
            Opcode::SLO => todo!(),
            Opcode::SRE => todo!(),
            Opcode::TAS => todo!(),
            Opcode::XAA => todo!(),
        };
    }

    pub fn inc_pc(&mut self) -> u8 {
        let addr = self.bus.read(self.pc);
        self.pc += 1;
        addr
    }
}

impl Display for Mos6502 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // Header
        writeln!(f, "┌──────────────────────────────────┐")?;
        writeln!(f, "│           MOS 6502 CPU           │")?;
        writeln!(f, "├──────────────────────────────────┤")?;

        // Program counter & stack pointer
        writeln!(
            f,
            "│ PC: 0x{:04X}   SP: 0x{:02X}            │",
            self.pc, self.sp
        )?;

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
