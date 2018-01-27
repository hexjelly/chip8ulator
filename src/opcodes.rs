/// 00E0 Clear the display
pub const CLS: u16 = 0b0000_0000_1110_0000;
/// 00EE Return from subroutine
pub const RET: u16 = 0b0000_0000_1110_1110;
/// 1nnn Jump to address nnn
pub const JP: u16 = 0b0001;
/// Annn Load address into I register
pub const LDI: u16 = 0b1010;
/// 6xkk Set value kk into register x
pub const LDV: u16 = 0b0110;
/// Dxyn Display n-byte sprite from memory I at Vx, Vy
/// XOR with current display; set VF 1 on collision, otherwise 0. Wraps
pub const DRW: u16 = 0b1101;
/// 7xkk Add kk to Vx
pub const ADD: u16 = 0b0111;

#[derive(Debug)]
pub enum OpCode {
    CLS,
    RET,
    JP(u16),
    LDI(u16),
    LDV(usize, u8),
    DRW(usize, usize, usize),
    ADD(usize, u8),
}

impl OpCode {
    pub fn from_instruction(ins: u16) -> OpCode {
        match ins >> 12 {
            JP => OpCode::JP(ins & 0xfff),
            LDI => OpCode::LDI(ins & 0xfff),
            LDV => OpCode::LDV(((ins >> 8) & 0xf) as usize, (ins & 0xff) as u8),
            DRW => OpCode::DRW(
                ((ins >> 8) & 0xf) as usize,
                ((ins >> 4) & 0xf) as usize,
                (ins & 0xf) as usize,
            ),
            ADD => OpCode::ADD(((ins >> 8) & 0xf) as usize, (ins & 0xff) as u8),
            _ => match ins {
                CLS => OpCode::CLS,
                RET => OpCode::RET,
                _ => {
                    error!("Unimplemented instruction: {:x}", ins);
                    panic!();
                }
            },
        }
    }
}
