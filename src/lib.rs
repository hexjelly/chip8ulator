extern crate byteorder;
#[macro_use]
extern crate failure;

mod audio;
mod input;
mod opcodes;

use std::io::Read;
use std::fmt;
use std::fs::File;
use std::path::Path;

use failure::Error;
use byteorder::{BigEndian, ByteOrder};

type BE = BigEndian;

use opcodes::*;

const PX_EMPTY: u32 = 0xff_6e_df_3f;
const PX_FILLED: u32 = 0xff_ff_ff_ff;

#[derive(Debug, Fail, PartialEq)]
pub enum Chip8Error {
    #[fail(display = "ROM is too large: {}", size)] ROMTooLarge {
        size: usize,
    },
    #[fail(display = "Attempt to read out of bounds memory: {}", address)]
    MemOOB {
        address: usize,
    },
}

pub struct Chip8 {
    reg_gpr: [u8; 16],
    reg_i: u16,
    reg_timer_audio: u8,
    reg_timer_delay: u8,
    pc: u16,
    stack: [u16; 16],
    sp: u8,
    mem: [u8; 4096],
    video: [u32; (64 * 32) as usize],
    redraw: bool,
}

impl fmt::Debug for Chip8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Chip8 {{ pc: {} }}", self.pc)
    }
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            reg_gpr: [0; 16],
            reg_i: 0,
            reg_timer_audio: 0,
            reg_timer_delay: 0,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            mem: [0; 4096],
            video: [PX_EMPTY; (64 * 32) as usize],
            redraw: false,
        }
    }

    pub fn reset(&mut self) {
        self.reg_gpr = [0; 16];
        self.reg_i = 0;
        self.reg_timer_audio = 0;
        self.reg_timer_delay = 0;
        self.pc = 0x200;
        self.stack = [0; 16];
        self.sp = 0;
        self.mem = [0; 4096];
        self.video = [PX_EMPTY; (64 * 32) as usize];
    }

    pub fn load_rom<P: AsRef<Path>>(&mut self, file: P) -> Result<(), Error> {
        self.reset();
        let mut file = File::open(file)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        self.mem = into_mem(&buffer)?;
        Ok(())
    }

    pub fn load_rom_from_bytes(&mut self, bytes: &[u8]) -> Result<(), Chip8Error> {
        self.mem = into_mem(bytes)?;
        Ok(())
    }

    pub fn video_frame(&self) -> &[u32] {
        &self.video
    }

    pub fn step(&mut self) -> Result<(), Chip8Error> {
        let pc = self.pc as usize;
        if self.mem.len() < pc {
            return Err(Chip8Error::MemOOB { address: pc });
        }
        let ins_bytes = &self.mem[pc..pc + 2];
        let ins = OpCode::from_instruction(BE::read_u16(ins_bytes));
        println!("{:?}", ins);
        match ins {
            OpCode::CLS => self.redraw = true,
        }
        self.pc += 2;
        Ok(())
    }
}

fn into_mem(bytes: &[u8]) -> Result<[u8; 4096], Chip8Error> {
    if bytes.len() > 3584 {
        return Err(Chip8Error::ROMTooLarge { size: bytes.len() });
    }

    let mut buffer = [0_u8; 4096];

    for (n, byte) in bytes.iter().enumerate() {
        buffer[n + 0x200] = *byte;
    }

    Ok(buffer)
}
