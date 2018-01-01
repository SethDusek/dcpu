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
    pub exit: bool, //If true, execution will be halted
    pub sp: u16,
    pub registers: Registers,
    pub pc: u16,
    pub excess: u16,
    pub memory: [u16; 0x10000],
    pub interrupt_address: u16,

}

impl CPU {
    pub fn new() -> Self {
        CPU {
            exit: false,
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
        let instr = Instr::new(self.memory[self.pc as usize]);
        self.pc+=1;
        instr
    }
    /// Fetches an instruction, does not increment/change PC
    pub fn fetch(&mut self) -> Instr {
        Instr::new(self.memory[self.pc as usize])
    }

    pub fn cycle(&mut self, cycles: u64) {// currently a noop
    }
    //Fetch an instruction, decode/execute it and then move on to the next
    pub fn tick(&mut self) {
        let prev_pc = self.pc;        
        let instr = self.next();
        let pc = instr.run(self);
        if self.pc==prev_pc { println!("exiting"); self.exit=true }  //The instruction has not touched PC, so we can increment it. If it detects that PC has changed, does nothing
    }
}

