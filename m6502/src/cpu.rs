use std::
    fmt::{self, Display, Formatter}
;

use bitflags::bitflags;

use crate::{bus::MainBus, opcodes::{resolve_opcode, AddressingMode, Instruction, Opcode}};


pub const VECTOR_BASE: u8 = 0xFF;
pub const RESET_VECTOR: u8 = 0xFC;

#[derive(Debug)]
pub enum CpuError {
    InvalidOpcode(u8),
    // other error variants...
}

#[derive(Debug)]
pub struct Mos6502 {
    pub bus: MainBus,

    //CPU registers
    pub pc: u16,
    pub p: PFlag,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,

    //helpers
    pub fetched: u8,
    pub rel_addr: u16,
    pub abs_addr: u16,
}
bitflags! {
    #[derive(Debug)]
    pub struct PFlag: u8 {
       const Carry =            0b0000_0001;
       const Zero =             0b0000_0010;
       const InterruptDisable = 0b0000_0100;
       const DecimalMode =      0b0000_1000;
       const BreakCommand =     0b0001_0000;
       const Unused =           0b0010_0000;
       const Overflow =         0b0100_0000;
       const Negative =         0b1000_0000;
    }
}

impl Mos6502 {
    pub fn new(bus: MainBus) -> Mos6502 {
        Mos6502 {
            a: 0,
            p: PFlag::from_bits(36).unwrap(),
            pc: 0,
            sp: 0xFD,
            x: 0,
            y: 0,
            bus,

            //helpers
            fetched: 0,
            abs_addr: 0,
            rel_addr: 0,
        }
    }

    pub fn zero_page(&self) -> &[u8] {
        return &self.bus.cpu_ram[0x0000..0x00FF];
    }

    pub fn dump(&self) {
        println!("{}", self)
    }

    pub fn nmi(&mut self) {
        self.push((self.pc >> 8) as u8);
        self.push(self.pc as u8);
        let mut flags = self.p.bits();
        flags &= !(1 << 4);
        self.push(flags);

        self.p.set(PFlag::InterruptDisable, true);
        let lo = self.bus.read(0xFFFA);
        let hi = self.bus.read(0xFFFB);
        self.pc = Mos6502::get_address_from_bytes(hi, lo);
    }

    pub fn irq(&mut self) {
        if self.p.contains(PFlag::InterruptDisable) {
            return;
        }
        self.push((self.pc >> 8) as u8);
        self.push(self.pc as u8);
        let mut flags = self.p.bits();
        flags &= !(1 << 4);
        self.push(flags);
        self.p.set(PFlag::InterruptDisable, true);
        let lo = self.bus.read(0xFFFE);
        let hi = self.bus.read(0xFFFF);
        self.pc = Mos6502::get_address_from_bytes(hi, lo)
    }

    pub fn reset(&mut self) {
        let address = ((VECTOR_BASE as u16) << 8) | RESET_VECTOR as u16;
        let lo = self.bus.read(address);
        let hi = self.bus.read(address + 1);
        self.pc = Mos6502::get_address_from_bytes(hi, lo);
        self.sp = 0xFD;
    }

    //little endian low-order byte
    pub fn get_address_from_bytes(hi: u8, lo: u8) -> u16 {
        u16::from(lo) + (u16::from(hi) << 8usize)
    }

    pub fn fetch(&mut self) -> Result<(Instruction), CpuError> {
        let opcode = self.bus.read(self.pc);
        let resolved = resolve_opcode(opcode);
        match resolved {
            Some(instruction) => Ok(instruction),
            None => Err(CpuError::InvalidOpcode(opcode)),
        }
    }

    fn update_zero_flag(&mut self, value: u8) {
        self.p.set(PFlag::Zero, value == 0);
    }
    fn update_neg_flag(&mut self, value: u8) {
        self.p.set(PFlag::Negative, value & 0x80 != 0);
    }
    fn update_overflow_flag(&mut self, a: u8, operand: u8, sum: u8) {
        self.p
            .set(PFlag::Overflow, ((!(a ^ operand) & (a ^ sum)) & 0x80) != 0)
    }

    fn update_carry_flag(&mut self, value: u16) {
        self.p.set(PFlag::Carry, value > 0xFF);
    }

    pub fn execute(&mut self, instruction: Instruction) {
        self.pc += 1;
        match instruction.0 {
            Opcode::ADC => {
                instruction.1.apply(self);
                let carrying = if self.p.contains(PFlag::Carry) { 1 } else { 0 };
                let v1 = self.fetched as u16 + carrying;
                let sum: u16 = self.a as u16 + v1;
                let a = self.a as u16;
                let result: u8 = sum as u8;
                self.update_zero_flag(result);
                self.update_neg_flag(result);
                self.update_carry_flag(sum);
                self.update_overflow_flag(a as u8, self.fetched, sum as u8);
                self.a = sum as u8;
            }
            Opcode::AND => {
                instruction.1.apply(self);
                let value = self.a & self.fetched;
                self.update_zero_flag(value);
                self.update_neg_flag(value);
                self.a = value;
            }
            Opcode::ASL => {
                let mut value: u16;
                if instruction.1 == AddressingMode::Implied
                    || instruction.1 == AddressingMode::Accumulator
                {
                    value = self.a as u16;
                } else {
                    instruction.1.apply(self);
                    value = self.fetched as u16;
                }
                value = value << 1;
                self.update_zero_flag(value as u8);
                self.update_neg_flag(value as u8);
                self.p.set(PFlag::Carry, (value & 0x100) != 0);
                if instruction.1 == AddressingMode::Implied
                    || instruction.1 == AddressingMode::Accumulator
                {
                    self.a = value as u8
                } else {
                    self.bus.write(self.abs_addr as usize, value as u8);
                }
            }
            Opcode::BCC => {
                instruction.1.apply(self);
                if !self.p.contains(PFlag::Carry) {
                    let offset = self.fetched as i8 as i32;
                    self.pc = (self.pc as i32 + offset) as u16;
                }
            }
            Opcode::BCS => {
                instruction.1.apply(self);
                if self.p.contains(PFlag::Carry) {
                    let offset = self.fetched as i8 as i32;
                    self.pc = (self.pc as i32 + offset) as u16;
                }
            }
            Opcode::BEQ => {
                instruction.1.apply(self);
                if self.p.contains(PFlag::Zero) {
                    let offset = self.fetched as i8 as i32;
                    self.pc = (self.pc as i32 + offset) as u16;
                }
            }
            Opcode::BIT => {
                instruction.1.apply(self);
                let value = self.a & self.fetched;

                self.p.set(PFlag::Zero, value == 0);
                self.p.set(PFlag::Negative, self.fetched & 0x80 != 0);
                self.p.set(PFlag::Overflow, self.fetched & 0x40 != 0);
            }
            Opcode::BMI => {
                instruction.1.apply(self);
                if self.p.contains(PFlag::Negative) {
                    let offset = self.fetched as i8 as i32;
                    self.pc = (self.pc as i32 + offset) as u16;
                }
            }
            Opcode::BNE => {
                instruction.1.apply(self);
                if !self.p.contains(PFlag::Zero) {
                    let offset = self.fetched as i8 as i32;
                    self.pc = (self.pc as i32 + offset) as u16;
                }
            }
            Opcode::BPL => {
                instruction.1.apply(self);
                if !self.p.contains(PFlag::Negative) {
                    let offset = self.fetched as i8 as i32;
                    self.pc = (self.pc as i32 + offset) as u16;
                }
            }
            Opcode::BRK => {
                instruction.1.apply(self);
                let ret_address = self.pc;
                let high_byte: u8 = (ret_address >> 8) as u8;
                let low_byte: u8 = (ret_address & 0xFF) as u8;
                self.push(low_byte);
                self.push(high_byte);
                let reg = self.p.bits() | PFlag::BreakCommand.bits() | PFlag::Unused.bits();
                self.push(reg);
                self.p.set(PFlag::InterruptDisable, true);
                let lo = self.bus.read(0xFFFE);
                let hi = self.bus.read(0xFFFF);
                self.pc = Mos6502::get_address_from_bytes(hi, lo)
            }
            Opcode::BVC => {
                instruction.1.apply(self);
                if !self.p.contains(PFlag::Overflow) {
                    let offset = self.fetched as i8 as i32;
                    self.pc = (self.pc as i32 + offset) as u16;
                }
            }
            Opcode::BVS => {
                instruction.1.apply(self);
                if self.p.contains(PFlag::Overflow) {
                    let offset = self.fetched as i8 as i32;
                    self.pc = (self.pc as i32 + offset) as u16;
                }
            }
            Opcode::CLC => self.p.remove(PFlag::Carry),
            Opcode::CLD => self.p.remove(PFlag::DecimalMode),
            Opcode::CLI => self.p.remove(PFlag::InterruptDisable),
            Opcode::CLV => self.p.remove(PFlag::Overflow),
            Opcode::DCP => {
                instruction.1.apply(self);
                let decremented = self.bus.read(self.abs_addr).wrapping_sub(1);
                self.bus.write(self.abs_addr as usize, decremented);
                let result = self.a.wrapping_sub(decremented);
                self.update_neg_flag(result);
                self.update_zero_flag(result);
                self.p.set(PFlag::Carry, self.a >= decremented);
            }
            Opcode::CMP => {
                instruction.1.apply(self);
                let value = (self.a as u16).wrapping_sub(self.fetched as u16);
                self.update_neg_flag(value as u8);
                self.update_zero_flag(value as u8);
                self.p.set(PFlag::Carry, self.a >= self.fetched as u8);
            }
            Opcode::CPX => {
                instruction.1.apply(self);
                let value = (self.x as u16).wrapping_sub(self.fetched as u16);
                self.p.set(PFlag::Carry, self.x >= self.fetched as u8);
                self.update_zero_flag(value as u8);
                self.update_neg_flag(value as u8);
            }
            Opcode::CPY => {
                instruction.1.apply(self);
                let value = (self.y as u16).wrapping_sub(self.fetched as u16);
                self.p.set(PFlag::Carry, self.y >= self.fetched as u8);
                self.update_neg_flag(value as u8);
                self.update_zero_flag(value as u8);
            }
            Opcode::DEC => {
                instruction.1.apply(self);
                let value = self.bus.read(self.abs_addr).wrapping_sub(1);
                self.update_neg_flag(value);
                self.update_zero_flag(value);
                self.bus.write(self.abs_addr.into(), value);
            }
            Opcode::DEX => {
                instruction.1.apply(self);
                self.x = self.x.wrapping_sub(1);
                self.update_neg_flag(self.x);
                self.update_zero_flag(self.x);
            }
            Opcode::DEY => {
                instruction.1.apply(self);
                self.y = self.y.wrapping_sub(1);
                self.update_neg_flag(self.y);
                self.update_zero_flag(self.y);
            }
            Opcode::EOR => {
                instruction.1.apply(self);
                let value = self.a ^ self.fetched;
                self.update_zero_flag(value);
                self.update_neg_flag(value);
                self.a = value;
            }
            Opcode::INC => {
                instruction.1.apply(self);
                let value = self.bus.read(self.abs_addr).wrapping_add(1);
                self.update_neg_flag(value);
                self.update_zero_flag(value);
                self.bus.write(self.abs_addr.into(), value);
            }
            Opcode::ISB => {
                instruction.1.apply(self);

                let fetched = self.bus.read(self.abs_addr).wrapping_add(1);
                self.bus.write(self.abs_addr as usize, fetched);

                let value = fetched ^ 0xFF;
                let carry_in = if self.p.contains(PFlag::Carry) { 1 } else { 0 };

                let sum = self.a as u16 + value as u16 + carry_in;

                self.update_carry_flag(sum);
                self.update_overflow_flag(value as u8, self.a, sum as u8);

                self.a = sum as u8;
                self.update_zero_flag(self.a);
                self.update_neg_flag(self.a);
            }
            Opcode::INX => {
                instruction.1.apply(self);
                self.x = self.x.wrapping_add(1);
                self.update_neg_flag(self.x);
                self.update_zero_flag(self.x);
            }
            Opcode::INY => {
                instruction.1.apply(self);
                self.y = self.y.wrapping_add(1);
                self.update_neg_flag(self.y);
                self.update_zero_flag(self.y);
            }
            Opcode::JMP => {
                instruction.1.apply(self);
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
                        self.abs_addr = Mos6502::get_address_from_bytes(high, low);
                        self.pc = self.abs_addr;
                    }
                    _ => {
                        eprintln!("Unsupported addressing mode {:?}", instruction.1)
                    }
                }
            }
            Opcode::JSR => {
                instruction.1.apply(self);
                let ret_address = self.pc.wrapping_sub(1);
                let high_byte: u8 = (ret_address >> 8) as u8;
                let low_byte: u8 = (ret_address & 0xFF) as u8;
                self.push(high_byte);
                self.push(low_byte);
                self.pc = self.abs_addr;
            }
            Opcode::LDA => {
                instruction.1.apply(self);
                self.a = self.fetched;
                self.update_neg_flag(self.fetched);
                self.update_zero_flag(self.fetched);
            }
            Opcode::LDX => {
                instruction.1.apply(self);
                self.x = self.fetched;
                self.update_neg_flag(self.fetched);
                self.update_zero_flag(self.fetched);
            }
            Opcode::LAX => {
                instruction.1.apply(self);
                self.x = self.fetched;
                self.a = self.fetched;
                self.update_neg_flag(self.fetched);
                self.update_zero_flag(self.fetched);
            }
            Opcode::LDY => {
                instruction.1.apply(self);
                self.y = self.fetched;
                self.update_neg_flag(self.fetched);
                self.update_zero_flag(self.fetched);
            }
            Opcode::LSR => {
                instruction.1.apply(self);
                self.p.set(PFlag::Carry, self.fetched & 0x01 != 0);
                let temp = self.fetched as u16 >> 1;
                self.p.set(PFlag::Negative, false);
                if instruction.1 == AddressingMode::Implied
                    || instruction.1 == AddressingMode::Accumulator
                {
                    self.a = temp as u8
                } else {
                    self.bus.write(self.abs_addr as usize, temp as u8);
                }
                self.update_zero_flag(temp as u8);
            }
            Opcode::NOP => {
                instruction.1.apply(self);
            }
            Opcode::UndocumentedNOP => {
                instruction.1.apply(self);
            }
            Opcode::ORA => {
                instruction.1.apply(self);
                let value = self.a | self.fetched;
                self.update_zero_flag(value);
                self.update_neg_flag(value);
                self.a = value;
            }
            Opcode::PHA => {
                instruction.1.apply(self);
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
            Opcode::ROL => {
                instruction.1.apply(self);
                let carry_in: u16 = if self.p.contains(PFlag::Carry) { 1 } else { 0 };
                let carry_out = (self.fetched & 0b10000000) != 0;
                let temp = (self.fetched as u16) << 1 | carry_in;
                self.p.set(PFlag::Negative, (temp & 0x80) != 0);
                self.p.set(PFlag::Carry, carry_out);
                if instruction.1 == AddressingMode::Implied
                    || instruction.1 == AddressingMode::Accumulator
                {
                    self.a = temp as u8
                } else {
                    self.bus.write(self.abs_addr as usize, temp as u8);
                }
                self.update_zero_flag(temp as u8);
            }
            Opcode::ROR => {
                instruction.1.apply(self);
                let carry_in: u8 = if self.p.contains(PFlag::Carry) { 1 } else { 0 };
                let carry_out = (self.fetched & 0x01) != 0;
                let temp = (self.fetched) >> 1 | (carry_in << 7);
                self.p.set(PFlag::Negative, (temp & 0x80) != 0);
                self.p.set(PFlag::Carry, carry_out);
                if instruction.1 == AddressingMode::Implied
                    || instruction.1 == AddressingMode::Accumulator
                {
                    self.a = temp as u8
                } else {
                    self.bus.write(self.abs_addr as usize, temp as u8);
                }
                self.update_zero_flag(temp as u8);
            }
            Opcode::RTI => {
                instruction.1.apply(self);
                let registers = self.pop();
                let pc_l = self.pop();
                let pc_h = self.pop();
                self.p = PFlag::from_bits(registers).unwrap();
                self.p.set(PFlag::Unused, true);
                self.pc = Mos6502::get_address_from_bytes(pc_h, pc_l)
            }
            Opcode::RTS => {
                instruction.1.apply(self);
                let low_byte = self.pop();
                let high_byte = self.pop();
                self.pc = Mos6502::get_address_from_bytes(high_byte, low_byte).wrapping_add(1)
            }
            Opcode::SBC => {
                instruction.1.apply(self);

                let value = self.fetched ^ 0xFF;
                let carry_in = if self.p.contains(PFlag::Carry) { 1 } else { 0 };

                let sum = self.a as u16 + value as u16 + carry_in;

                self.update_carry_flag(sum);
                self.update_overflow_flag(value as u8, self.a, sum as u8);

                self.a = sum as u8;
                self.update_zero_flag(self.a);
                self.update_neg_flag(self.a);
            }
            Opcode::SEC => {
                instruction.1.apply(self);
                self.p.set(PFlag::Carry, true);
            }
            Opcode::SED => {
                instruction.1.apply(self);
                self.p.set(PFlag::DecimalMode, true);
            }
            Opcode::SEI => {
                instruction.1.apply(self);
                self.p.set(PFlag::InterruptDisable, true);
            }
            Opcode::STA => {
                instruction.1.apply(self);
                self.bus.write(self.abs_addr as usize, self.a);
            }
            Opcode::STX => {
                instruction.1.apply(self);
                self.bus.write(self.abs_addr as usize, self.x);
            }
            Opcode::SAX => {
                instruction.1.apply(self);
                self.bus.write(self.abs_addr as usize, self.x & self.a);
            }
            Opcode::SLO => {
                instruction.1.apply(self);
                self.p.set(PFlag::Carry, self.fetched & 0x80 != 0);

                let shifted = (self.fetched << 1) as u8;

                if instruction.1 == AddressingMode::Implied
                    || instruction.1 == AddressingMode::Accumulator
                {
                    self.a = shifted;
                } else {
                    self.bus.write(self.abs_addr as usize, shifted);
                }

                self.a |= shifted;

                self.update_zero_flag(self.a);
                self.update_neg_flag(self.a);
            }
            Opcode::STY => {
                instruction.1.apply(self);
                self.bus.write(self.abs_addr as usize, self.y);
            }
            Opcode::TAX => {
                instruction.1.apply(self);
                self.x = self.a;
                self.update_neg_flag(self.x);
                self.update_zero_flag(self.x);
            }
            Opcode::TAY => {
                instruction.1.apply(self);
                self.y = self.a;
                self.update_neg_flag(self.y);
                self.update_zero_flag(self.y);
            }
            Opcode::TSX => {
                instruction.1.apply(self);
                self.x = self.sp;
                self.update_neg_flag(self.x);
                self.update_zero_flag(self.x);
            }
            Opcode::TXA => {
                instruction.1.apply(self);
                self.a = self.x;
                self.update_neg_flag(self.a);
                self.update_zero_flag(self.a);
            }
            Opcode::TXS => {
                instruction.1.apply(self);
                self.sp = self.x;
            }
            Opcode::TYA => {
                instruction.1.apply(self);
                self.a = self.y;
                self.update_neg_flag(self.a);
                self.update_zero_flag(self.a);
            }
            Opcode::RLA => {
                instruction.1.apply(self);
                let carry_in: u16 = if self.p.contains(PFlag::Carry) { 1 } else { 0 };
                let carry_out = (self.fetched & 0b10000000) != 0;
                let temp = (self.fetched as u16) << 1 | carry_in;
                self.p.set(PFlag::Negative, (temp & 0x80) != 0);
                self.p.set(PFlag::Carry, carry_out);
                self.bus.write(self.abs_addr as usize, temp as u8);
                self.a &= temp as u8;

                self.update_zero_flag(temp as u8);
            }
            Opcode::SRE => {
                instruction.1.apply(self);
                self.p.set(PFlag::Carry, self.fetched & 0x01 != 0);
                let shifted = (self.fetched >> 1);
                self.p.set(PFlag::Negative, false);
                if instruction.1 != AddressingMode::Implied
                    && instruction.1 != AddressingMode::Accumulator
                {
                    self.bus.write(self.abs_addr as usize, shifted);
                }
                self.a ^= shifted;
                self.update_neg_flag(self.a);
                self.update_zero_flag(self.a);
            }
            Opcode::RRA => {
                instruction.1.apply(self);
                let carry_in: u8 = if self.p.contains(PFlag::Carry) { 1 } else { 0 };
                let carry_out = (self.fetched & 0x01) != 0;
                let rotated = (self.fetched) >> 1 | (carry_in << 7);
                self.p.set(PFlag::Negative, (rotated & 0x80) != 0);
                self.p.set(PFlag::Carry, carry_out);
                if instruction.1 != AddressingMode::Implied
                    && instruction.1 != AddressingMode::Accumulator
                {
                    self.bus.write(self.abs_addr as usize, rotated as u8);
                }
                let a = self.a;
                let result =
                    a as u16 + rotated as u16 + if self.p.contains(PFlag::Carry) { 1 } else { 0 };

                self.p.set(PFlag::Carry, result > 0xFF);
                let result8 = result as u8;
                self.p.set(
                    PFlag::Overflow,
                    ((a ^ result8) & (rotated ^ result8) & 0x80) != 0,
                );
                self.a = result8;
                self.update_zero_flag(self.a);
                self.update_neg_flag(self.a);
            }
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
        writeln!(
            f,
            "│  P: 0b{:08b}  ({:?})  ({:02X}). │",
            self.p,
            self.p,
            self.p.bits()
        )?;

        // Footer
        writeln!(f, "└──────────────────────────────────┘")
    }
}

impl Display for MainBus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "+---------------- 2 KiB RAM ----------------+")?;
        writeln!(f)?;
        writeln!(f, "----------------- ZERO PAGE -----------------")?;
        for (row, chunk) in self.cpu_ram.chunks(16).enumerate() {
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
