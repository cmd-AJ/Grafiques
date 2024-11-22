use nalgebra_glm::Vec3;

use crate::{obj::Obj, vertex::Vertex};
pub struct ObjectInPos {
    pub scale_factor: f32,
    pub obj: Obj,  // Assuming Obj is a type that handles loading and getting vertex arrays
    pub vertex_array: Vec<Vertex>,  // Replace `Vertex` with the actual type used in your `Obj` structure
    pub blend_type: String,
    pub position: Vec3, // X, Y, Z Position as Vec3
    pub rotation: Vec3, 
}

impl ObjectInPos {
    // Constructor that takes parameters to initialize the struct
    pub fn new(scale_factor: f32, obj_path: &str, blend_type: &str, position: Vec3, rotation: Vec3) -> Self {
        // Load the object and get the vertex array
        let obj = Obj::load(obj_path).expect("Failed to load .obj file");
        let vertex_array = obj.get_vertex_array();

        ObjectInPos {
            scale_factor,
            obj,
            vertex_array,
            blend_type: blend_type.to_string(),
            position,
            rotation
        }
    }

    pub fn bounding_sphere(&self) -> (Vec3, f32) {
        (self.position, self.scale_factor) // Position and scale factor can be used as radius
    
    }

    
}