macro_rules! gen_instructions {
    ($($instr:ty), *) => {
        pub fn run(self, cpu: &mut ::CPU) {
            match (self.opcode(), self.1) {
                $(
                    //<$instr>::OPCODE if  <$instr>::SPECIAL==true => println!("{}", <$instr>::NAME),
                    (<$instr>::OPCODE, <$instr>::SPECIAL) => {
                        if cfg!(debug) {
                            println!("running instruction {}", <$instr>::NAME);
                        }
                        if let Some(cycles) = <$instr>::CYCLES {
                            cpu.cycle(cycles);
                        }
                        <$instr>::run(cpu, self);
                    },
                    )*
                    _ => {}
            }
        }
    }
}


