use std::env;
use std::fs::File;
use std::time::Duration;

mod device;
mod cpu;
mod stack;
mod ram;

fn main() {
    let mut dev = device::Device::new_device();

    // Load file into memory
    let args: Vec<String>  = env::args().collect();
    let filename = args.get(1).expect("No ROM filename provided.");
    dev.load_rom(File::open(filename).unwrap());
    println!("Loaded ROM.");
}
