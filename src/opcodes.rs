use std::ops::Add;

use crate::m6502::Mos6502;
pub type Instruction = (Opcode, AddressingMode);

//add with carry
pub fn adc(cpu: &mut Mos6502, am: AddressingMode) {}

//and (with accumulator)
pub fn and(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//arithmetic shift left
pub fn asl(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//branch on carry clear
pub fn bcc(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//branch on carry set
pub fn bcs(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//branch on equal (zero set)
pub fn beq(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//bit test
pub fn bit(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//branch on minus (negative set)
pub fn bmi(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//branch on not equal (zero clear)
pub fn bne(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//branch on plus (negative clear)
pub fn bpl(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//break / interrupt
pub fn brk(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//branch on overflow clear
pub fn bvc(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//branch on overflow set
pub fn bvs(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//clear carry
pub fn clc(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//clear decimal
pub fn cld(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//clear interrupt disable
pub fn cli(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//clear overflow
pub fn clv(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//compare (with accumulator)
pub fn cmp(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//compare with X
pub fn cpx(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//compare with Y
pub fn cpy(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//decrement
pub fn dec(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//decrement X
pub fn dex(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//decrement Y
pub fn dey(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//exclusive or (with accumulator)
pub fn eor(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//increment
pub fn inc(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//increment X
pub fn inx(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//increment Y
pub fn iny(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//jump
pub fn jmp(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//jump subroutine
pub fn jsr(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//load accumulator
pub fn lda(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//load X
pub fn ldx(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//load Y
pub fn ldy(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//logical shift right
pub fn lsr(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//no operation
pub fn nop(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//or with accumulator
pub fn ora(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//push accumulator
pub fn pha(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//push processor status (SR)
pub fn php(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//pull accumulator
pub fn pla(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//pull processor status (SR)
pub fn plp(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//rotate left
pub fn rol(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//rotate right
pub fn ror(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//return from interrupt
pub fn rti(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//return from subroutine
pub fn rts(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//subtract with carry
pub fn sbc(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//set carry
pub fn sec(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//set decimal
pub fn sed(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//set interrupt disable
pub fn sei(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//store accumulator
pub fn sta(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//store X
pub fn stx(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//store Y
pub fn sty(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//transfer accumulator to X
pub fn tax(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//transfer accumulator to Y
pub fn tay(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//transfer stack pointer to X
pub fn tsx(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//transfer X to accumulator
pub fn txa(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//transfer X to stack pointer
pub fn txs(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

//transfer Y to accumulator
pub fn tya(cpu: &mut Mos6502, am: AddressingMode) {
    todo!();
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AddressingMode {
    // 1-bytes
    BuggyIndirect,
    Implied,
    Accumulator,

    // 2-bytes
    Immediate,
    ZeroPage,
    XIndexedZeroPage, // ($zp,X)
    YIndexedZeroPage, // ($zp),Y
    XIndexedZeroPageIndirect,
    ZeroPageIndirectYIndexed,
    ZeroPageIndirectXIndexed,
    Relative,

    // 3-bytes
    Absolute,
    XIndexedAbsolute,
    YIndexedAbsolute,
    AbsoluteIndirect,
}

impl AddressingMode {
    pub fn get_bytes(self) -> u8 {
        use AddressingMode::*;
        match self {
            Implied | Accumulator => 1,
            Immediate
            | ZeroPage
            | XIndexedZeroPageIndirect
            | XIndexedZeroPage
            | YIndexedZeroPage
            | ZeroPageIndirectXIndexed
            | ZeroPageIndirectYIndexed
            | Relative => 2,
            Absolute | XIndexedAbsolute | YIndexedAbsolute | AbsoluteIndirect => 3,
            BuggyIndirect => todo!(),
        }
    }
    pub fn ex(self, cpu: &mut Mos6502) {
        use AddressingMode::*;
        match self {
            Implied => {}
            Accumulator => {
                cpu.operand = cpu.a;
            }
            Immediate => {
                let operand = cpu.inc_pc();
                cpu.operand = operand;
            }
            ZeroPage => {
                let zero_page_addr = cpu.inc_pc() as u16;
                cpu.abs_addr = zero_page_addr;
            }
            XIndexedZeroPage => {
                let zero_page_addr = cpu.inc_pc().wrapping_add(cpu.x);
                cpu.abs_addr = zero_page_addr as u16;
            }
            YIndexedZeroPage => {
                let zero_page_addr = cpu.inc_pc().wrapping_add(cpu.y);
                cpu.abs_addr = zero_page_addr as u16;
            }
            XIndexedZeroPageIndirect => todo!(),
            ZeroPageIndirectYIndexed => {
                let zero_page_addr = cpu.inc_pc().wrapping_add(cpu.y);
                cpu.abs_addr = zero_page_addr as u16;
            }
            Relative => todo!(),
            Absolute => {
                let hh = cpu.inc_pc();
                let lo: u8 = cpu.inc_pc();
                cpu.abs_addr = Mos6502::get_address_from_bytes(hh, lo);
            }
            XIndexedAbsolute => {
                let hh = cpu.inc_pc();
                let lo = cpu.inc_pc();
                cpu.abs_addr = Mos6502::get_address_from_bytes(hh, lo.add(cpu.x));
            }
            YIndexedAbsolute => {
                let hh = cpu.inc_pc();
                let lo = cpu.inc_pc();
                cpu.abs_addr = Mos6502::get_address_from_bytes(hh, lo.add(cpu.y));
            }
            AbsoluteIndirect => todo!(),
            BuggyIndirect => todo!(),
            ZeroPageIndirectXIndexed => todo!(),
        }
    }
}
pub fn resolve_opcode(n: u8) -> Option<Instruction> {
    match n {
        0x00 => Some((Opcode::BRK, AddressingMode::Implied)),
        0x01 => Some((Opcode::ORA, AddressingMode::ZeroPageIndirectXIndexed)),
        0x02 => None,
        0x03 => None,
        0x04 => None,
        0x05 => Some((Opcode::ORA, AddressingMode::ZeroPage)),
        0x06 => Some((Opcode::ASL, AddressingMode::ZeroPage)),
        0x07 => None,
        0x08 => Some((Opcode::PHP, AddressingMode::Implied)),
        0x09 => Some((Opcode::ORA, AddressingMode::Immediate)),
        0x0a => Some((Opcode::ASL, AddressingMode::Accumulator)),
        0x0b => None,
        0x0c => None,
        0x0d => Some((Opcode::ORA, AddressingMode::Absolute)),
        0x0e => Some((Opcode::ASL, AddressingMode::Absolute)),
        0x0f => None,
        0x10 => Some((Opcode::BPL, AddressingMode::Relative)),
        0x11 => Some((Opcode::ORA, AddressingMode::ZeroPageIndirectXIndexed)),
        0x12 => None,
        0x13 => None,
        0x14 => None,
        0x15 => Some((Opcode::ORA, AddressingMode::XIndexedZeroPage)),
        0x16 => Some((Opcode::ASL, AddressingMode::XIndexedZeroPage)),
        0x17 => None,
        0x18 => Some((Opcode::CLC, AddressingMode::Implied)),
        0x19 => Some((Opcode::ORA, AddressingMode::YIndexedAbsolute)),
        0x1a => None,
        0x1b => None,
        0x1c => None,
        0x1d => Some((Opcode::ORA, AddressingMode::XIndexedAbsolute)),
        0x1e => Some((Opcode::ASL, AddressingMode::XIndexedAbsolute)),
        0x1f => None,
        0x20 => Some((Opcode::JSR, AddressingMode::Absolute)),
        0x21 => Some((Opcode::AND, AddressingMode::ZeroPageIndirectXIndexed)),
        0x22 => None,
        0x23 => None,
        0x24 => Some((Opcode::BIT, AddressingMode::ZeroPage)),
        0x25 => Some((Opcode::AND, AddressingMode::ZeroPage)),
        0x26 => Some((Opcode::ROL, AddressingMode::ZeroPage)),
        0x27 => None,
        0x28 => Some((Opcode::PLP, AddressingMode::Implied)),
        0x29 => Some((Opcode::AND, AddressingMode::Immediate)),
        0x2a => Some((Opcode::ROL, AddressingMode::Accumulator)),
        0x2b => None,
        0x2c => Some((Opcode::BIT, AddressingMode::Absolute)),
        0x2d => Some((Opcode::AND, AddressingMode::Absolute)),
        0x2e => Some((Opcode::ROL, AddressingMode::Absolute)),
        0x2f => None,
        0x30 => Some((Opcode::BMI, AddressingMode::Relative)),
        0x31 => Some((Opcode::AND, AddressingMode::ZeroPageIndirectXIndexed)),
        0x32 => None,
        0x33 => None,
        0x34 => None,
        0x35 => Some((Opcode::AND, AddressingMode::XIndexedZeroPage)),
        0x36 => Some((Opcode::ROL, AddressingMode::XIndexedZeroPage)),
        0x37 => None,
        0x38 => Some((Opcode::SEC, AddressingMode::Implied)),
        0x39 => Some((Opcode::AND, AddressingMode::YIndexedAbsolute)),
        0x3a => None,
        0x3b => None,
        0x3c => None,
        0x3d => Some((Opcode::AND, AddressingMode::XIndexedAbsolute)),
        0x3e => Some((Opcode::ROL, AddressingMode::XIndexedAbsolute)),
        0x3f => None,
        0x40 => Some((Opcode::RTI, AddressingMode::Implied)),
        0x41 => Some((Opcode::EOR, AddressingMode::ZeroPageIndirectXIndexed)),
        0x42 => None,
        0x43 => None,
        0x44 => None,
        0x45 => Some((Opcode::EOR, AddressingMode::ZeroPage)),
        0x46 => Some((Opcode::LSR, AddressingMode::ZeroPage)),
        0x47 => None,
        0x48 => Some((Opcode::PHA, AddressingMode::Implied)),
        0x49 => Some((Opcode::EOR, AddressingMode::Immediate)),
        0x4a => Some((Opcode::LSR, AddressingMode::Accumulator)),
        0x4b => None,
        0x4c => Some((Opcode::JMP, AddressingMode::Absolute)),
        0x4d => Some((Opcode::EOR, AddressingMode::Absolute)),
        0x4e => Some((Opcode::LSR, AddressingMode::Absolute)),
        0x4f => None,
        0x50 => Some((Opcode::BVC, AddressingMode::Relative)),
        0x51 => Some((Opcode::EOR, AddressingMode::ZeroPageIndirectYIndexed)),
        0x52 => None,
        0x53 => None,
        0x54 => None,
        0x55 => Some((Opcode::EOR, AddressingMode::XIndexedZeroPage)),
        0x56 => Some((Opcode::LSR, AddressingMode::XIndexedZeroPage)),
        0x57 => None,
        0x58 => Some((Opcode::CLI, AddressingMode::Implied)),
        0x59 => Some((Opcode::EOR, AddressingMode::YIndexedAbsolute)),
        0x5a => None,
        0x5b => None,
        0x5c => None,
        0x5d => Some((Opcode::EOR, AddressingMode::XIndexedAbsolute)),
        0x5e => Some((Opcode::LSR, AddressingMode::XIndexedAbsolute)),
        0x5f => None,
        0x60 => Some((Opcode::RTS, AddressingMode::Implied)),
        0x61 => Some((Opcode::ADC, AddressingMode::XIndexedZeroPageIndirect)),
        0x62 => None,
        0x63 => None,
        0x64 => None,
        0x65 => Some((Opcode::ADC, AddressingMode::ZeroPage)),
        0x66 => Some((Opcode::ROR, AddressingMode::ZeroPage)),
        0x67 => None,
        0x68 => Some((Opcode::PLA, AddressingMode::Implied)),
        0x69 => Some((Opcode::ADC, AddressingMode::Immediate)),
        0x6a => Some((Opcode::ROR, AddressingMode::Accumulator)),
        0x6b => None,
        0x6c => Some((Opcode::JMP, AddressingMode::BuggyIndirect)),
        0x6d => Some((Opcode::ADC, AddressingMode::Absolute)),
        0x6e => Some((Opcode::ROR, AddressingMode::Absolute)),
        0x6f => None,
        0x70 => Some((Opcode::BVS, AddressingMode::Relative)),
        0x71 => Some((Opcode::ADC, AddressingMode::ZeroPageIndirectXIndexed)),
        0x72 => None,
        0x73 => None,
        0x74 => None,
        0x75 => Some((Opcode::ADC, AddressingMode::XIndexedZeroPage)),
        0x76 => Some((Opcode::ROR, AddressingMode::XIndexedZeroPage)),
        0x77 => None,
        0x78 => Some((Opcode::SEI, AddressingMode::Implied)),
        0x79 => Some((Opcode::ADC, AddressingMode::YIndexedAbsolute)),
        0x7a => None,
        0x7b => None,
        0x7c => None,
        0x7d => Some((Opcode::ADC, AddressingMode::XIndexedAbsolute)),
        0x7e => Some((Opcode::ROR, AddressingMode::XIndexedAbsolute)),
        0x7f => None,
        0x80 => None,
        0x81 => Some((Opcode::STA, AddressingMode::ZeroPageIndirectXIndexed)),
        0x82 => None,
        0x83 => None,
        0x84 => Some((Opcode::STY, AddressingMode::ZeroPage)),
        0x85 => Some((Opcode::STA, AddressingMode::ZeroPage)),
        0x86 => Some((Opcode::STX, AddressingMode::ZeroPage)),
        0x87 => None,
        0x88 => Some((Opcode::DEY, AddressingMode::Implied)),
        0x89 => None,
        0x8a => Some((Opcode::TXA, AddressingMode::Implied)),
        0x8b => None,
        0x8c => Some((Opcode::STY, AddressingMode::Absolute)),
        0x8d => Some((Opcode::STA, AddressingMode::Absolute)),
        0x8e => Some((Opcode::STX, AddressingMode::Absolute)),
        0x8f => None,
        0x90 => Some((Opcode::BCC, AddressingMode::Relative)),
        0x91 => Some((Opcode::STA, AddressingMode::ZeroPageIndirectXIndexed)),
        0x92 => None,
        0x93 => None,
        0x94 => Some((Opcode::STY, AddressingMode::XIndexedZeroPage)),
        0x95 => Some((Opcode::STA, AddressingMode::XIndexedZeroPage)),
        0x96 => Some((Opcode::STX, AddressingMode::YIndexedZeroPage)),
        0x97 => None,
        0x98 => Some((Opcode::TYA, AddressingMode::Implied)),
        0x99 => Some((Opcode::STA, AddressingMode::YIndexedAbsolute)),
        0x9a => Some((Opcode::TXS, AddressingMode::Implied)),
        0x9b => None,
        0x9c => None,
        0x9d => Some((Opcode::STA, AddressingMode::XIndexedAbsolute)),
        0x9e => None,
        0x9f => None,
        0xa0 => Some((Opcode::LDY, AddressingMode::Immediate)),
        0xa1 => Some((Opcode::LDA, AddressingMode::ZeroPageIndirectXIndexed)),
        0xa2 => Some((Opcode::LDX, AddressingMode::Immediate)),
        0xa3 => None,
        0xa4 => Some((Opcode::LDY, AddressingMode::ZeroPage)),
        0xa5 => Some((Opcode::LDA, AddressingMode::ZeroPage)),
        0xa6 => Some((Opcode::LDX, AddressingMode::ZeroPage)),
        0xa7 => None,
        0xa8 => Some((Opcode::TAY, AddressingMode::Implied)),
        0xa9 => Some((Opcode::LDA, AddressingMode::Immediate)),
        0xaa => Some((Opcode::TAX, AddressingMode::Implied)),
        0xab => None,
        0xac => Some((Opcode::LDY, AddressingMode::Absolute)),
        0xad => Some((Opcode::LDA, AddressingMode::Absolute)),
        0xae => Some((Opcode::LDX, AddressingMode::Absolute)),
        0xaf => None,
        0xb0 => Some((Opcode::BCS, AddressingMode::Relative)),
        0xb1 => Some((Opcode::LDA, AddressingMode::ZeroPageIndirectYIndexed)),
        0xb2 => None,
        0xb3 => None,
        0xb4 => Some((Opcode::LDY, AddressingMode::XIndexedZeroPage)),
        0xb5 => Some((Opcode::LDA, AddressingMode::XIndexedZeroPage)),
        0xb6 => Some((Opcode::LDX, AddressingMode::YIndexedZeroPage)),
        0xb7 => None,
        0xb8 => Some((Opcode::CLV, AddressingMode::Implied)),
        0xb9 => Some((Opcode::LDA, AddressingMode::YIndexedAbsolute)),
        0xba => Some((Opcode::TSX, AddressingMode::Implied)),
        0xbb => None,
        0xbc => Some((Opcode::LDY, AddressingMode::XIndexedAbsolute)),
        0xbd => Some((Opcode::LDA, AddressingMode::XIndexedAbsolute)),
        0xbe => Some((Opcode::LDX, AddressingMode::YIndexedAbsolute)),
        0xbf => None,
        0xc0 => Some((Opcode::CPY, AddressingMode::Immediate)),
        0xc1 => Some((Opcode::CMP, AddressingMode::ZeroPageIndirectXIndexed)),
        0xc2 => None,
        0xc3 => None,
        0xc4 => Some((Opcode::CPY, AddressingMode::ZeroPage)),
        0xc5 => Some((Opcode::CMP, AddressingMode::ZeroPage)),
        0xc6 => Some((Opcode::DEC, AddressingMode::ZeroPage)),
        0xc7 => None,
        0xc8 => Some((Opcode::INY, AddressingMode::Implied)),
        0xc9 => Some((Opcode::CMP, AddressingMode::Immediate)),
        0xca => Some((Opcode::DEX, AddressingMode::Implied)),
        0xcb => None,
        0xcc => Some((Opcode::CPY, AddressingMode::Absolute)),
        0xcd => Some((Opcode::CMP, AddressingMode::Absolute)),
        0xce => Some((Opcode::DEC, AddressingMode::Absolute)),
        0xcf => None,
        0xd0 => Some((Opcode::BNE, AddressingMode::Relative)),
        0xd1 => Some((Opcode::CMP, AddressingMode::ZeroPageIndirectYIndexed)),
        0xd2 => None,
        0xd3 => None,
        0xd4 => None,
        0xd5 => Some((Opcode::CMP, AddressingMode::XIndexedZeroPage)),
        0xd6 => Some((Opcode::DEC, AddressingMode::XIndexedZeroPage)),
        0xd7 => None,
        0xd8 => Some((Opcode::CLD, AddressingMode::Implied)),
        0xd9 => Some((Opcode::CMP, AddressingMode::YIndexedAbsolute)),
        0xda => None,
        0xdb => None,
        0xdc => None,
        0xdd => Some((Opcode::CMP, AddressingMode::XIndexedAbsolute)),
        0xde => Some((Opcode::DEC, AddressingMode::XIndexedAbsolute)),
        0xdf => None,
        0xe0 => Some((Opcode::CPX, AddressingMode::Immediate)),
        0xe1 => Some((Opcode::SBC, AddressingMode::XIndexedZeroPageIndirect)),
        0xe2 => None,
        0xe3 => None,
        0xe4 => Some((Opcode::CPX, AddressingMode::ZeroPage)),
        0xe5 => Some((Opcode::SBC, AddressingMode::ZeroPage)),
        0xe6 => Some((Opcode::INC, AddressingMode::ZeroPage)),
        0xe7 => None,
        0xe8 => Some((Opcode::INX, AddressingMode::Implied)),
        0xe9 => Some((Opcode::SBC, AddressingMode::Immediate)),
        0xea => Some((Opcode::NOP, AddressingMode::Implied)),
        0xeb => None,
        0xec => Some((Opcode::CPX, AddressingMode::Absolute)),
        0xed => Some((Opcode::SBC, AddressingMode::Absolute)),
        0xee => Some((Opcode::INC, AddressingMode::Absolute)),
        0xef => None,
        0xf0 => Some((Opcode::BEQ, AddressingMode::Relative)),
        0xf1 => Some((Opcode::SBC, AddressingMode::ZeroPageIndirectYIndexed)),
        0xf2 => None,
        0xf3 => None,
        0xf4 => None,
        0xf5 => Some((Opcode::SBC, AddressingMode::XIndexedZeroPage)),
        0xf6 => Some((Opcode::INC, AddressingMode::XIndexedZeroPage)),
        0xf7 => None,
        0xf8 => Some((Opcode::SED, AddressingMode::Implied)),
        0xf9 => Some((Opcode::SBC, AddressingMode::YIndexedAbsolute)),
        0xfa => None,
        0xfb => None,
        0xfc => None,
        0xfd => Some((Opcode::SBC, AddressingMode::XIndexedAbsolute)),
        0xfe => Some((Opcode::INC, AddressingMode::XIndexedAbsolute)),
        0xff => None,
        _ => None,
    }
}
pub static OPCODES_STRING: [&str; 256] = [
    " BRK 7        $00: bytes: 0 cycles: 0 _____=>_____ __ ",
    " ORA izx 6    $01: bytes: 2 cycles: 6 A____=>____P R_ izx",
    " *KIL         $02: CRASH",
    " *SLO izx 8   $03: bytes: 2 cycles: 8 A____=>____P RW izx",
    " *NOP zp 3    $04: bytes: 2 cycles: 3 _____=>_____ R_ zp",
    " ORA zp 3     $05: bytes: 2 cycles: 3 A____=>A___P R_ zp",
    " ASL zp 5     $06: bytes: 2 cycles: 5 _____=>____P RW zp",
    " *SLO zp 5    $07: bytes: 2 cycles: 5 A____=>A___P RW zp",
    " PHP 3        $08: bytes: 1 cycles: 3 ___SP=>___S_ _W ",
    " ORA imm 2    $09: bytes: 2 cycles: 2 _____=>A___P __ ",
    " ASL 2        $0A: bytes: 1 cycles: 2 A____=>A___P __ ",
    " *ANC imm 2   $0B: bytes: 2 cycles: 2 A____=>____P __ ",
    " *NOP abs 4   $0C: bytes: 3 cycles: 4 _____=>_____ R_ abs",
    " ORA abs 4    $0D: bytes: 3 cycles: 4 A____=>A___P R_ abs",
    " ASL abs 6    $0E: bytes: 3 cycles: 6 _____=>____P RW abs",
    " *SLO abs 6   $0F: bytes: 3 cycles: 6 A____=>A___P RW abs",
    " BPL rel 2*   $10: bytes: 2 cycles: 3 ____P=>_____ __ ",
    " ORA izy 5*   $11: bytes: 2 cycles: 5 A____=>____P R_ izy",
    " *KIL         $12: CRASH",
    " *SLO izy 8   $13: bytes: 2 cycles: 8 A____=>____P RW izy",
    " *NOP zpx 4   $14: bytes: 2 cycles: 4 _____=>_____ R_ zpx",
    " ORA zpx 4    $15: bytes: 2 cycles: 4 A____=>A___P R_ zpx",
    " ASL zpx 6    $16: bytes: 2 cycles: 6 _____=>____P RW zpx",
    " *SLO zpx 6   $17: bytes: 2 cycles: 6 A____=>A___P RW zpx",
    " CLC 2        $18: bytes: 1 cycles: 2 _____=>____P __ ",
    " ORA aby 4*   $19: bytes: 3 cycles: 4 A____=>A___P R_ absy",
    " *NOP 2       $1A: bytes: 1 cycles: 2 _____=>_____ __ ",
    " *SLO aby 7   $1B: bytes: 3 cycles: 7 A____=>A___P RW absy",
    " *NOP abx 4*  $1C: bytes: 3 cycles: 4 _____=>_____ R_ absx",
    " ORA abx 4*   $1D: bytes: 3 cycles: 4 A____=>A___P R_ absx",
    " ASL abx 7    $1E: bytes: 3 cycles: 7 _____=>____P RW absx",
    " *SLO abx 7   $1F: bytes: 3 cycles: 7 A____=>A___P RW absx",
    " JSR abs 6    $20: bytes: X cycles: 6 ___S_=>___S_ _W ",
    " AND izx 6    $21: bytes: 2 cycles: 6 _____=>A___P R_ izx",
    " *KIL         $22: CRASH",
    " *RLA izx 8   $23: bytes: 2 cycles: 8 ____P=>A___P RW izx",
    " BIT zp 3     $24: bytes: 2 cycles: 3 A____=>____P R_ zp",
    " AND zp 3     $25: bytes: 2 cycles: 3 A____=>A___P R_ zp",
    " ROL zp 5     $26: bytes: 2 cycles: 5 ____P=>____P RW zp",
    " *RLA zp 5    $27: bytes: 2 cycles: 5 A___P=>A___P RW zp",
    " PLP 4        $28: bytes: 1 cycles: 4 ___S_=>___SP __ ",
    " AND imm 2    $29: bytes: 2 cycles: 2 A____=>A___P __ ",
    " ROL 2        $2A: bytes: 1 cycles: 2 A___P=>A___P __ ",
    " *ANC imm 2   $2B: bytes: 2 cycles: 2 A____=>____P __ ",
    " BIT abs 4    $2C: bytes: 3 cycles: 4 A____=>____P R_ abs",
    " AND abs 4    $2D: bytes: 3 cycles: 4 A____=>A___P R_ abs",
    " ROL abs 6    $2E: bytes: 3 cycles: 6 ____P=>____P RW abs",
    " *RLA abs 6   $2F: bytes: 3 cycles: 6 A___P=>A___P RW abs",
    " BMI rel 2*   $30: bytes: 2 cycles: 2 _____=>_____ __ ",
    " AND izy 5*   $31: bytes: 2 cycles: 5 _____=>A___P R_ izy",
    " *KIL         $32: CRASH",
    " *RLA izy 8   $33: bytes: 2 cycles: 8 ____P=>A___P RW izy",
    " *NOP zpx 4   $34: bytes: 2 cycles: 4 _____=>_____ R_ zpx",
    " AND zpx 4    $35: bytes: 2 cycles: 4 A____=>A___P R_ zpx",
    " ROL zpx 6    $36: bytes: 2 cycles: 6 ____P=>____P RW zpx",
    " *RLA zpx 6   $37: bytes: 2 cycles: 6 A___P=>A___P RW zpx",
    " SEC 2        $38: bytes: 1 cycles: 2 _____=>____P __ ",
    " AND aby 4*   $39: bytes: 3 cycles: 4 A____=>A___P R_ absy",
    " *NOP 2       $3A: bytes: 1 cycles: 2 _____=>_____ __ ",
    " *RLA aby 7   $3B: bytes: 3 cycles: 7 A___P=>A___P RW absy",
    " *NOP abx 4*  $3C: bytes: 3 cycles: 4 _____=>_____ R_ absx",
    " AND abx 4*   $3D: bytes: 3 cycles: 4 A____=>A___P R_ absx",
    " ROL abx 7    $3E: bytes: 3 cycles: 7 ____P=>____P RW absx",
    " *RLA abx 7   $3F: bytes: 3 cycles: 7 A___P=>A___P RW absx",
    " RTI 6        $40: bytes: X cycles: 6 ___S_=>___SP __ ",
    " EOR izx 6    $41: bytes: 2 cycles: 6 A____=>____P R_ izx",
    " *KIL         $42: CRASH",
    " *SRE izx 8   $43: bytes: 2 cycles: 8 A____=>____P RW izx",
    " *NOP zp 3    $44: bytes: 2 cycles: 3 _____=>_____ R_ zp",
    " EOR zp 3     $45: bytes: 2 cycles: 3 A____=>A___P R_ zp",
    " LSR zp 5     $46: bytes: 2 cycles: 5 _____=>____P RW zp",
    " *SRE zp 5    $47: bytes: 2 cycles: 5 A____=>A___P RW zp",
    " PHA 3        $48: bytes: 1 cycles: 3 A__S_=>___S_ _W ",
    " EOR imm 2    $49: bytes: 2 cycles: 2 A____=>A___P __ ",
    " LSR 2        $4A: bytes: 1 cycles: 2 A____=>A___P __ ",
    " *ALR imm 2   $4B: bytes: 2 cycles: 2 A____=>A___P __ ",
    " JMP abs 3    $4C: bytes: X cycles: 3 _____=>_____ __ ",
    " EOR abs 4    $4D: bytes: 3 cycles: 4 A____=>A___P R_ abs",
    " LSR abs 6    $4E: bytes: 3 cycles: 6 _____=>____P RW abs",
    " *SRE abs 6   $4F: bytes: 3 cycles: 6 A____=>A___P RW abs",
    " BVC rel 2*   $50: bytes: 2 cycles: 3 ____P=>_____ __ ",
    " EOR izy 5*   $51: bytes: 2 cycles: 5 A____=>____P R_ izy",
    " *KIL         $52: CRASH",
    " *SRE izy 8   $53: bytes: 2 cycles: 8 A____=>____P RW izy",
    " *NOP zpx 4   $54: bytes: 2 cycles: 4 _____=>_____ R_ zpx",
    " EOR zpx 4    $55: bytes: 2 cycles: 4 A____=>A___P R_ zpx",
    " LSR zpx 6    $56: bytes: 2 cycles: 6 _____=>____P RW zpx",
    " *SRE zpx 6   $57: bytes: 2 cycles: 6 A____=>A___P RW zpx",
    " CLI 2        $58: bytes: 1 cycles: 2 _____=>____P __ ",
    " EOR aby 4*   $59: bytes: 3 cycles: 4 A____=>A___P R_ absy",
    " *NOP 2       $5A: bytes: 1 cycles: 2 _____=>_____ __ ",
    " *SRE aby 7   $5B: bytes: 3 cycles: 7 A____=>A___P RW absy",
    " *NOP abx 4*  $5C: bytes: 3 cycles: 4 _____=>_____ R_ absx",
    " EOR abx 4*   $5D: bytes: 3 cycles: 4 A____=>A___P R_ absx",
    " LSR abx 7    $5E: bytes: 3 cycles: 7 _____=>____P RW absx",
    " *SRE abx 7   $5F: bytes: 3 cycles: 7 A____=>A___P RW absx",
    " RTS 6        $60: bytes: X cycles: 6 ___S_=>___S_ __ ",
    " ADC izx 6    $61: bytes: 2 cycles: 6 A___P=>A___P R_ izx",
    " *KIL         $62: CRASH",
    " *RRA izx 8   $63: bytes: 2 cycles: 8 A___P=>A___P RW izx",
    " *NOP zp 3    $64: bytes: 2 cycles: 3 _____=>_____ R_ zp",
    " ADC zp 3     $65: bytes: 2 cycles: 3 A___P=>A___P R_ zp",
    " ROR zp 5     $66: bytes: 2 cycles: 5 ____P=>____P RW zp",
    " *RRA zp 5    $67: bytes: 2 cycles: 5 A___P=>A___P RW zp",
    " PLA 4        $68: bytes: 1 cycles: 4 ___S_=>A__SP __ ",
    " ADC imm 2    $69: bytes: 2 cycles: 2 A___P=>A___P __ ",
    " ROR 2        $6A: bytes: 1 cycles: 2 A___P=>A___P __ ",
    " *ARR imm 2   $6B: bytes: 2 cycles: 2 A___P=>A___P __ ",
    " JMP ind 5    $6C: bytes: X cycles: 5 _____=>_____ __ ",
    " ADC abs 4    $6D: bytes: 3 cycles: 4 A___P=>A___P R_ abs",
    " ROR abs 6    $6E: bytes: 3 cycles: 6 ____P=>____P RW abs",
    " *RRA abs 6   $6F: bytes: 3 cycles: 6 A___P=>A___P RW abs",
    " BVS rel 2*   $70: bytes: 2 cycles: 2 _____=>_____ __ ",
    " ADC izy 5*   $71: bytes: 2 cycles: 5 A___P=>A___P R_ izy",
    " *KIL         $72: CRASH",
    " *RRA izy 8   $73: bytes: 2 cycles: 8 A___P=>A___P RW izy",
    " *NOP zpx 4   $74: bytes: 2 cycles: 4 _____=>_____ R_ zpx",
    " ADC zpx 4    $75: bytes: 2 cycles: 4 A___P=>A___P R_ zpx",
    " ROR zpx 6    $76: bytes: 2 cycles: 6 ____P=>____P RW zpx",
    " *RRA zpx 6   $77: bytes: 2 cycles: 6 A___P=>A___P RW zpx",
    " SEI 2        $78: bytes: 1 cycles: 2 _____=>____P __ ",
    " ADC aby 4*   $79: bytes: 3 cycles: 4 A___P=>A___P R_ absy",
    " *NOP 2       $7A: bytes: 1 cycles: 2 _____=>_____ __ ",
    " *RRA aby 7   $7B: bytes: 3 cycles: 7 A___P=>A___P RW absy",
    " *NOP abx 4*  $7C: bytes: 3 cycles: 4 _____=>_____ R_ absx",
    " ADC abx 4*   $7D: bytes: 3 cycles: 4 A___P=>A___P R_ absx",
    " ROR abx 7    $7E: bytes: 3 cycles: 7 ____P=>____P RW absx",
    " *RRA abx 7   $7F: bytes: 3 cycles: 7 A___P=>A___P RW absx",
    " *NOP imm 2   $80: bytes: 2 cycles: 2 _____=>_____ __ ",
    " STA izx 6    $81: bytes: 2 cycles: 6 A____=>_____ RW izx",
    " *NOP imm 2   $82: bytes: 2 cycles: 2 _____=>_____ __ ",
    " *SAX izx 6   $83: bytes: 2 cycles: 6 _____=>_____ RW izx",
    " STY zp 3     $84: bytes: 2 cycles: 3 __Y__=>_____ _W zp",
    " STA zp 3     $85: bytes: 2 cycles: 3 A____=>_____ _W zp",
    " STX zp 3     $86: bytes: 2 cycles: 3 _X___=>_____ _W zp",
    " *SAX zp 3    $87: bytes: 2 cycles: 3 _____=>_____ _W zp",
    " DEY 2        $88: bytes: 1 cycles: 2 __Y__=>__Y_P __ ",
    " *NOP imm 2   $89: bytes: 2 cycles: 2 _____=>_____ __ ",
    " TXA 2        $8A: bytes: 1 cycles: 2 _X___=>A___P __ ",
    " *XAA imm 2   $8B: bytes: 2 cycles: 2 _____=>A___P __ ",
    " STY abs 4    $8C: bytes: 3 cycles: 4 __Y__=>_____ _W abs",
    " STA abs 4    $8D: bytes: 3 cycles: 4 A____=>_____ _W abs",
    " STX abs 4    $8E: bytes: 3 cycles: 4 _X___=>_____ _W abs",
    " *SAX abs 4   $8F: bytes: 3 cycles: 4 _____=>_____ _W abs",
    " BCC rel 2*   $90: bytes: 2 cycles: 3 ____P=>_____ __ ",
    " STA izy 6    $91: bytes: 2 cycles: 6 A____=>_____ RW izy",
    " *KIL         $92: CRASH",
    " *AHX izy 6   $93: bytes: 2 cycles: 6 _____=>_____ RW izy",
    " STY zpx 4    $94: bytes: 2 cycles: 4 __Y__=>_____ RW zpx",
    " STA zpx 4    $95: bytes: 2 cycles: 4 A____=>_____ RW zpx",
    " STX zpy 4    $96: bytes: 2 cycles: 4 _X___=>_____ RW zpy",
    " *SAX zpy 4   $97: bytes: 2 cycles: 4 _____=>_____ RW zpy",
    " TYA 2        $98: bytes: 1 cycles: 2 __Y__=>A___P __ ",
    " STA aby 5    $99: bytes: 3 cycles: 5 A____=>_____ RW absy",
    " TXS 2        $9A: bytes: X cycles: 2 _X___=>___S_ __ ",
    " *TAS aby 5   $9B: bytes: X cycles: 5 __Y__=>___S_ _W ",
    " *SHY abx 5   $9C: bytes: 3 cycles: 5 __Y__=>_____ RW absx",
    " STA abx 5    $9D: bytes: 3 cycles: 5 A____=>_____ RW absx",
    " *SHX aby 5   $9E: bytes: 3 cycles: 5 _X___=>_____ RW absy",
    " *AHX aby 5   $9F: bytes: 3 cycles: 5 _____=>_____ RW absy",
    " LDY imm 2    $A0: bytes: 2 cycles: 2 _____=>__Y_P __ ",
    " LDA izx 6    $A1: bytes: 2 cycles: 6 _____=>A___P R_ izx",
    " LDX imm 2    $A2: bytes: 2 cycles: 2 _____=>_X__P __ ",
    " *LAX izx 6   $A3: bytes: 2 cycles: 6 _____=>AX__P R_ izx",
    " LDY zp 3     $A4: bytes: 2 cycles: 3 _____=>__Y_P R_ zp",
    " LDA zp 3     $A5: bytes: 2 cycles: 3 _____=>A___P R_ zp",
    " LDX zp 3     $A6: bytes: 2 cycles: 3 _____=>_X__P R_ zp",
    " *LAX zp 3    $A7: bytes: 2 cycles: 3 _____=>AX__P R_ zp",
    " TAY 2        $A8: bytes: 1 cycles: 2 A____=>__Y_P __ ",
    " LDA imm 2    $A9: bytes: 2 cycles: 2 _____=>A___P __ ",
    " TAX 2        $AA: bytes: 1 cycles: 2 A____=>_X__P __ ",
    " *LAX imm 2   $AB: bytes: 2 cycles: 2 A____=>AX__P __ ",
    " LDY abs 4    $AC: bytes: 3 cycles: 4 _____=>__Y_P R_ abs",
    " LDA abs 4    $AD: bytes: 3 cycles: 4 _____=>A___P R_ abs",
    " LDX abs 4    $AE: bytes: 3 cycles: 4 _____=>_X__P R_ abs",
    " *LAX abs 4   $AF: bytes: 3 cycles: 4 _____=>AX__P R_ abs",
    " BCS rel 2*   $B0: bytes: 2 cycles: 2 _____=>_____ __ ",
    " LDA izy 5*   $B1: bytes: 2 cycles: 5 _____=>A___P R_ izy",
    " *KIL         $B2: CRASH",
    " *LAX izy 5*  $B3: bytes: 2 cycles: 5 _____=>AX__P R_ izy",
    " LDY zpx 4    $B4: bytes: 2 cycles: 4 _____=>__Y_P R_ zpx",
    " LDA zpx 4    $B5: bytes: 2 cycles: 4 _____=>A___P R_ zpx",
    " LDX zpy 4    $B6: bytes: 2 cycles: 4 _____=>_X__P R_ zpy",
    " *LAX zpy 4   $B7: bytes: 2 cycles: 4 _____=>AX__P R_ zpy",
    " CLV 2        $B8: bytes: 1 cycles: 2 _____=>____P __ ",
    " LDA aby 4*   $B9: bytes: 3 cycles: 4 _____=>A___P R_ absy",
    " TSX 2        $BA: bytes: 1 cycles: 2 ___S_=>_X__P __ ",
    " *LAS aby 4*  $BB: bytes: 3 cycles: 4 ___S_=>AX_SP R_ absy",
    " LDY abx 4*   $BC: bytes: 3 cycles: 4 _____=>__Y_P R_ absx",
    " LDA abx 4*   $BD: bytes: 3 cycles: 4 _____=>A___P R_ absx",
    " LDX aby 4*   $BE: bytes: 3 cycles: 4 _____=>_X__P R_ absy",
    " *LAX aby 4*  $BF: bytes: 3 cycles: 4 _____=>AX__P R_ absy",
    " CPY imm 2    $C0: bytes: 2 cycles: 2 __Y__=>____P __ ",
    " CMP izx 6    $C1: bytes: 2 cycles: 6 A____=>____P R_ izx",
    " *NOP imm 2   $C2: bytes: 2 cycles: 2 _____=>_____ __ ",
    " *DCP izx 8   $C3: bytes: 2 cycles: 8 A____=>____P RW izx",
    " CPY zp 3     $C4: bytes: 2 cycles: 3 __Y__=>____P R_ zp",
    " CMP zp 3     $C5: bytes: 2 cycles: 3 A____=>____P R_ zp",
    " DEC zp 5     $C6: bytes: 2 cycles: 5 _____=>____P RW zp",
    " *DCP zp 5    $C7: bytes: 2 cycles: 5 A____=>____P RW zp",
    " INY 2        $C8: bytes: 1 cycles: 2 __Y__=>__Y_P __ ",
    " CMP imm 2    $C9: bytes: 2 cycles: 2 A____=>____P __ ",
    " DEX 2        $CA: bytes: 1 cycles: 2 _X___=>_X__P __ ",
    " *AXS imm 2   $CB: bytes: 2 cycles: 2 _____=>_X__P __ ",
    " CPY abs 4    $CC: bytes: 3 cycles: 4 __Y__=>____P R_ abs",
    " CMP abs 4    $CD: bytes: 3 cycles: 4 A____=>____P R_ abs",
    " DEC abs 6    $CE: bytes: 3 cycles: 6 _____=>____P RW abs",
    " *DCP abs 6   $CF: bytes: 3 cycles: 6 A____=>____P RW abs",
    " BNE rel 2*   $D0: bytes: 2 cycles: 3 ____P=>_____ __ ",
    " CMP izy 5*   $D1: bytes: 2 cycles: 5 A____=>____P R_ izy",
    " *KIL         $D2: CRASH",
    " *DCP izy 8   $D3: bytes: 2 cycles: 8 A____=>____P RW izy",
    " *NOP zpx 4   $D4: bytes: 2 cycles: 4 _____=>_____ R_ zpx",
    " CMP zpx 4    $D5: bytes: 2 cycles: 4 A____=>____P R_ zpx",
    " DEC zpx 6    $D6: bytes: 2 cycles: 6 _____=>____P RW zpx",
    " *DCP zpx 6   $D7: bytes: 2 cycles: 6 A____=>____P RW zpx",
    " CLD 2        $D8: bytes: 1 cycles: 2 _____=>____P __ ",
    " CMP aby 4*   $D9: bytes: 3 cycles: 4 A____=>____P R_ absy",
    " *NOP 2       $DA: bytes: 1 cycles: 2 _____=>_____ __ ",
    " *DCP aby 7   $DB: bytes: 3 cycles: 7 A____=>____P RW absy",
    " *NOP abx 4*  $DC: bytes: 3 cycles: 4 _____=>_____ R_ absx",
    " CMP abx 4*   $DD: bytes: 3 cycles: 4 A____=>____P R_ absx",
    " DEC abx 7    $DE: bytes: 3 cycles: 7 _____=>____P RW absx",
    " *DCP abx 7   $DF: bytes: 3 cycles: 7 A____=>____P RW absx",
    " CPX imm 2    $E0: bytes: 2 cycles: 2 _X___=>____P __ ",
    " SBC izx 6    $E1: bytes: 2 cycles: 6 A___P=>A___P R_ izx",
    " *NOP imm 2   $E2: bytes: 2 cycles: 2 _____=>_____ __ ",
    " *ISC izx 8   $E3: bytes: 2 cycles: 8 A___P=>A___P RW izx",
    " CPX zp 3     $E4: bytes: 2 cycles: 3 _X___=>____P R_ zp",
    " SBC zp 3     $E5: bytes: 2 cycles: 3 A___P=>A___P R_ zp",
    " INC zp 5     $E6: bytes: 2 cycles: 5 _____=>____P RW zp",
    " *ISC zp 5    $E7: bytes: 2 cycles: 5 A___P=>A___P RW zp",
    " INX 2        $E8: bytes: 1 cycles: 2 _X___=>_X__P __ ",
    " SBC imm 2    $E9: bytes: 2 cycles: 2 A___P=>A___P __ ",
    " NOP 2        $EA: bytes: 1 cycles: 2 _____=>_____ __ ",
    " *SBC imm 2   $EB: bytes: 2 cycles: 2 A___P=>A___P __ ",
    " CPX abs 4    $EC: bytes: 3 cycles: 4 _X___=>____P R_ abs",
    " SBC abs 4    $ED: bytes: 3 cycles: 4 A___P=>A___P R_ abs",
    " INC abs 6    $EE: bytes: 3 cycles: 6 _____=>____P RW abs",
    " *ISC abs 6   $EF: bytes: 3 cycles: 6 A___P=>A___P RW abs",
    " BEQ rel 2*   $F0: bytes: 2 cycles: 2 _____=>_____ __ ",
    " SBC izy 5*   $F1: bytes: 2 cycles: 5 A___P=>A___P R_ izy",
    " *KIL         $F2: CRASH",
    " *ISC izy 8   $F3: bytes: 2 cycles: 8 A___P=>A___P RW izy",
    " *NOP zpx 4   $F4: bytes: 2 cycles: 4 _____=>_____ R_ zpx",
    " SBC zpx 4    $F5: bytes: 2 cycles: 4 A___P=>A___P R_ zpx",
    " INC zpx 6    $F6: bytes: 2 cycles: 6 _____=>____P RW zpx",
    " *ISC zpx 6   $F7: bytes: 2 cycles: 6 A___P=>A___P RW zpx",
    " SED 2        $F8: bytes: 1 cycles: 2 _____=>____P __ ",
    " SBC aby 4*   $F9: bytes: 3 cycles: 4 A___P=>A___P R_ absy",
    " *NOP 2       $FA: bytes: 1 cycles: 2 _____=>_____ __ ",
    " *ISC aby 7   $FB: bytes: 3 cycles: 7 A___P=>A___P RW absy",
    " *NOP abx 4*  $FC: bytes: 3 cycles: 4 _____=>_____ R_ absx",
    " SBC abx 4*   $FD: bytes: 3 cycles: 4 A___P=>A___P R_ absx",
    " INC abx 7    $FE: bytes: 3 cycles: 7 _____=>____P RW absx",
    " *ISC abx     $FF: bytes: 3 cycles: 7 A___P=>A___P RW absx",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Opcode {
    ADC,
    AHX,
    ALR,
    ANC,
    AND,
    ARR,
    ASL,
    AXS,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DCP,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    ISC,
    JMP,
    JSR,
    KIL,
    LAS,
    LAX,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    RLA,
    ROL,
    ROR,
    RRA,
    RTI,
    RTS,
    SAX,
    SBC,
    SEC,
    SED,
    SEI,
    SHX,
    SHY,
    SLO,
    SRE,
    STA,
    STX,
    STY,
    TAS,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
    XAA,
}
