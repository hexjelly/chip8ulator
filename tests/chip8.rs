extern crate chip8ulator;

use chip8ulator::{Chip8, Chip8Error};

#[test]
fn loads_valid_rom_correctly() {
    let mut chip8 = Chip8::new();
    chip8.load_rom("tests/assets/IBM Logo.ch8").unwrap();
}

#[test]
fn rom_loading_from_buffer() {
    let mut chip8 = Chip8::new();
    let rom = vec![0; 3584];
    let res = chip8.load_rom_from_bytes(&rom);
    assert!(res.is_ok());
}

#[test]
fn rom_loading_from_buffer_too_large() {
    let mut chip8 = Chip8::new();
    let rom = vec![0; 3585];
    let res = chip8.load_rom_from_bytes(&rom);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), Chip8Error::ROMTooLarge { size: 3585 });
}
