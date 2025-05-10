use m6502::{Bus, Mos6502};
use opcodes::adc;

mod m6502;
mod opcodes;

fn main() {
    println!("Hello, world!");
    let bus = Bus::new();
    let mut cpu = Mos6502::new(bus);
    adc(&mut cpu);
    println!("{}", cpu);
    println!("{}", cpu.bus);
}
