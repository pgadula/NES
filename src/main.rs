use m6502::{Bus, Mos6502, RESET_VECTOR, VECTOR_BASE};
use opcodes::adc;

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

    // println!("{}", cpu.bus);
    let program = [
        // (F)irst | (S)econd
        // .algo
        0xa5, 0x00, // Load from F to A
        // .algo_
        0x38, // Set carry flag
        0xe5, 0x01, // Substract S from number in A (from F)
        0xf0, 0x07, // Jump to .end if diff is zero
        0x30, 0x08, // Jump to .swap if diff is negative
        0x85, 0x00, // Load A to F
        0x4c, 0x12, 0x00, // Jump to .algo_
        // .end
        0xa5, 0x00, // Load from S to A
        0xff, // .swap
        0xa6, 0x00, // load F to X
        0xa4, 0x01, // load S to Y
        0x86, 0x01, // Store X to F
        0x84, 0x00, // Store Y to S
        0x4c, 0x10, 0x00, // Jump to .algo
    ];
    cpu.bus.write_bytes(0x30, &program);
    cpu.pc = 0x30;
    cpu.bus.dump();
    cpu.fetch();
    cpu.fetch();
    cpu.fetch();
    cpu.fetch();
    cpu.fetch();
    cpu.fetch();
    cpu.fetch();
}


