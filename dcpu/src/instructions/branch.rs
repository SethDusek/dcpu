//This file contains all of the control flow/if instructions
use CPU;
use instruction::*;

//performs next instruction only if (b&a)!=0
#[derive(InstructionInfo)]
#[opcode = 0x10]
#[cycles = 2]
pub struct Ifb;


impl Instruction for Ifb {
    fn run(cpu: &mut CPU, instr: Instr) {
        let a = if let Val(val) = instr.arg_a(cpu).into_val() { val } else {return};
        let b = if let Val(val) = instr.arg_b(cpu).into_val() { val } else {return};
        if (b&a)==0 { //Skip next instruction if (b&a)==0
            skip(cpu);
        }
    }
}

#[derive(InstructionInfo)]
#[opcode = 0x11]
#[cycles = 2]
pub struct Ifc;

impl Instruction for Ifc {
    fn run(cpu: &mut CPU, instr: Instr) {        
        let a = if let Val(val) = instr.arg_a(cpu).into_val() { val } else {return};
        let b = if let Val(val) = instr.arg_b(cpu).into_val() { val } else {return};
        if (b&a)!=0 {
            skip(cpu);
        }
    }
}


//performs next instruction only if b==a
#[derive(InstructionInfo)]
#[opcode = 0x12]
#[cycles = 2]
pub struct Ife;


impl Instruction for Ife {
    fn run(cpu: &mut CPU, instr: Instr) {
        let a = if let Val(val) = instr.arg_a(cpu).into_val() { val } else {return};
        let b = if let Val(val) = instr.arg_b(cpu).into_val() { val } else {return};
        if a != b { //Skip next instruction if a==b = false
            skip(cpu);
        }
    }
}

//performs next instruction only if b!=a
#[derive(InstructionInfo)]
#[opcode = 0x13]
#[cycles = 2]
pub struct Ifn;

impl Instruction for Ifn {
    fn run(cpu: &mut CPU, instr: Instr) {
        let a = if let Val(val) = instr.arg_a(cpu).into_val() { val } else {return};
        let b = if let Val(val) = instr.arg_b(cpu).into_val() { val } else {return};
        if a==b {
            skip(cpu);
        }
    }
}

#[derive(InstructionInfo)]
#[opcode = 0x14]
#[cycles = 2]
pub struct Ifg;

impl Instruction for Ifg {
    fn run(cpu: &mut CPU, instr: Instr) {
        let a = if let Val(val) = instr.arg_a(cpu).into_val() { val } else {return};
        let b = if let Val(val) = instr.arg_b(cpu).into_val() { val } else {return};
        if b < a {
            skip(cpu);
        }
    }
}

#[derive(InstructionInfo)]
#[opcode = 0x15]
#[cycles = 2]
pub struct Ifa;

impl Instruction for Ifa {
    fn run(cpu: &mut CPU, instr: Instr) {
        let a = if let Val(val) = instr.arg_a(cpu).into_val() { val as i16 } else {return};
        let b = if let Val(val) = instr.arg_b(cpu).into_val() { val as i16 } else {return};
        if b < a  {// skip next instruction if b < a (signed) 
            skip(cpu);
        }
    }
}


#[derive(InstructionInfo)]
#[opcode = 0x16]
#[cycles = 2]
pub struct Ifl;

impl Instruction for Ifl {
    fn run(cpu: &mut CPU, instr: Instr) {
        let a = if let Val(val) = instr.arg_a(cpu).into_val() { val as i16 } else {return};
        let b = if let Val(val) = instr.arg_b(cpu).into_val() { val as i16 } else {return};
        if b > a { // skip next instruction if b > a 
            skip(cpu);
        }
    }
}

#[derive(InstructionInfo)]
#[opcode = 0x17]
#[cycles = 2]
pub struct Ifu;

impl Instruction for Ifu {
    fn run(cpu: &mut CPU, instr: Instr) {
        let a = if let Val(val) = instr.arg_a(cpu).into_val() { val as i16 } else {return};
        let b = if let Val(val) = instr.arg_b(cpu).into_val() { val as i16 } else {return};
        if b > a { //skip next instruction if b > a (signd)
            skip(cpu);
        }
    }
}





//Check to see whether an instruction is an IFB/IFC/IFE/IFN/IFG/IFA/IFL/IFU instruction
fn is_if(instr: Instr) -> bool{
    let opcode = instr.opcode();
    match opcode {
        0x10...0x17 => true,
        _ => false
    }
}
//Skips the next instruction, or 2 instructions 
fn skip(cpu: &mut CPU) {
    let instr = cpu.next();
    cpu.cycle(1);
    if is_if(instr) {
        cpu.cycle(1);
        cpu.next();
    }
}

