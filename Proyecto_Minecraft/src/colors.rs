// color.rs

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    // Constructor to create a new Color
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }


    pub  const  fn black() -> Self {
        Color {r:0, g:0,b:0}
    }

    pub  const  fn skybox_color() -> Self {
        Color {r:141, g:162,b:255}
    }

    // Convert color to a hexadecimal representation
    pub fn to_hex(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }


    // Blend this color with another color
    pub fn blend(&self, other: &Color) -> Color {
        Color {
            r: (self.r as f32 + other.r as f32 ) as u8,
            g: (self.g as f32 + other.g as f32 ) as u8,
            b: (self.b as f32 + other.b as f32 ) as u8,
        }
    }

    // Adjust color brightness
    pub fn adjust_brightness(&self, factor: f32) -> Color {
        Color {
            r: ((self.r as f32 * factor).clamp(0.0, 255.0)) as u8,
            g: ((self.g as f32 * factor).clamp(0.0, 255.0)) as u8,
            b: ((self.b as f32 * factor).clamp(0.0, 255.0)) as u8,
        }
    }
}