// fragment.rs

use nalgebra_glm::{Vec2, Vec3};
use crate::{colors::Color, uniform::{self, Uniforms}};

pub struct Fragment {
    pub position: Vec2,
    pub color: Color,
    pub depth: f32,
    pub normal: Vec3,
    pub intensity: f32,
}

impl Fragment {
    pub fn new(x: f32, y: f32, color: Color, depth: f32, normal: Vec3, intensity:f32) -> Self {
        Fragment {
            position: Vec2::new(x, y),
            color,
            depth,
            normal,
            intensity,

        }
    }

}

pub fn fragment_shader(fragment: &Fragment, uniform: &Uniforms) -> Color{
    fragment.color.adjust_brightness(fragment.intensity)
}