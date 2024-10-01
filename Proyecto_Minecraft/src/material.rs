
use std::sync::Arc;

use once_cell::sync::Lazy;

use crate::{texture::{self, Texture}, Color};


#[derive(Debug, Clone)]
pub struct Material {
    pub diffuse: Color,
    pub specular: f32,
    pub albedo: [f32;4],
    pub refraction_index: f32, 
    pub texture: Option<Arc<Texture>>
}

impl Material{

    pub fn new(
        diffuse: Color,
        specular: f32,
        albedo: [f32;4],
        refraction_index: f32, 
    ) -> Self {
        Material {
            diffuse,
            specular,
            albedo,
            refraction_index,
            texture : None 
        }
    }

    pub fn new_with_text(
        specular: f32,
        albedo: [f32;4],
        refraction_index: f32, 
        texture: Arc<Texture>,
    ) -> Self {
        Material {
            diffuse: Color::black(),
            specular,
            albedo,
            refraction_index,
            texture: Some(texture)
        }
    }

    pub fn get_diffuse(&self, u: f32, v: f32) -> Color {
        if let Some(texture) = &self.texture {
            // Clamp the UV coordinates to the range [0, 1]

            let u_clamped = u.clamp(0.0, 1.0);
            let v_clamped = v.clamp(0.0, 1.0);         
        
            // Calculate pixel indices using floor instead of round
            let x = (u_clamped * (texture.width as f32 - 1.0)).floor() as usize; 
            let y = (v_clamped * (texture.height as f32 - 1.0)).floor() as usize;

    
            // Ensure indices are within bounds
            if x < texture.width as usize && y < texture.height as usize {
                texture.get_pixel_color(u , v )
            } else {
                // Handle out-of-bounds access gracefully, e.g., return a default color
                Color::new(0, 255, 0) // or any default color you prefer
            }
        } else {
            self.diffuse
        }
    }

    pub fn black() -> Self{
        Material {
            diffuse:Color::black(),
            specular: 0.0,
            albedo: [0.0,0.0,0.0,0.0],
            refraction_index:0.0,
            texture: None


        }
    }

}
