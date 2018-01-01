use CPU;
use instruction::*;

#[derive(InstructionInfo)]
#[opcode = 0x02]
pub struct Add;

impl Instruction for Add {
    fn run(cpu: &mut CPU, instr: Instr) -> Option<u16> {
        let a: u16;

        match instr.arg_a(cpu) {
            Ptr(ptr, _) => a = *ptr,
            Val(val) => a = val,
            _ => a = 0
        }
        let mut arg_b = instr.arg_b(cpu);
        if let Ptr(ptr, pc) = arg_b { //Don't write if it's a literal
            if !pc {
                *ptr=ptr.wrapping_add(a);
            }
            else { return Some(*ptr+a) }
        }
        None
    }
}
            
