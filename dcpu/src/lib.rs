#![feature(exclusive_range_pattern)]
#[macro_use]
extern crate dcpu_macros;
pub mod registers;
#[macro_use]
mod macros;
pub mod instructions;
pub use instructions::Instr;
use registers::*;
pub struct CPU {
    pub sp: u16,
    registers: Registers,
    pub pc: u16,
    pub excess: u16,
    pub memory: [u16; 0x10000],
    pub interrupt_address: u16
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            sp: 0xffff,
            registers: Registers::new(),
            pc: 0,
            excess: 0,
            memory: [0; 0x10000],
            interrupt_address: 0
        }
    }
    pub fn registers(&mut self) -> &mut Registers {
        &mut self.registers
    }
    pub fn push(&mut self, val: u16) {
        self.sp-=1;
        self.memory[self.sp as usize] = val;
    }
    pub fn pop(&mut self) -> u16 {
        let val = self.peek();
        self.sp+=1;
        val
    }

    pub fn peek(&self) -> u16 {
        self.memory[self.sp as usize]
    }
    ///Returns the next instruction
    pub fn next(&mut self) -> Instr {
        self.sp+=1;
        Instr::new(self.memory[self.sp as usize])
    }

}
