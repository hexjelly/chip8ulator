extern crate chip8ulator;

#[macro_use]
extern crate log;
extern crate minifb;
extern crate simplelog;
#[macro_use]
extern crate structopt;

use chip8ulator::{Chip8, Key};
use minifb::{Key as mfbKey, KeyRepeat, Scale, Window, WindowOptions};
use simplelog::*;
use std::path::PathBuf;
use structopt::StructOpt;

const PX_EMPTY: u32 = 0;
const PX_FILLED: u32 = 0xff_6e_df_3f;

/// A basic example
#[derive(StructOpt, Debug)]
struct Opt {
    /// Activate debug mode
    #[structopt(short = "d", long = "debug")]
    debug: bool,

    /// Activate rendering mode
    #[structopt(short = "r", long = "render")]
    render: bool,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    file: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    let _ = TermLogger::init(
        match opt.debug {
            true => LevelFilter::Trace,
            false => LevelFilter::Off,
        },
        Config::default(),
    );

    let mut chip8 = Chip8::new();
    chip8.load_rom(opt.file).unwrap();

    if opt.render {
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
        let mut buffer: [u32; 64 * 32] = [PX_EMPTY; 64 * 32];
        window.update_with_buffer(&buffer).unwrap();

        while window.is_open() && !window.is_key_down(mfbKey::Escape) {
            if window.is_key_pressed(mfbKey::A, KeyRepeat::Yes) {
                chip8.key(&Key::KeyA);
            }
            if window.is_key_pressed(mfbKey::N, KeyRepeat::Yes) {
                debug!("{:?}", chip8);
                chip8.step().unwrap();
            }
            if chip8.redraw {
                for (i, px) in chip8
                    .video_frame()
                    .iter()
                    .map(|px| if *px { PX_FILLED } else { PX_EMPTY })
                    .enumerate()
                {
                    buffer[i] = px;
                }
                window.update_with_buffer(&buffer).unwrap();
            } else {
                window.update();
            }
        }
    } else {
        loop {
            debug!("{:?}", chip8);
            chip8.step().unwrap();
        }
    }
}
