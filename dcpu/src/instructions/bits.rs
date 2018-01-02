//This file contains all the bitwise operations, and/or/xor/shift functions

use CPU;
use instruction::*;

/* AND b, a
 * sets b to b&a
 */
#[derive(InstructionInfo)]
#[opcode = 0x0a]
#[cycles = 1]
pub struct And; 


impl Instruction for And {
    fn run(cpu: &mut CPU, instr: Instr) {
        let a = if let Val(val) = instr.arg_a(cpu).into_val() {
            val
        }
        else {return};
        if let Ptr(ptr) = instr.arg_b(cpu) {
            *ptr=*ptr&a;
        }
    }
}


/* BOR b,a
 * sets b to b|a
 */
#[derive(InstructionInfo)]
#[opcode = 0x0b]
#[cycles = 1]
pub struct Bor;

impl Instruction for Bor {
    fn run(cpu: &mut CPU, instr: Instr) {
        let a = if let Val(val) = instr.arg_a(cpu).into_val() {
            val
        }
        else {return};
        
        if let Ptr(ptr) = instr.arg_b(cpu) {
            *ptr=*ptr|a
        }
    }
}


#[derive(InstructionInfo)]
#[opcode = 0x0c]
#[cycles = 1]
pub struct Xor;

impl Instruction for Xor {
    fn run(cpu: &mut CPU, instr: Instr) {
        let a = if let Val(val) = instr.arg_a(cpu).into_val() {
            val
        }
        else {return};

        if let Ptr(ptr) = instr.arg_b(cpu) {
            *ptr=*ptr^a
        }
    }
}

#[derive(InstructionInfo)]
#[opcode = 0x0d]
#[cycles = 1]
pub struct Shr;

impl Instruction for Shr {
    fn run(cpu: &mut CPU, instr: Instr) {
        let a = if let Val(val) = instr.arg_a(cpu).into_val() {
            val
        }
        else {return};

        if let Ptr(ptr) = instr.arg_b(cpu) {
            *ptr=*ptr>>a
        }
    }
}


#[derive(InstructionInfo)]
#[opcode = 0x0e]
#[cycles = 1]
pub struct Asr;

impl Instruction for Asr {
    fn run(cpu: &mut CPU, instr: Instr) {
        let a = if let Val(val) = instr.arg_a(cpu).into_val() {
            val
        } 
        else {return};

        if let Ptr(ptr) = instr.arg_b(cpu) {
            *ptr=ptr.rotate_right(a as u32);
        }
    }
}


#[derive(InstructionInfo)]
#[opcode = 0x0f]
pub struct Shl;

impl Instruction for Shl {
    fn run(cpu: &mut CPU, instr: Instr) {
        let a = if let Val(val) = instr.arg_a(cpu).into_val() {
            val
        }
        else {return};

        if let Ptr(ptr) = instr.arg_b(cpu) {
            *ptr=*ptr<<a
        }
    }
}
