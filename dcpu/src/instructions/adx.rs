//sets b to b+a+EX, sets EX to 0x0001 if there is an overflow, 0x0 otherwise
use CPU;
use instruction::*;


#[derive(InstructionInfo)]
#[opcode = 0x1a]
#[cycles = 3]
pub struct Adx;


impl Instruction for Adx {
    fn run(cpu: &mut CPU, instr: Instr) {
        let mut ex = 0;
        let cpu_ex = cpu.excess;
        let a = if let Val(val) = instr.arg_a(cpu).into_val() {
            val
        }
        else {return};
        if let Ptr(ptr) = instr.arg_b(cpu) {
            if let Some(sum) = ptr.checked_add(a) { //first sum was successful, now move onto adding excess
                if let Some(sum2) = sum.checked_add(cpu_ex) {
                    *ptr=sum2
                }
                else { ex = 0x1 }
            }
            else { ex = 0x1 }
        }
        cpu.excess = ex;
    }
}

