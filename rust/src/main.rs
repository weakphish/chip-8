use std::env;
use std::fs::File;

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod cpu;
mod ram;
mod stack;

use crate::cpu::CPU;
use crate::ram::RAM;
use crate::stack::Stack;

pub const DISPLAY_HEIGHT: usize = 32;
pub const DISPLAY_WIDTH: usize = 64;

fn main() -> Result<(), Error> {
    // Instantiate device components
    let mut ram = RAM::new();
    let mut stack = Stack::new();
    let mut vram = [[u8::from(0); DISPLAY_WIDTH]; DISPLAY_HEIGHT]; // access at vram[y][x]
    let mut cpu = CPU::new();

    // Load file into memory
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("No ROM filename provided.");
    ram.load_rom(File::open(filename).unwrap());
    println!("Loaded ROM.");

    // Setup Pixels context
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(DISPLAY_WIDTH as f64, DISPLAY_HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Chip-8")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(DISPLAY_WIDTH as u32, DISPLAY_HEIGHT as u32, surface_texture)?
    };

    // Main event loop
    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame from the contents of the VRAM
        if let Event::RedrawRequested(_) = event {
            let frame = pixels.get_frame();
            // Update frame with contents of device's VRAM
            for (chunk, vram_pix) in frame.chunks_exact_mut(4).zip(vram.iter().flatten()) {
                let new_chunk = match *vram_pix {
                    0 => [0, 0, 0, 255],
                    _ => [255, 255, 255, 255],
                };
                chunk.copy_from_slice(&new_chunk);
            }
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            // Update internal state and request a redraw
            cpu.emulate_cycle(&input, &mut stack, &mut vram, &mut ram);
            window.request_redraw();
        }
    });
}
