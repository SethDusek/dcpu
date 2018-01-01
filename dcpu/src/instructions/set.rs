use CPU;
use instruction::*;

#[derive(InstructionInfo)]
#[opcode = 0x01]
#[cycles = 2]
pub struct Set;

impl Instruction for Set {
    fn run(cpu: &mut CPU, instr: Instr) -> Option<u16> {
        println!("??");
        let a = {
            let arg_a = instr.arg_a(cpu).into_val();
            if let Val(val) = arg_a {
                val
            }
            else { return None; }
        };

        let arg_b = instr.arg_b(cpu);
        if let Ptr(ptr, pc) = arg_b { //ignore value otherwise, since writing to a literal doesn't/isn't supposed to do anything
            if pc {
                return Some(a)
            }
            else { *ptr=a }
        }
        None
    }
}
