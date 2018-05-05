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
    /// Fx29 Set I = location of sprite for digit Vx.
    LDF(usize),
    /// 4xkk Skip next instruction if Vx != kk.
    SNE(usize, u8),
    /// Fx33 Store BCD representation of Vx in memory locations I, I+1, and I+2.
    LDB(usize),
    /// 8xy4 Set Vx = Vx + Vy, set VF = carry.
    ADDXY(usize, usize),
    /// 8xy5 Set Vx = Vx - Vy, set VF = NOT borrow.
    SUBXY(usize, usize),
    /// 8xy7 Set Vx = Vy - Vx, set VF = NOT borrow.
    SUBN(usize, usize),
    /// 8xy6 Set Vx = Vx SHR 1.
    SHR(usize),
    /// 8xyE Set Vx = Vx SHL 1.
    SHL(usize),
    /// 8xy3 Set Vx = Vx XOR Vy.
    XOR(usize, usize),
    /// Fx15 Set delay timer = Vx.
    LDDT(usize),
}

impl OpCode {
    pub fn from_instruction(ins: u16) -> OpCode {
        match ins >> 12 {
            0x0 => match ins & 0xff {
                0xe0 => OpCode::CLS,
                0xee => OpCode::RET,
                _ => {
                    error!("Unimplemented instruction: {:x}", ins);
                    panic!();
                }
            },
            0x1 => OpCode::JP(ins & 0xfff),
            0x2 => OpCode::CALL((ins & 0xfff) as usize),
            0x4 => OpCode::SNE(((ins >> 8) & 0xf) as usize, (ins & 0xff) as u8),
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
                0x3 => OpCode::XOR(((ins >> 8) & 0xf) as usize, ((ins >> 4) & 0xf) as usize),
                0x4 => OpCode::ADDXY(((ins >> 8) & 0xf) as usize, ((ins >> 4) & 0xf) as usize),
                0x5 => OpCode::SUBXY(((ins >> 8) & 0xf) as usize, ((ins >> 4) & 0xf) as usize),
                0x6 => OpCode::SHR(((ins >> 8) & 0xf) as usize),
                0x7 => OpCode::SUBN(((ins >> 8) & 0xf) as usize, ((ins >> 4) & 0xf) as usize),
                0xe => OpCode::SHL(((ins >> 8) & 0xf) as usize),
                _ => {
                    error!("Unimplemented instruction: {:x}", ins);
                    panic!();
                }
            },
            0xf => match ins & 0xff {
                0x15 => OpCode::LDDT(((ins >> 8) & 0xf) as usize),
                0x29 => OpCode::LDF(((ins >> 8) & 0xf) as usize),
                0x33 => OpCode::LDB(((ins >> 8) & 0xf) as usize),
                0x55 => OpCode::LDIV(((ins >> 8) & 0xf) as usize),
                0x65 => OpCode::LDVI(((ins >> 8) & 0xf) as usize),
                0x1e => OpCode::ADDI(((ins >> 8) & 0xf) as usize),
                0x0a => OpCode::LDK(((ins >> 8) & 0xf) as usize),
                _ => {
                    error!("Unimplemented instruction: {:x}", ins);
                    panic!();
                }
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
