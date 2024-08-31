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

    // Convert color to a hexadecimal representation
    pub fn to_hex(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    // Blend this color with another color
    pub fn blend(&self, other: &Color, factor: f32) -> Color {
        let factor = factor.clamp(0.0, 1.0);
        Color {
            r: (self.r as f32 * (1.0 - factor) + other.r as f32 * factor) as u8,
            g: (self.g as f32 * (1.0 - factor) + other.g as f32 * factor) as u8,
            b: (self.b as f32 * (1.0 - factor) + other.b as f32 * factor) as u8,
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