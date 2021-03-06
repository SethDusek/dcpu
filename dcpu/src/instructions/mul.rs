use CPU;
use instruction::*;
#[derive(InstructionInfo)]
#[opcode = 0x04]
#[cycles = 2]
pub struct Mul;


impl Instruction for Mul {
    fn run(cpu: &mut CPU, instr: Instr) {
        let a = if let Val(val) = instr.arg_a(cpu).into_val() {
            val
        }
        else {return};
        if let Ptr(ptr) = instr.arg_b(cpu) {
            *ptr=*ptr*a;
        }
    }
}


#[derive(InstructionInfo)]
#[name = "MLI"]
#[opcode=0x05]
#[cycles = 2]
pub struct MulI;


impl Instruction for MulI {
    fn run(cpu: &mut CPU, instr: Instr) {
        let a = if let Val(val) = instr.arg_a(cpu).into_val() {
            val as i16
        }
        else {return};
        if let Ptr(ptr) = instr.arg_b(cpu) {
            *ptr=((*ptr as i16)*a) as u16;
        }
    }
}


