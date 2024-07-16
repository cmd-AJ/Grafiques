use crate::framebuffer::Framebuffer;
use crate::line::Line;

pub trait Polygon {
    fn polygon(&mut self, vertices: &[(usize, usize)]);
}

impl Polygon for Framebuffer {
    fn polygon(&mut self, vertices: &[(usize, usize)]) {
        // Iterate through each pair of consecutive vertices
        let num_vertices = vertices.len();
        for i in 0..num_vertices {
            let (x1, y1) = vertices[i];
            let (x2, y2) = vertices[(i + 1) % num_vertices]; // Wrap around to connect the last vertex with the first

            self.line(x1, y1, x2, y2);
        }
    }
}

