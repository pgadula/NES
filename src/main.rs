use m6502::{Bus, Mos6502, RESET_VECTOR, VECTOR_BASE};

mod m6502;
mod opcodes;

fn main() {
    println!("Hello, world!");
    let bus = Bus::new();
    let mut cpu = Mos6502::new(bus);
    // adc(&mut cpu);
    // println!("{}", cpu);
    // println!("{}", cpu.bus);
    // cpu.pc.to_le_bytes()
    // cpu.reset();

    // println!("{}", cpu.bus);
    let program = [
        0xA9, 0x42, 0xAA, 0xA8, 0x8D, 0x00, 0x02, 0x8E, 0x01, 0x02, 0x8C, 0x02, 0x02,
    ];
    let program1: [u8; 7] = [
        0xA9, 0x05, // LDA #$05
        0x38, // SEC
        0x69, 0x03, // ADC #$03
        0xC9, 0x09, // CMP #$09
    ];
    // Program 2: load A = 0x80, move to X, CPX 0x80, BIT $00
    let program2: [u8; 7] = [
        0xA9, 0x80, // LDA #$80
        0xAA, // TAX
        0xE0, 0x80, // CPX #$80
        0x24, 0x00, // BIT $00
    ];
    cpu.bus.write_bytes(0x30, &program1);
    cpu.pc = 0x30;
    println!("{}", cpu);
    cpu.fetch();
    println!("{}", cpu);
    cpu.fetch();
    println!("{}", cpu);
    cpu.fetch();
    println!("{}", cpu);
    cpu.fetch();
    println!("{}", cpu);
    // cpu.fetch();
    // println!("{}", cpu);
    // cpu.fetch();
    // println!("{}", cpu);

    cpu.bus.dump();
    // cpu.fetch();
    // cpu.fetch();
    // cpu.fetch();
    // cpu.fetch();
    // cpu.fetch();
    // cpu.fetch();
    // cpu.fetch();
}
