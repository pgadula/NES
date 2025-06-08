use std::fmt::{self, Display, Formatter};

use bitflags::{bitflags, Flags};

use crate::opcodes::{resolve_opcode, AddressingMode, Instruction, Opcode};

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
    pub fetched: u8,
    pub rel_addr: u16,
    pub abs_addr: u16,
}
bitflags! {
    #[derive(Debug)]
    pub struct PFlag: u8 {
       const Carry =            0b00000001;
       const Zero =             0b00000010;
       const InterruptDisable = 0b00000100;
       const DecimalMode =      0b00001000;
       const BreakCommand =     0b00010000;
       const Unused =           0b00100000;
       const Overflow =         0b01000000;
       const Negative =         0b10000000;
    }
}

#[derive(Debug)]
pub struct Bus {
    memory: [u8; 0x10000],
}

impl Bus {
    pub fn new() -> Bus {
        return Bus {
            memory: [0; 0x10000],
        };
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
            p: PFlag::Unused | PFlag::InterruptDisable, 
            pc: 0,
            sp: VECTOR_BASE,
            x: 0,
            y: 0,
            bus,
            //helpers
            fetched: 0,
            abs_addr: 0,
            rel_addr: 0,
        }
    }
    pub fn dump(&self) {
        println!("{}", self)
    }

    pub fn reset(&mut self) {
        let address = ((VECTOR_BASE as u16) << 8) | RESET_VECTOR as u16;
        let lo = self.bus.read(address);
        let hi = self.bus.read(address + 1);
        self.pc = Mos6502::get_address_from_bytes(hi, lo);
        self.sp = VECTOR_BASE;
    }
    //little endian low-order byte
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

    fn update_zero_flag(&mut self, value: u8) {
        self.p.set(PFlag::Zero, value == 0);
    }
    fn update_neg_flag(&mut self, value: u8) {
        self.p.set(PFlag::Negative, value & 0x80 > 0);
    }
    fn update_overflow_flag(&mut self, a: u8, operand: u8, sum: u8) {
        self.p
            .set(PFlag::Overflow, ((!(a ^ operand) & (a ^ sum)) & 0x80) != 0)
    }

    fn update_carry_flag(&mut self, value: u16) {
        self.p.set(PFlag::Carry, value > 0xFF);
    }

    fn execute(&mut self, instruction: Instruction) {
        self.pc += 1;
        match instruction.0 {
            Opcode::ADC => {
                instruction.1.ex(self);
                let carrying = if self.p.contains(PFlag::Carry) { 1 } else { 0 };
                let v1 = self.fetched as u16 + carrying;
                let sum: u16 = self.a as u16 + v1;
                let a = self.a as u16;
                self.update_carry_flag(sum);
                self.update_overflow_flag(v1 as u8, a as u8, sum as u8);
                self.a = sum as u8;
            }
            Opcode::AND => {
                instruction.1.ex(self);
                let value = self.a & self.fetched;
                self.update_zero_flag(value);
                self.update_neg_flag(value);
                self.a = value;
            }
            Opcode::ASL => {
                let mut value: u16 = 0;
                if instruction.1 == AddressingMode::Implied {
                    value = self.a as u16;
                } else {
                    instruction.1.ex(self);
                    value = self.fetched as u16;
                }
                value = value << 1;
                self.update_zero_flag(value as u8);
                self.update_neg_flag(value as u8);
                self.update_carry_flag(value);
            }
            Opcode::BCC => todo!(),
            Opcode::BCS => todo!(),
            Opcode::BEQ => todo!(),
            Opcode::BIT => {
                instruction.1.ex(self);
                let value = self.a & self.fetched;

                self.update_zero_flag(value);
                self.p.set(PFlag::Negative, self.fetched & 0x80 != 0);
                self.p.set(PFlag::Overflow, self.fetched & 0x40 != 0);
            }
            Opcode::BMI => todo!(),
            Opcode::BNE => todo!(),
            Opcode::BPL => todo!(),
            Opcode::BRK => todo!(),
            Opcode::BVC => todo!(),
            Opcode::BVS => todo!(),
            Opcode::CLC => self.p.remove(PFlag::Carry),
            Opcode::CLD => self.p.remove(PFlag::DecimalMode),
            Opcode::CLI => self.p.remove(PFlag::InterruptDisable),
            Opcode::CLV => self.p.remove(PFlag::Overflow),
            Opcode::CMP => {
                instruction.1.ex(self);
                let value = (self.a as u16).wrapping_sub(self.fetched as u16);
                self.update_neg_flag(value as u8);
                self.update_zero_flag(value as u8);
                self.update_carry_flag(value);
            }
            Opcode::CPX => {
                instruction.1.ex(self);
                let value = (self.x as u16).wrapping_sub(self.fetched as u16);
                self.update_carry_flag(value);
                self.update_zero_flag(value as u8);
                self.update_neg_flag(value as u8);
            }
            Opcode::CPY => {
                instruction.1.ex(self);
                let value = (self.y as u16).wrapping_sub(self.fetched as u16);
                self.update_carry_flag(value);
                self.update_neg_flag(value as u8);
                self.update_zero_flag(value as u8);
            }
            Opcode::DEC => {
                instruction.1.ex(self);
                let value = self.bus.read(self.abs_addr).wrapping_sub(1);
                self.update_neg_flag(value);
                self.update_zero_flag(value);
                self.bus.write(self.abs_addr.into(), value);
            }
            Opcode::DEX => {
                instruction.1.ex(self);
                self.x = self.x.wrapping_sub(1);
                self.update_neg_flag(self.x);
                self.update_zero_flag(self.x);
            }
            Opcode::DEY => {
                instruction.1.ex(self);
                self.y = self.y.wrapping_sub(1);
                self.update_neg_flag(self.y);
                self.update_zero_flag(self.y);
            }
            Opcode::EOR => {
                instruction.1.ex(self);
                let value = self.a ^ self.fetched;
                self.update_zero_flag(value);
                self.update_neg_flag(value);
                self.a = value;
            }
            Opcode::INC => {
                instruction.1.ex(self);
                let value = self.bus.read(self.abs_addr).wrapping_add(1);
                self.update_neg_flag(value);
                self.update_zero_flag(value);
                self.bus.write(self.abs_addr.into(), value);
            }
            Opcode::INX => {
                instruction.1.ex(self);
                self.x = self.x.wrapping_add(1);
                self.update_neg_flag(self.x);
                self.update_zero_flag(self.x);
            }
            Opcode::INY => {
                instruction.1.ex(self);
                self.y = self.y.wrapping_add(1);
                self.update_neg_flag(self.y);
                self.update_zero_flag(self.y);
            }
            Opcode::JMP => {
                instruction.1.ex(self);
                match instruction.1 {
                    AddressingMode::Absolute => {
                        self.pc = self.abs_addr;
                    }
                    AddressingMode::Indirect => {
                        let lo = self.inc_pc();
                        let hi = self.inc_pc();
                        let ptr = Mos6502::get_address_from_bytes(hi, lo);
                        let low = self.bus.read(ptr);
                        let high = if lo == 0xFF {
                            self.bus.read(ptr & 0xFF00)
                        } else {
                            self.bus.read(ptr.wrapping_add(1))
                        };
                        self.abs_addr = Mos6502::get_address_from_bytes(high, low)
                    }
                    _ => {
                        eprintln!("Unsupported addressing mode {:?}", instruction.1)
                    }
                }
            }
            Opcode::JSR => todo!(),
            Opcode::LDA => {
                instruction.1.ex(self);
                self.a = self.fetched;
                self.update_neg_flag(self.fetched);
                self.update_zero_flag(self.fetched);
            }
            Opcode::LDX => {
                instruction.1.ex(self);
                self.x = self.fetched;
                self.update_neg_flag(self.fetched);
                self.update_zero_flag(self.fetched);
            }
            Opcode::LDY => {
                instruction.1.ex(self);
                self.y = self.fetched;
                self.update_neg_flag(self.fetched);
                self.update_zero_flag(self.fetched);
            }
            Opcode::LSR => todo!(),
            Opcode::NOP => todo!(),
            Opcode::ORA => {
                instruction.1.ex(self);
                let value = self.a | self.fetched;
                self.update_zero_flag(value);
                self.update_neg_flag(value);
                self.a = value;
            }
            Opcode::PHA => {
                instruction.1.ex(self);
                self.push(self.a);
            }
            Opcode::PHP => {
                let value: u8 = (self.p.bits() as u8) | 0b00110000;
                self.push(value);
            }
            Opcode::PLA => {
                let value = self.pop();
                self.update_zero_flag(value);
                self.update_neg_flag(value);
                self.a = value;
            }
            Opcode::PLP => {
                let p = self.pop() & 0b11001111;
                self.p = PFlag::from_bits(p).unwrap();
                self.p.set(PFlag::Unused, true);
            }
            Opcode::ROL => todo!(),
            Opcode::ROR => todo!(),
            Opcode::RTI => todo!(),
            Opcode::RTS => todo!(),
            Opcode::SBC => todo!(),
            Opcode::SEC => {
                instruction.1.ex(self);
                self.p.set(PFlag::Carry, true);
            }
            Opcode::SED => {
                instruction.1.ex(self);
                self.p.set(PFlag::DecimalMode, true);
            }
            Opcode::SEI => {
                instruction.1.ex(self);
                self.p.set(PFlag::InterruptDisable, true);
            }
            Opcode::STA => {
                instruction.1.ex(self);
                self.bus.write(self.abs_addr as usize, self.a);
            }
            Opcode::STX => {
                instruction.1.ex(self);
                self.bus.write(self.abs_addr as usize, self.x);
            }
            Opcode::STY => {
                instruction.1.ex(self);
                self.bus.write(self.abs_addr as usize, self.y);
            }
            Opcode::TAX => {
                instruction.1.ex(self);
                self.x = self.a;
                self.update_neg_flag(self.x);
                self.update_zero_flag(self.x);
            }
            Opcode::TAY => {
                instruction.1.ex(self);
                self.y = self.a;
                self.update_neg_flag(self.y);
                self.update_zero_flag(self.y);
            }
            Opcode::TSX => {
                instruction.1.ex(self);
                self.x = self.sp;
                self.update_neg_flag(self.x);
                self.update_zero_flag(self.x);
            }
            Opcode::TXA => {
                instruction.1.ex(self);
                self.a = self.x;
                self.update_neg_flag(self.a);
                self.update_zero_flag(self.a);
            }
            Opcode::TXS => {
                instruction.1.ex(self);
                self.sp = self.x;
            }
            Opcode::TYA => {
                instruction.1.ex(self);
                self.a = self.y;
                self.update_neg_flag(self.a);
                self.update_zero_flag(self.a);
            }
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

    pub fn push(&mut self, value: u8) {
        let addr: u16 = 0x0100 + self.sp as u16;
        self.bus.write(addr as usize, value);
        self.sp = self.sp.wrapping_sub(1)
    }

    pub fn pop(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        let value = self.bus.read((0x0100 as u16).wrapping_add(self.sp as u16));
        return value;
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
        writeln!(f)?;
        writeln!(f, "----------------- ZERO PAGE -----------------")?;
        for (row, chunk) in self.memory.chunks(16).enumerate() {
            let row16 = row * 16;
            if row16 == 0x0100 {
                writeln!(f, "----------------- STACK -----------------")?;
            }
            if row16 == 0x0200 {
                writeln!(f, "----------------- RAM -----------------")?;
            }
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
