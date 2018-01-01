use CPU;
use instruction::*;
#[derive(InstructionInfo)]
#[opcode = 0x04]
#[cycles = 2]
struct Mul;


/*impl Instruction for Mul [
    fn run(cpu: &mut CPU, instr: Instr) {
        let a = if let Val(val) = instr.arg_a().into_val() {
            val
        }
        else { return }
        if let Ptr(ptr) = instr.arg_b() {
*/
