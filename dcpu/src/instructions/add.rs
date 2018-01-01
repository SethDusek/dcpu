use CPU;
use instruction::*;

#[derive(InstructionInfo)]
#[opcode = 0x02]
#[cycles = 2]
pub struct Add;

impl Instruction for Add {
    fn run(cpu: &mut CPU, instr: Instr) {
        let a: u16;

        match instr.arg_a(cpu).into_val() {
            Val(val) => a = val,
            _ => a = 0
        }
        let arg_b = instr.arg_b(cpu);
        if let Ptr(ptr) = arg_b { //Don't write if it's a literal
            *ptr=ptr.wrapping_add(a);
        }
    }
}
            
