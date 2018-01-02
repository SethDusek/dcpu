/* MOD b,a 
 * sets b to b%a. if a==0, sets b to 0 instead.
 */

use CPU;
use instruction::*;


#[derive(InstructionInfo)]
#[opcode=0x08]
#[cycles=3]
pub struct Mod;

impl Instruction for Mod {
    fn run(cpu: &mut CPU, instr: Instr) {
        let a = if let Val(val) = instr.arg_a(cpu).into_val() {
            val
        }
        else {return};

        if let Ptr(ptr) = instr.arg_b(cpu) {
            if a == 0 { *ptr=0 }
            else { *ptr=*ptr%a }
        }
    }
}

#[derive(InstructionInfo)]
#[opcode=0x09]
#[cycles=3]
pub struct Mdi;


impl Instruction for Mdi {
    fn run(cpu: &mut CPU, instr: Instr) {
        let a = if let Val(val) = instr.arg_a(cpu).into_val() {
            val as i16
        }
        else {return};

        if let Ptr(ptr) = instr.arg_b(cpu) {
            if a == 0 { *ptr = 0 }
            else { *ptr=((*ptr as i16)%a) as u16 }
        }
    }
}
