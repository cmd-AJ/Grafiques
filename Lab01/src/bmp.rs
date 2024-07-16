use std::fs::File;
use std::io::{self, Write};

pub fn write_bmp_file(file_path: &str, buffer: &[u32], width: usize, height: usize) -> io::Result<()> {
    let mut file = File::create(file_path)?;

    // BMP file header
    file.write_all(b"BM")?;
    file.write_all(&((54 + buffer.len() * 4) as u32).to_le_bytes())?;
    file.write_all(&[0; 4])?;
    file.write_all(&54u32.to_le_bytes())?;

    // DIB header
    file.write_all(&40u32.to_le_bytes())?;
    file.write_all(&(width as i32).to_le_bytes())?;
    file.write_all(&(height as i32).to_le_bytes())?;
    file.write_all(&1u16.to_le_bytes())?;
    file.write_all(&32u16.to_le_bytes())?;
    file.write_all(&[0; 24])?;

    // Pixel data
    for &pixel in buffer.iter() {
        file.write_all(&pixel.to_le_bytes())?;
    }

    Ok(())
}
