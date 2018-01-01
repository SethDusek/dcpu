#![feature(exclusive_range_pattern)]
#![feature(attr_literals)]
#[macro_use]
extern crate dcpu_macros;
pub mod registers;
#[macro_use]
mod macros;
pub mod instruction;
pub mod instructions;
pub use instruction::Instr;
use registers::*;



pub struct CPU {
    pub sp: u16,
    pub registers: Registers,
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
        self.pc+=1;
        Instr::new(self.memory[self.pc as usize])
    }
    /// Fetches an instruction, does not increment/change PC
    pub fn fetch(&mut self) -> Instr {
        Instr::new(self.memory[self.pc as usize])
    }

    pub fn cycle(&mut self, cycles: u64) {// currently a noop
    }
    //Fetch an instruction, decode/execute it and then move on to the next
    pub fn tick(&mut self) {
        let instr = self.fetch();
        let pc = instr.run(self);
        if let Some(pc) = pc {
            self.pc = pc;
        }
        else {
            self.pc+=1;
        }
    }
}

