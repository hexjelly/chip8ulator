extern crate byteorder;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;

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

const FONT_SET: [u8; 80] = [
    0xf0, 0x90, 0x90, 0x90, 0xf0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xf0, 0x10, 0xf0, 0x80, 0xf0, 0xf0,
    0x10, 0xf0, 0x10, 0xf0, 0xa0, 0xa0, 0xf0, 0x20, 0x20, 0xf0, 0x80, 0xf0, 0x10, 0xf0, 0xf0, 0x80,
    0xf0, 0x90, 0xf0, 0xf0, 0x10, 0x20, 0x40, 0x40, 0xf0, 0x90, 0xf0, 0x90, 0xf0, 0xf0, 0x90, 0xf0,
    0x10, 0xf0, 0xf0, 0x90, 0xf0, 0x90, 0x90, 0xe0, 0x90, 0xe0, 0x90, 0xe0, 0xf0, 0x80, 0x80, 0x80,
    0xf0, 0xe0, 0x90, 0x90, 0x90, 0xe0, 0xf0, 0x80, 0xf0, 0x80, 0xf0, 0xf0, 0x80, 0xf0, 0x80, 0x80,
];

#[derive(Debug, Copy, Clone)]
pub enum Key {
    Key0 = 0x0,
    Key1 = 0x1,
    Key2 = 0x2,
    Key3 = 0x3,
    Key4 = 0x4,
    Key5 = 0x5,
    Key6 = 0x6,
    Key7 = 0x7,
    Key8 = 0x8,
    Key9 = 0x9,
    KeyA = 0xA,
    KeyB = 0xB,
    KeyC = 0xC,
    KeyD = 0xD,
    KeyE = 0xE,
    KeyF = 0xF,
}

#[derive(Debug, Fail, PartialEq)]
pub enum Chip8Error {
    #[fail(display = "ROM is too large: {}", size)]
    ROMTooLarge { size: usize },
    #[fail(display = "Attempt to read out of bounds memory: {}", address)]
    MemOOB { address: usize },
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
    video: [bool; (64 * 32)],
    pub redraw: bool,
    key_waiting: bool,
    key_reg: usize,
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
            mem: new_mem(),
            video: [false; (64 * 32)],
            redraw: false,
            key_waiting: false,
            key_reg: 0,
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
        self.mem = new_mem();
        self.video = [false; (64 * 32)];
        self.redraw = false;
        self.key_waiting = false;
        self.key_reg = 0;
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

    pub fn video_frame(&mut self) -> &[bool] {
        &self.video
    }

    pub fn key(&mut self, key: &Key) {
        debug!("Got keypress: {:?}", key);
        if self.key_waiting {
            self.reg_gpr[self.key_reg] = *key as u8;
            self.key_waiting = false;
            debug!("Set reg[{:?}] to: {:?}", self.key_reg, *key as u8);
        }
    }

    pub fn step(&mut self) -> Result<(), Chip8Error> {
        if self.key_waiting {
            debug!("Waiting for keypress...");
            return Ok(());
        }
        let pc = self.pc as usize;
        if self.mem.len() < pc {
            return Err(Chip8Error::MemOOB { address: pc });
        }
        let ins;
        {
            let ins_bytes = &self.mem[pc..pc + 2];
            ins = OpCode::from_instruction(BE::read_u16(ins_bytes));
        }
        self.pc += 2;
        debug!("Executing instruction: {:?}", ins);
        match ins {
            OpCode::CLS => {
                // Clear screen and set redraw flag to true
                self.video = [false; (64 * 32) as usize];
                self.redraw = true;
            }
            OpCode::RET => {
                // Set PC to top stack address, and decrement SP
                self.pc = self.stack[self.sp as usize];
                self.sp -= 1;
            }
            OpCode::JP(addr) => {
                // Set PC to address
                self.pc = addr;
            }
            OpCode::LDI(addr) => {
                // Set I register to address
                self.reg_i = addr;
            }
            OpCode::LDV(reg, val) => {
                // Load value into register
                self.reg_gpr[reg] = val;
            }
            OpCode::DRW(x, y, rows) => {
                let x = self.reg_gpr[x] as usize;
                let y = self.reg_gpr[y] as usize;
                debug!("DRW from X = {}, Y = {}, {} rows", x, y, rows);
                // for each line (byte) in sprite, check each bit (pixel)
                for row in 0..rows {
                    let line = self.mem[self.reg_i as usize + row];
                    debug!("line {}: {:08b}", row, line);
                    for (n, bit) in (0..8).rev().enumerate() {
                        let mut x = x + n;
                        if x > 64 {
                            x -= 64;
                        }
                        let value = line & (1 << bit) != 0;
                        let v_address = x + (64 * (y + row));

                        self.reg_gpr[15] = if self.video[v_address] != self.video[v_address] ^ value
                        {
                            1
                        } else {
                            0
                        };
                        self.video[v_address] ^= value;
                    }
                }
                self.redraw = true;
            }
            OpCode::ADD(reg, val) => {
                // Add val to reg
                self.reg_gpr[reg] += val;
            }
            OpCode::LDIREGS(regs) => {
                // Store registers V0 through Vx in memory starting at location I
                for i in 0..regs {
                    self.mem[self.reg_i as usize + i] = self.reg_gpr[i];
                }
            }
            OpCode::ADDI(reg) => {
                // Set I = I + Vx.
                self.reg_i += self.reg_gpr[reg] as u16;
            }
            OpCode::SE(reg, val) => {
                // Skip next instruction if Vx = kk.
                if self.reg_gpr[reg] == val {
                    self.pc += 2;
                }
            }
            OpCode::LDK(reg) => {
                self.key_waiting = true;
                self.key_reg = reg;
            }
            _ => {
                error!("Unimplemented handling of instruction: {:?}", ins);
                panic!();
            }
        }
        Ok(())
    }
}

fn new_mem() -> [u8; 4096] {
    let mut mem = [0; 4096];
    for (n, byte) in FONT_SET.iter().enumerate() {
        mem[n] = *byte;
    }
    mem
}

fn into_mem(bytes: &[u8]) -> Result<[u8; 4096], Chip8Error> {
    if bytes.len() > 3584 {
        return Err(Chip8Error::ROMTooLarge { size: bytes.len() });
    }

    let mut buffer = new_mem();

    for (n, byte) in bytes.iter().enumerate() {
        buffer[n + 0x200] = *byte;
    }

    Ok(buffer)
}
