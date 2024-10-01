extern crate image;

use image::{ImageReader, Pixel};
use image::{DynamicImage, GenericImageView};

use crate::colors::Color;



#[derive(Debug, Clone)]
pub struct Texture {
  image: DynamicImage,
  pub width: u32,
  pub height: u32,
}

impl Texture {
  pub fn new(file_path: &str) -> Texture {
    let img = ImageReader::open(file_path).unwrap().decode().unwrap();
    let width = img.width();
    let height = img.height();  

    Texture { image: img, width, height }
  }

  pub fn get_pixel_color(&self, u: f32, v: f32) -> Color {
    // Clamp the UV coordinates to the range [0, 1]
    let u_clamped = u.clamp(0.0, 1.0);
    let v_clamped = v.clamp(0.0, 1.0);


    // Convert normalized UV coordinates to pixel coordinates
    let x = (u_clamped * (self.width as f32 - 1.0)).floor() as u32; 
    let y = (v_clamped * (self.height as f32 - 1.0)).floor() as u32; 

    // Ensure indices are within bounds
    if x < self.width && y < self.height {
        let pixel = self.image.get_pixel(x, y).to_rgb();
        let r = pixel[0];
        let g = pixel[1];
        let b = pixel[2];        
        Color::new(r, g, b)
    } else {
        // Handle out-of-bounds gracefully
        Color::black() // or any default color you prefer
    }
}
  
}
