/* ADD b, a
 * sets b to b+a, sets EX to 0x0001 if there's an overflow, 0x0 otherwise
 * TODO: Set EX
 */
use CPU;
use instruction::*;

#[derive(InstructionInfo)]
#[opcode = 0x02]
#[cycles = 2]
pub struct Add;

impl Instruction for Add {
    fn run(cpu: &mut CPU, instr: Instr) {
        let a: u16;
        let mut ex = 0;

        match instr.arg_a(cpu).into_val() {
            Val(val) => a = val,
            _ => a = 0
        }
        if let Ptr(ptr) = instr.arg_b(cpu) { //Don't write if it's a literal
            let sum = ptr.checked_add(a);
            if let Some(sum) = sum {
                *ptr=sum;
            }
            else { *ptr = 0; ex = 1 }
        }
        cpu.excess = ex;
    }
}
            
