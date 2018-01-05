use CPU;
use instruction::*;

#[derive(InstructionInfo)]
#[opcode = 0x1b]
#[cycles = 3]
pub struct Sbx;



impl Instruction for Sbx {
    fn run(cpu: &mut CPU, instr: Instr) {
        let mut ex = 0;
        let cpu_ex = cpu.excess;
        let a = if let Val(val) = instr.arg_a(cpu).into_val() {
            val
        }
        else {return};
        if let Ptr(ptr) = instr.arg_b(cpu) {
            if let Some(sum) = cpu_ex.checked_add(a) {
                if let Some(sub) = sum.checked_sub(*ptr) {
                    *ptr=sub
                }
                else { ex = 0xffff }
            }
            else { ex = 0xffff }
        }
        cpu.excess = ex
    }
}
        



