use CPU;
use instruction::*;

#[derive(InstructionInfo)]
#[opcode=0x06]
#[cycles=3]
pub struct Div;

impl Instruction for Div {
    fn run(cpu: &mut CPU, instr: Instr) {
        let a = if let Val(val) = instr.arg_a(cpu) {
            val
        }
        else {return};

        if let Ptr(ptr) = instr.arg_b(cpu) {
            if a == 0 {
                *ptr=0;
            }
            else {
                *ptr=*ptr/a
            }
        }
    }
}

#[derive(InstructionInfo)]
#[opcode=0x07]
#[cycles=3]
pub struct Dvi;

impl Instruction for Dvi {
    fn run(cpu: &mut CPU, instr: Instr) {
        let a = if let Val(val) = instr.arg_a(cpu) {
            val as i16
        }
        else {return;};

        if let Ptr(ptr) = instr.arg_b(cpu) {
            if a == 0 {
                *ptr = 0;
            }
            else {
                *ptr=((*ptr as i16)/a) as u16
            }
        }
    }
}
