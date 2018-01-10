mod cpu;
mod audio;
mod video;
mod input;
mod mem;
mod opcodes;

use cpu::Cpu;
use mem::Mem;
use opcodes::OpCode;

use std::path::Path;
use std::fs::File;
use std::io::Read;

use std::io;

pub struct Instance {
    cpu: Cpu,
    mem: Mem,
    rom: Box<[u8]>,
}

impl Instance {
    pub fn load_rom<P: AsRef<Path>>(&mut self, file: P) -> io::Result<()> {
        let mut file = File::open(file)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        self.rom = buffer.into();
        Ok(())
    }
}
