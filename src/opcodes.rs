pub const CLS: u16 = 0b0000_0000_1110_0000;

#[derive(Debug)]
pub enum OpCode {
    CLS,
}

impl OpCode {
    pub fn from_instruction(ins: u16) -> OpCode {
        match ins {
            CLS => OpCode::CLS,
            _ => unimplemented!(),
        }
    }
}
