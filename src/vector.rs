use nalgebra_glm as glm;

// Importing Vec3 from nalgebra_glm
use glm::Vec3 as Vec3glm;

// Define your Vec3u struct
pub struct Vec3u {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Vec3u {
    // Constructor from nalgebra-glm Vec3
    pub fn from_glm_vec3(v: Vec3glm) -> Self {
        Vec3u {
            x: v.x as usize,
            y: v.y as usize,
            z: v.z as usize,
        }
    }
}