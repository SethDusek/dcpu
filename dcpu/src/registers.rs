#[derive(Debug)]
pub struct Registers {
    _reg: [u16; 8]
}
pub enum Register {
    A,
    B,
    C,
    X,
    Y,
    Z,
    I,
    J
}


impl Registers {
    pub fn new() -> Self {
        Registers{ _reg: [0; 8] }
    }
}
//TODO: merge both functions into one and clean it up, maybe not necessary but would be nice
impl ::std::ops::Index<Register> for Registers {
    type Output = u16;
    fn index(&self, idx: Register) -> &Self::Output {
        match idx {
            Register::A => &self._reg[0],
            Register::B => &self._reg[1],
            Register::C => &self._reg[2],
            Register::X => &self._reg[3],
            Register::Y => &self._reg[4],
            Register::Z => &self._reg[5],
            Register::I => &self._reg[6],
            Register::J => &self._reg[7]
        }
    }
}


impl ::std::ops::IndexMut<Register> for Registers {
    fn index_mut(&mut self, idx: Register) -> &mut Self::Output {
        match idx {
            Register::A => &mut self._reg[0],
            Register::B => &mut self._reg[1],
            Register::C => &mut self._reg[2],
            Register::X => &mut self._reg[3],
            Register::Y => &mut self._reg[4],
            Register::Z => &mut self._reg[5],
            Register::I => &mut self._reg[6],
            Register::J => &mut self._reg[7]
        }
    }
}

impl ::std::ops::Index<u8> for Registers {
    type Output = u16;
    fn index(&self, idx: u8) -> &Self::Output {
        &self._reg[idx as usize]
    }
}

impl ::std::ops::IndexMut<u8> for Registers {
    fn index_mut(&mut self, idx: u8) -> &mut Self::Output {
        &mut self._reg[idx as usize]
    }
}

        
