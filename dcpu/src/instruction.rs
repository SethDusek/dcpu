#[derive(Copy, Clone)]
pub struct Instr(u16, bool);
///Argument type for DCPU
#[derive(Debug)]
pub enum Argument<'a> {
    Ptr(&'a mut u16),
    Val(u16),
    None
}

impl <'a> Argument <'a> {
    ///Derefs a Ptr into a value, or does nothing if it's a value 
    pub fn into_val(self) -> Self {
        if let Ptr(ptr) = self {
            Argument::Val(*ptr)
        }
        else {
            self
        }
    }
    ///Returns true if the pointers point to the same address
    pub fn cmp_ptr(&self, addr: &u16) -> bool {
        if let Ptr(ref ptr) = *self {
            ::std::ptr::eq(*ptr, addr)
        }
        else { false }
    }
}
pub use self::Argument::{Ptr, Val};

impl Instr {
    pub fn new(instr: u16) -> Self {
        let mut special = false;
        if (instr & 0b11111) == 0 {
            special = true;
        }
        Instr(instr, special)
    }
    pub fn opcode(self) -> u8 {
        if !self.1 {
        (self.0 & 0b11111) as u8
        }
        else {
            (self.0 & (0b11111 << 5)) as u8
        }
    }
    ///Returns the word that this Instr represents
    pub fn word(self) -> u16 {
        self.0
    }
    fn a(self) -> u8 {
        ((self.0 >> 10) & 0b111111) as u8
    }
    fn b(self) -> Option<u8> {
        if !self.1 {
            Some(((self.0 >> 5) & 0b11111) as u8)
        }
        else { None }
    }
    fn parse_arg(cpu: &mut ::CPU, arg: u8) -> Argument {
        match arg {
            0x00..0x07 => Ptr(&mut cpu.registers()[arg]),
            0x08..0x0f => Ptr(&mut cpu.memory[cpu.registers()[arg-8] as usize]), //[register]
            0x10..0x17 => Ptr(&mut cpu.memory[(cpu.registers()[arg-10] + cpu.next().word()) as usize]), // [register + next word]
            0x19 => Ptr(&mut cpu.memory[cpu.sp as usize]),
            0x1b => Ptr(&mut cpu.sp),
            0x1c => Ptr(&mut cpu.pc),
            0x1d => Ptr(&mut cpu.excess),
            0x1e => Ptr(&mut cpu.memory[cpu.next().word() as usize]),
            0x1f => Val(cpu.next().word()),
            _ => Argument::None
        }
    }
    pub fn arg_a(self, cpu: &mut ::CPU) -> Argument {
        let arg = self.a();
        match arg {
            0x18 => Val(cpu.pop()),
            0x20..0x3f => Val((arg as u16).wrapping_sub(0x21)),
            _ => Self::parse_arg(cpu, arg)
        }.into_val()
    }
    pub fn arg_b(self, cpu: &mut ::CPU) -> Argument {
        if self.1 == true {
            return Argument::None
        }
        let arg = self.b().unwrap();
        match arg {
            0x18 => {cpu.sp-=1; Ptr(&mut cpu.memory[cpu.sp as usize])},
            _ => Self::parse_arg(cpu, arg)
        }
    }
    gen_instructions!(::instructions::Set, ::instructions::Add, ::instructions::Sub);

}


//trait that instructions should implement with Instruction
//TODO: improve the design, take feedback, etc
pub trait InstructionInfo {
    const OPCODE: u8;
    const NAME: &'static str;
    const SPECIAL: bool;
    //responsible for proper cycles itself
    const CYCLES: Option<u64>;
}


pub trait Instruction: InstructionInfo {
    ///Run the instruction. The return type is the program counter. If it is None, the DCPU will
    ///automatically increase the program counter by 1
    fn run(cpu: &mut ::CPU, instr: Instr);
}
