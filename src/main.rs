use std::env;
use std::fs::File;

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod device;
mod cpu;
mod stack;
mod ram;

fn main() -> Result<(), Error> {
    // Create the struct that represents the physical device state
    let mut dev = device::Device::new_device();

    // Load file into memory
    let args: Vec<String>  = env::args().collect();
    let filename = args.get(1).expect("No ROM filename provided.");
    dev.load_rom(File::open(filename).unwrap());
    println!("Loaded ROM.");

    // Setup Pixels context
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(device::DISPLAY_WIDTH as f64, device::DISPLAY_HEIGHT as f64);
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
        Pixels::new(device::DISPLAY_WIDTH as u32, device::DISPLAY_HEIGHT as u32, surface_texture)?
    };

    // Main event loop
    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            let frame = pixels.get_frame();
            // Update frame with contents of device's VRAM
            if dev.get_vram_changed() {
                let vram_snap = dev.get_vram();
                for (vram_pixel, frame_pixel) in vram_snap.iter().flatten().zip(frame.chunks_exact_mut(4)) {
                    let chunk = if *vram_pixel {
                        [255, 255, 255, 255]
                    } else {
                        [0, 0, 0, 255]
                    };
                    frame_pixel.copy_from_slice(&chunk);
                    println!("Set {:?} to {:?}", frame_pixel, vram_pixel);
                }
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
            dev.emulate_cycle();
            window.request_redraw();
        }
    });
}
