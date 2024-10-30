use nalgebra_glm::{Mat4, Vec4};
use crate::vertex::Vertex;
use crate::framebuffer::Framebuffer;
use crate::line::line;
use crate::fragment::Fragment;
use crate::vertexshader::vertex_shader;

pub struct Uniforms {
    pub model_matrix: Mat4,
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
}

pub fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    // Vertex Shader Stage: Transform all vertices using the vertex shader
    let transformed_vertices: Vec<Vertex> = vertex_array
        .iter()
        .map(|vertex| vertex_shader(vertex, uniforms))
        .collect();

    // Primitive Assembly Stage: Group vertices into triangles
    let triangles: Vec<[Vertex; 3]> = transformed_vertices
        .chunks(3)
        .filter_map(|chunk| {
            if chunk.len() == 3 {
                Some([chunk[0].clone(), chunk[1].clone(), chunk[2].clone()])
            } else {
                None
            }
        })
        .collect();

    // Rasterization Stage: Convert triangles into fragments using the line function
    let mut fragments = Vec::new();
    for tri in triangles {
        fragments.extend(line(&tri[0], &tri[1]));
        fragments.extend(line(&tri[1], &tri[2]));
        fragments.extend(line(&tri[2], &tri[0]));
    }

    // Fragment Processing Stage: Write fragments to framebuffer
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;
        if x < framebuffer.width && y < framebuffer.height {
            framebuffer.buffer[y * framebuffer.width + x] = fragment.color.to_hex();
        }
    }
}