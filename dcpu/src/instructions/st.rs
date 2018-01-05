/*
 * This file contains both STI and STD instructions
 */

use CPU;
use instruction::*;
use registers::Register;
use instructions::Set;



#[derive(InstructionInfo)]
#[opcode = 0x1e]
#[cycles = 2]
pub struct Sti;


impl Instruction for Sti {
    fn run(cpu: &mut CPU, instr: Instr) {
        Set::run(cpu, instr);
        cpu.registers[Register::I]=cpu.registers[Register::I].wrapping_add(1);
        cpu.registers[Register::J]=cpu.registers[Register::I].wrapping_add(1);
    }
}


#[derive(InstructionInfo)]
#[opcode = 0x1f]
#[cycles = 2]
pub struct Std;

impl Instruction for Std {
    fn run(cpu: &mut CPU, instr: Instr) {
        Set::run(cpu, instr);
        cpu.registers[Register::I]=cpu.registers[Register::I].wrapping_sub(1);
        cpu.registers[Register::J]=cpu.registers[Register::J].wrapping_sub(1);
    }
}
