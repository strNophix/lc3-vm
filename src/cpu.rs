use core::panic;
use std::{
    char,
    io::{self, Read, Write},
    ops::{Index, IndexMut},
};

use crate::memory::Memory;

pub enum ConditionFlag {
    POS = 0b001,
    ZRO = 0b010,
    NEG = 0b100,
}

impl Into<u16> for ConditionFlag {
    fn into(self) -> u16 {
        return self as u16;
    }
}

impl From<u16> for ConditionFlag {
    fn from(value: u16) -> Self {
        match value {
            0b001 => ConditionFlag::POS,
            0b010 => ConditionFlag::ZRO,
            0b100 => ConditionFlag::NEG,
            _ => panic!("Invalid u16 value: {}", value),
        }
    }
}

pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC,
    COND,
    COUNT,
}

pub enum Trapcode {
    GETC = 0x20,
    OUT = 0x21,
    PUTS = 0x22,
    IN = 0x23,
    PUTSP = 0x24,
    HALT = 0x25,
}

impl From<u16> for Trapcode {
    fn from(value: u16) -> Self {
        match value {
            0x20 => Trapcode::GETC,
            0x21 => Trapcode::OUT,
            0x22 => Trapcode::PUTS,
            0x23 => Trapcode::IN,
            0x24 => Trapcode::PUTSP,
            0x25 => Trapcode::HALT,
            _ => panic!("Invalid u16 value: {}", value),
        }
    }
}

pub enum Opcode {
    BR,
    ADD,
    LD,
    ST,
    JSR,
    AND,
    LDR,
    STR,
    RTI,
    NOT,
    LDI,
    STI,
    JMP,
    RES,
    LEA,
    TRAP,
}

impl Into<u16> for Opcode {
    fn into(self) -> u16 {
        return self as u16;
    }
}

impl From<u16> for Opcode {
    fn from(value: u16) -> Self {
        match value >> 12 {
            0b0000 => Opcode::BR,
            0b0001 => Opcode::ADD,
            0b0010 => Opcode::LD,
            0b0011 => Opcode::ST,
            0b0100 => Opcode::JSR,
            0b0101 => Opcode::AND,
            0b0110 => Opcode::LDR,
            0b0111 => Opcode::STR,
            0b1000 => Opcode::RTI,
            0b1001 => Opcode::NOT,
            0b1010 => Opcode::LDI,
            0b1011 => Opcode::STI,
            0b1100 => Opcode::JMP,
            0b1101 => Opcode::RES,
            0b1110 => Opcode::LEA,
            0b1111 => Opcode::TRAP,
            _ => panic!("Invalid u16 value: {}", value),
        }
    }
}

pub struct Cpu {
    registers: [u16; Register::COUNT as usize],
}

impl Cpu {
    pub fn new() -> Self {
        let mut cpu = Self {
            registers: [0; Register::COUNT as usize],
        };

        cpu[Register::COND] = ConditionFlag::ZRO.into();
        cpu[Register::PC] = 0x3000;

        return cpu;
    }

    fn fetch(&mut self, memory: &Memory) -> u16 {
        let value = memory[self[Register::PC]];
        self[Register::PC] += 1;
        return value;
    }

    fn update_flags(&mut self, r: u16) {
        if self[r] == 0 {
            self[Register::COND] = ConditionFlag::ZRO.into();
        } else if self[r] >> 15 == 1 {
            self[Register::COND] = ConditionFlag::NEG.into();
        } else {
            self[Register::COND] = ConditionFlag::POS.into();
        }
    }

    pub fn execute(&mut self, memory: &mut Memory) {
        loop {
            let instr = self.fetch(memory);
            match Opcode::from(instr) {
                Opcode::BR => {
                    let cond: u16 = (instr >> 9) & 0b111;
                    let pc_offset = instr & 0b11111111;
                    if cond & self[Register::COND] == 1 {
                        self[Register::PC] += pc_offset;
                    }
                }
                Opcode::ADD => {
                    let dr = (instr >> 9) & 0b111;
                    let sr1 = (instr >> 6) & 0b111;
                    if (instr >> 5) & 0b1 == 0 {
                        let sr2 = instr & 0b111;
                        self[dr] = self[sr1] + self[sr2];
                    } else {
                        let imm = instr & 0b11111;
                        self[dr] = self[sr1] + imm;
                    }

                    self.update_flags(dr);
                }
                Opcode::LD => {
                    let dr = (&instr >> 9) & 0b111;
                    let pc_offset = instr & 0b11111111;
                    self[Register::R0] = memory[self[Register::PC] + pc_offset];
                    self.update_flags(dr);
                }
                Opcode::ST => {
                    let dr = (instr >> 9) & 0b111;
                    let pc_offset = instr & 0b11111111;
                    memory[self[Register::PC] + pc_offset] = self[dr];
                }
                Opcode::JSR => {
                    self[Register::R7] = self[Register::PC];
                    if (instr >> 11) & 0b1 == 1 {
                        let pc_offset = instr & 0b1111111111;
                        self[Register::PC] += pc_offset;
                    } else {
                        let r1 = (instr >> 6) & 0b111;
                        self[Register::PC] = self[r1];
                    }
                }
                Opcode::AND => {
                    let dr = (instr >> 9) & 0b111;
                    let sr1 = (instr >> 6) & 0b111;
                    if (instr >> 5) & 0b1 == 0 {
                        let sr2 = instr & 0b111;
                        self[dr] = self[sr1] & self[sr2]
                    } else {
                        let imm = instr & 0b11111;
                        self[dr] = self[sr1] & imm;
                    }

                    self.update_flags(dr);
                }
                Opcode::LDR => {
                    let dr = (instr >> 9) & 0b111;
                    let base_r = (instr >> 6) & 0b111;
                    let offset = instr & 0b111111;
                    self[dr] = memory[self[base_r] + offset];
                    self.update_flags(dr);
                }
                Opcode::STR => {
                    let sr = (instr >> 9) & 0b111;
                    let base_r = (instr >> 6) & 0b111;
                    let offset = (instr >> 6) & 0b11111;
                    memory[self[base_r] + offset] = self[sr];
                }
                Opcode::RTI => panic!("Bad opcode: {:#b}", instr),
                Opcode::NOT => {
                    let dr = (instr >> 9) & 0b111;
                    let sr = (instr >> 6) & 0b111;
                    self[dr] = !self[sr];
                    self.update_flags(dr);
                }
                Opcode::LDI => {
                    let dr = (instr >> 9) & 0b111;
                    let pc_offset = instr & 0b11111111;
                    let loc = self[Register::PC] + pc_offset;
                    self[dr] = memory[memory[loc]];
                    self.update_flags(dr);
                }
                Opcode::STI => {
                    let dr = (instr >> 9) & 0b111;
                    let pc_offset = instr & 0b11111111;
                    let addr = memory[self[Register::PC] + pc_offset];
                    memory[addr] = self[dr];
                }
                Opcode::JMP => {
                    let r1 = (instr >> 6) & 0b111;
                    self[Register::PC] = self[r1];
                }
                Opcode::RES => panic!("Bad opcode: {:#b}", instr),
                Opcode::LEA => {
                    let dr = (instr >> 9) & 0b111;
                    let pc_offset = instr & 0b11111111;
                    self[dr] = self[Register::PC] + pc_offset;
                    self.update_flags(dr);
                }
                Opcode::TRAP => {
                    self[Register::R7] = self[Register::PC];
                    let trap = Trapcode::from(instr & 0b11111111);
                    match trap {
                        Trapcode::GETC => {
                            let mut input = [0u8; 1];
                            io::stdin().read(&mut input).unwrap();
                            self[Register::R0] = input[0] as u16;
                        }
                        Trapcode::OUT => {
                            let ch = char::from_u32(self[Register::R0] as u32).unwrap();
                            print!("{}", ch);
                        }
                        Trapcode::PUTS => {
                            let mut loc = self[Register::R0];
                            let mut chars = String::new();
                            loop {
                                let chr = memory[loc];
                                if chr == 0 {
                                    break;
                                }

                                let ch = char::from_u32(chr.into()).unwrap();
                                chars.push(ch);
                                loc += 1;
                            }

                            print!("{}", chars);
                        }
                        Trapcode::IN => {
                            print!("Enter a character: ");
                            io::stdout().flush().unwrap();

                            let mut input = [0u8; 1];
                            io::stdin().read(&mut input).unwrap();
                            self[Register::R0] = input[0] as u16;
                            self.update_flags(0); // Register::R0
                        }
                        Trapcode::PUTSP => {
                            let mut loc = self[Register::R0];
                            let mut chars = String::new();
                            loop {
                                let chr = memory[loc];
                                if chr == 0 {
                                    break;
                                }

                                chars.push(char::from_u32((chr & 0xFF).into()).unwrap());

                                let chr2 = chr >> 8;
                                if chr2 != 0 {
                                    let ch2 = char::from_u32(chr2.into()).unwrap();
                                    chars.push(ch2);
                                }

                                loc += 1;
                            }

                            print!("{}", chars);
                        }
                        Trapcode::HALT => {
                            println!("HALT");
                            return;
                        }
                    }
                }
            }
        }
    }
}

impl Index<Register> for Cpu {
    type Output = u16;

    fn index(&self, index: Register) -> &Self::Output {
        return &self.registers[index as usize];
    }
}

impl IndexMut<Register> for Cpu {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        &mut self.registers[index as usize]
    }
}

impl Index<u16> for Cpu {
    type Output = u16;

    fn index(&self, index: u16) -> &Self::Output {
        &self.registers[index as usize]
    }
}

impl IndexMut<u16> for Cpu {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.registers[index as usize]
    }
}
