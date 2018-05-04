#[derive(Debug)]
pub enum OpCode {
    CLS,
    RET,
    JP(u16),
    /// Load address into I register
    LDI(u16),
    /// Set value kk into register x
    LDV(usize, u8),
    /// Dxyn Display n-byte sprite from memory I at Vx, Vy
    /// XOR with current display; set VF 1 on collision, otherwise 0. Wraps
    DRW(usize, usize, usize),
    /// 7xkk Add kk to Vx
    ADD(usize, u8),
    /// Fx55 Store registers V0 through Vx in memory starting at location I.
    LDIV(usize),
    /// Fx65 Read registers V0 through Vx from memory starting at location I.
    LDVI(usize),
    /// Fx1E Set I = I + Vx.
    ADDI(usize),
    /// 3xkk Skip next instruction if Vx = kk.
    SE(usize, u8),
    /// Fx0a Wait for a key press, store the value of the key in Vx.
    LDK(usize),
    /// Cxkk Set Vx = random byte AND kk.
    RND(usize, u8),
    /// 8xy0 Set Vx = Vy.
    LDVV(usize, usize),
    /// 2nnn Call subroutine at nnn.
    CALL(usize),
}

impl OpCode {
    pub fn from_instruction(ins: u16) -> OpCode {
        match ins >> 12 {
            0x0 => match ins & 0xff {
                0xe0 => OpCode::CLS,
                0xee => OpCode::RET,
                _ => unreachable!(),
            },
            0x1 => OpCode::JP(ins & 0xfff),
            0x2 => OpCode::CALL((ins & 0xfff) as usize),
            0xa => OpCode::LDI(ins & 0xfff),
            0x6 => OpCode::LDV(((ins >> 8) & 0xf) as usize, (ins & 0xff) as u8),
            0xd => OpCode::DRW(
                ((ins >> 8) & 0xf) as usize,
                ((ins >> 4) & 0xf) as usize,
                (ins & 0xf) as usize,
            ),
            0x7 => OpCode::ADD(((ins >> 8) & 0xf) as usize, (ins & 0xff) as u8),
            0x8 => match ins & 0xf {
                0x0 => OpCode::LDVV(((ins >> 8) & 0xf) as usize, ((ins >> 4) & 0xf) as usize),
                _ => unreachable!(),
            },
            0xf => match ins & 0xff {
                0x55 => OpCode::LDIV(((ins >> 8) & 0xf) as usize),
                0x65 => OpCode::LDVI(((ins >> 8) & 0xf) as usize),
                0x1e => OpCode::ADDI(((ins >> 8) & 0xf) as usize),
                0x0a => OpCode::LDK(((ins >> 8) & 0xf) as usize),
                _ => unreachable!(),
            },
            0x3 => OpCode::SE(((ins >> 8) & 0xf) as usize, (ins & 0xff) as u8),
            0xc => OpCode::RND(((ins >> 8) & 0xf) as usize, (ins & 0xff) as u8),
            _ => {
                error!("Unimplemented instruction: {:x}", ins);
                panic!();
            }
        }
    }
}
