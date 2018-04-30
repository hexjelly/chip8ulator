extern crate chip8ulator;

#[macro_use]
extern crate log;
extern crate minifb;
extern crate simplelog;

use chip8ulator::Chip8;
use minifb::{Key, KeyRepeat, Scale, Window, WindowOptions};
use simplelog::*;

const PX_EMPTY: u32 = 0;
const PX_FILLED: u32 = 0xff_6e_df_3f;

fn main() {
    let _ = TermLogger::init(LevelFilter::Trace, Config::default());

    let mut chip8 = Chip8::new();
    chip8.load_rom("tests/assets/HIDDEN.ch8").unwrap();
    // loop {
    //     debug!("{:?}", chip8);
    //     chip8.step().unwrap();
    // }

    let mut window = Window::new(
        "chip8ulator",
        64,
        32,
        WindowOptions {
            scale: Scale::X8,
            ..Default::default()
        },
    ).expect("Could not create window");

    debug!("{:?}", chip8);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_pressed(Key::N, KeyRepeat::No) {
            chip8.step().unwrap();
            debug!("{:?}", chip8);
        }
        // if chip8.redraw {
        let buffer: Vec<u32> = chip8
            .video_frame()
            .iter()
            .map(|px| if *px { PX_FILLED } else { PX_EMPTY })
            .collect();
        window.update_with_buffer(&buffer).unwrap();
        // }
    }
}
