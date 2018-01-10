pub struct Cpu {
    reg_gpr: [u8; 16],
    reg_i: u16,
    reg_timer_audio: u8,
    reg_timer_delay: u8,
    pc: u16,
    sp: u8,
}
