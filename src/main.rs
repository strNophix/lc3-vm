use lc3_vm::{cpu::Cpu, memory::Memory};

fn main() {
    let instr = [0b1111000000100011, 0b1111000000100101];

    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    memory.write_at(&instr, 0x3000);
    cpu.execute(&mut memory)
}
