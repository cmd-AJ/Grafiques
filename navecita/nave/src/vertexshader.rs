use nalgebra_glm::{Mat4, Vec4};

use crate::vertex::Vertex;
use crate::{uniform, vertex, Vec3};
use crate::uniform::Uniforms;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let position = Vec4::new(
        vertex.position.x,
        vertex.position.y,
        vertex.position.z,
        1.0,
    );

    // Apply the transformation using the model matrix
    let transformed = uniforms.model_matrix * position;

    let w = transformed.w;
    let transformed_position = Vec3::new(
        transformed.x / w,
        transformed.y / w,
        transformed.z / w,
    );

    // Return the vertex with the transformed position
    Vertex {
        position: transformed_position, // Use the transformed position here
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position,
        transformed_normal: vertex.normal, // If normal transformation is needed, apply it here as well
    }
}
