use nalgebra_glm::Vec3;

use crate::{obj::Obj, vertex::Vertex};

struct ObjectInPos {
    scale_factor: f32,
    obj: Obj,  // Assuming Obj is a type that handles loading and getting vertex arrays
    vertex_array: Vec<Vertex>,  // Replace `Vertex` with the actual type used in your `Obj` structure
    blend_type: String,
    position: Vec3, // X, Y, Z Position as Vec3
}

impl ObjectInPos {
    // Constructor that takes parameters to initialize the struct
    fn new(scale_factor: f32, obj_path: &str, blend_type: &str, position: Vec3) -> Self {
        // Load the object and get the vertex array
        let obj = Obj::load(obj_path).expect("Failed to load .obj file");
        let vertex_array = obj.get_vertex_array();

        ObjectInPos {
            scale_factor,
            obj,
            vertex_array,
            blend_type: blend_type.to_string(),
            position,
        }
    }
}