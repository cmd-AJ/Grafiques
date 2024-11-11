// color.rs

#[derive(Debug, Clone, Copy, PartialEq)]
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

    pub fn lerp(&self, other: &Color, t: f32) -> Color {
        let r = (self.r as f32 + t * (other.r as f32 - self.r as f32))
            .round()
            .clamp(0.0, 255.0) as u8;
        let g = (self.g as f32 + t * (other.g as f32 - self.g as f32))
            .round()
            .clamp(0.0, 255.0) as u8;
        let b = (self.b as f32 + t * (other.b as f32 - self.b as f32))
            .round()
            .clamp(0.0, 255.0) as u8;
        Color::new(r, g, b)
    }

    pub fn is_black(&self) -> bool {
        self.r == 0 && self.g == 0 && self.b == 0
    }

    // New blend mode methods
    pub fn blend_normal(&self, blend: &Color) -> Color {
        if blend.is_black() {
            *self
        } else {
            *blend
        }
    }

    pub fn blend_multiply(&self, blend: &Color) -> Color {
        Color::new(
            ((self.r as f32 * blend.r as f32) / 255.0) as u8,
            ((self.g as f32 * blend.g as f32) / 255.0) as u8,
            ((self.b as f32 * blend.b as f32) / 255.0) as u8,
        )
    }

    pub fn blend_add(&self, blend: &Color) -> Color {
        Color::new(
            (self.r as u16 + blend.r as u16).min(255) as u8,
            (self.g as u16 + blend.g as u16).min(255) as u8,
            (self.b as u16 + blend.b as u16).min(255) as u8,
        )
    }
    pub fn blend_screen(&self, blend: &Color) -> Color {
        Color::new(
            255 - ((255 - self.r as u16) * (255 - blend.r as u16) / 255) as u8,
            255 - ((255 - self.g as u16) * (255 - blend.g as u16) / 255) as u8,
            255 - ((255 - self.b as u16) * (255 - blend.b as u16) / 255) as u8
        )
    }

    pub fn blend_subtract(&self, blend: &Color) -> Color {
        if blend.is_black() {
            *self
        } else {
            Color::new(
                self.r.saturating_sub(blend.r),
                self.g.saturating_sub(blend.g),
                self.b.saturating_sub(blend.b),
            )
        }
    }

    pub const fn black() -> Self {
        Color { r: 0, g: 0, b: 0 }
    }

    pub const fn skybox_color_j() -> Self {
        Color {
            r: 141,
            g: 162,
            b: 255,
        } // Color for J key
    }

    pub const fn skybox_color_k() -> Self {
        Color {
            r: 213,
            g: 135,
            b: 57,
        } // Color for K key
    }

    pub const fn skybox_color_l() -> Self {
        Color { r: 0, g: 0, b: 0 } // Color for L key
    }

    // Convert color to a hexadecimal representation
    pub fn to_hex(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    // Blend this color with another color
    pub fn blend(&self, other: &Color) -> Color {
        let r = (self.r as f32 + other.r as f32).min(255.0) as u8;
        let g = (self.g as f32 + other.g as f32).min(255.0) as u8;
        let b = (self.b as f32 + other.b as f32).min(255.0) as u8;
        Color::new(r, g, b)
    }

    // Adjust color brightness
    pub fn adjust_brightness(&self, factor: f32) -> Color {
        let r = (self.r as f32 * factor).min(255.0).max(0.0) as u8;
        let g = (self.g as f32 * factor).min(255.0).max(0.0) as u8;
        let b = (self.b as f32 * factor).min(255.0).max(0.0) as u8;
        Color::new(r, g, b)
    }

    pub fn interpolate(v1: &Color, v2: &Color, v3: &Color, w1: f32, w2: f32, w3: f32) -> Color {
        let r = (v1.r as f32 * w1 + v2.r as f32 * w2 + v3.r as f32 * w3)
            .round()
            .clamp(0.0, 255.0) as u8;
        let g = (v1.g as f32 * w1 + v2.g as f32 * w2 + v3.g as f32 * w3)
            .round()
            .clamp(0.0, 255.0) as u8;
        let b = (v1.b as f32 * w1 + v2.b as f32 * w2 + v3.b as f32 * w3)
            .round()
            .clamp(0.0, 255.0) as u8;
        Color::new(r, g, b)
    }


    pub fn blend_overlay(&self, blend: &Color) -> Color {
        // Overlay: Combines Multiply and Screen blend modes
        fn overlay_channel(base: u8, blend: u8) -> u8 {
            if base < 128 {
                ((2 * base as u16 * blend as u16) / 255) as u8
            } else {
                (255 - 2 * (255 - base as u16) * (255 - blend as u16) / 255) as u8
            }
        }
        Color::new(
            overlay_channel(self.r, blend.r),
            overlay_channel(self.g, blend.g),
            overlay_channel(self.b, blend.b)
        )
    }

    pub fn blend_darken(&self, blend: &Color) -> Color {
        Color::new(
            self.r.min(blend.r),
            self.g.min(blend.g),
            self.b.min(blend.b)
        )
    }

    pub fn blend_lighten(&self, blend: &Color) -> Color {
        Color::new(
            self.r.max(blend.r),
            self.g.max(blend.g),
            self.b.max(blend.b)
        )
    }

    pub fn blend_color_dodge(&self, blend: &Color) -> Color {
        fn dodge_channel(base: u8, blend: u8) -> u8 {
            if blend == 255 {
                255
            } else {
                ((base as u16 * 255) / (255 - blend as u16)).min(255) as u8
            }
        }
        Color::new(
            dodge_channel(self.r, blend.r),
            dodge_channel(self.g, blend.g),
            dodge_channel(self.b, blend.b)
        )
    }

    pub fn blend_color_burn(&self, blend: &Color) -> Color {
        fn burn_channel(base: u8, blend: u8) -> u8 {
            if blend == 0 {
                0
            } else {
                255 - ((255 - base as u16) * 255 / blend as u16).min(255) as u8
            }
        }
        Color::new(
            burn_channel(self.r, blend.r),
            burn_channel(self.g, blend.g),
            burn_channel(self.b, blend.b)
        )
    }

    pub fn blend_hard_light(&self, blend: &Color) -> Color {
        // Hard Light: Similar to Overlay, but with blend and base colors swapped
        self.blend_overlay(blend)
    }

    pub fn blend_soft_light(&self, blend: &Color) -> Color {
        fn soft_light_channel(base: u8, blend: u8) -> u8 {
            let b = base as f32 / 255.0;
            let s = blend as f32 / 255.0;
            if s < 0.5 {
                (b - (1.0 - 2.0 * s) * b * (1.0 - b)) * 255.0
            } else {
                (b + (2.0 * s - 1.0) * (((b - 0.5).abs() * 16.0 + 12.0) * b - 3.0)) * 255.0
            }.round() as u8
        }
        Color::new(
            soft_light_channel(self.r, blend.r),
            soft_light_channel(self.g, blend.g),
            soft_light_channel(self.b, blend.b)
        )
    }

    pub fn blend_difference(&self, blend: &Color) -> Color {
        Color::new(
            (self.r as i16 - blend.r as i16).abs() as u8,
            (self.g as i16 - blend.g as i16).abs() as u8,
            (self.b as i16 - blend.b as i16).abs() as u8
        )
    }

    pub fn blend_exclusion(&self, blend: &Color) -> Color {
        Color::new(
            (self.r as u16 + blend.r as u16 - 2 * self.r as u16 * blend.r as u16 / 255) as u8,
            (self.g as u16 + blend.g as u16 - 2 * self.g as u16 * blend.g as u16 / 255) as u8,
            (self.b as u16 + blend.b as u16 - 2 * self.b as u16 * blend.b as u16 / 255) as u8
        )
    }

 

}
