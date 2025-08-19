use ddsfile::Dds;
use image_dds::image_from_dds;
use minifb::{Key, Window, WindowOptions};
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = std::env::args().nth(1).expect("Usage: <file.dds>");

    let mut file = File::open(&filename).expect("Failed to open file");
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Failed to read file");

    let dds = Dds::read(&mut &data[..]).expect("Failed to parse DDS");
    let img = image_from_dds(&dds, 0)?;
    let (width, height) = (img.width() as usize, img.height() as usize);
    let raw = img.into_raw();

    println!("Loaded DDS: {}x{}", width, height);

    let buffer: Vec<u32> = raw
        .chunks_exact(4)
        .map(|c| {
            let r = c[0] as u32;
            let g = c[1] as u32;
            let b = c[2] as u32;
            let a = c[3] as u32;
            (a << 24) | (r << 16) | (g << 8) | b
        })
        .collect();

    let mut window = Window::new(&filename, width, height, WindowOptions::default())
        .expect("Failed to open window");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, width, height)?;
    }

    Ok(())
}
