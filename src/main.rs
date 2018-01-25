extern crate chip8ulator;
extern crate minifb;

use chip8ulator::Chip8;
use minifb::{Key, Scale, Window, WindowOptions};

fn main() {
    let mut chip8 = Chip8::new();
    chip8.load_rom("tests/assets/IBM Logo.ch8").unwrap();
    loop {
        println!("{:?}", chip8);
        chip8.step().unwrap();
    }

    // let mut window = Window::new(
    //     "chip8ulator",
    //     64,
    //     32,
    //     WindowOptions {
    //         scale: Scale::X8,
    //         ..Default::default()
    //     },
    // ).unwrap_or_else(|e| {
    //     panic!("{}", e);
    // });
    //
    // while window.is_open() && !window.is_key_down(Key::Escape) {
    // chip8.step().unwrap();
    // if chip8.redraw {
    //     window.update_with_buffer(&chip8.video_frame()).unwrap();
    // }
    // }
}
