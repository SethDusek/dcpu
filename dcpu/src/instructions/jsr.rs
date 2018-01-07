/* jsr.rs 
 * pushes the address of the next instruction to the stack, then sets PC to a
 * cycles: 3
 * opcode: 0x1 (special instruction)
 */
use CPU;
use instruction::*;


#[derive(InstructionInfo)]
#[cycles = 3]
#[opcode = 0x1]
#[special = true]
pub struct Jsr;


impl Instruction for Jsr {
    fn run(cpu: &mut CPU, instr: Instr) {
        let pc = cpu.pc;
        cpu.push(pc); //Reads the next instruction, turns it into a u16 and pushes it
        if let Val(val) = instr.arg_a(cpu).into_val() {
            println!("jsring to {}", val);
            cpu.pc=val;
        }
    }
}
