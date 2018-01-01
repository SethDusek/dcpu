use CPU;
use instruction::*;
#[derive(InstructionInfo)]
#[opcode = 0x03]
#[cycles = 2]
pub struct Sub;


impl Instruction for Sub {
    fn run(cpu: &mut CPU, instr: Instr) {
        let a = {
            if let Val(val) = instr.arg_a(cpu).into_val() {
                val
            } else { return }
        };
        if let Ptr(ptr) = instr.arg_b(cpu ) {
            *ptr=ptr.wrapping_sub(a)
        }
    }
}
