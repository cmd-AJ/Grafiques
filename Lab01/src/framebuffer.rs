use crate::{bmp::write_bmp_file, line::Vecline};
use nalgebra_glm as glm;
use glm::Vec3 as Vec3glm;

pub struct Framebuffer {
    buffer: Vec<u32>,
    pub width: usize,
    pub height: usize,
    background_color: u32,
    foreground_color: u32,    
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let background_color = 0x00FF00;
        let foreground_color = 0xFF00FF;
        let buffer = vec![background_color; width * height];

        Self {
            buffer,
            width,
            height,
            background_color,
            foreground_color,
        }
    }

    pub fn clear(&mut self) {
        self.buffer = vec![self.background_color; self.width * self.height];
    }

    pub fn point(&mut self, x: isize, y: isize) {
        if x >= 0 && y >= 0 && x < self.width as isize && y < self.height as isize {
            let pix_pos = (y * self.width as isize + x) as usize;
            self.buffer[pix_pos] = self.foreground_color;
        }
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
        self.clear()
    }

    pub fn set_foreground_color(&mut self, color: u32) {
        self.foreground_color = color;
    }

    pub fn render_buffer(&self, file_path: &str) -> std::io::Result<()> {
        write_bmp_file(file_path, &self.buffer, self.width, self.height)?;
        Ok(())
    }

    pub fn draw_polygon(&mut self, vertices: &[Vec3glm]) {
        if vertices.len() < 2 {
            return;
        }

        for i in 0..vertices.len() {
            let start = &vertices[i];
            let end = if i == vertices.len() - 1 {
                &vertices[0]
            } else {
                &vertices[i + 1]
            };
            self.line(start, end);
        }
    }

    pub fn fill_polygon(&mut self, vertices: &[Vec3glm]) {
        let n = vertices.len();
        if n < 3 {
            return;
        }

        let mut min_y = vertices[0].y as usize;
        let mut max_y = vertices[0].y as usize;

        for vertex in vertices.iter() {
            if vertex.y  < min_y as f32 {
                min_y = vertex.y as usize;
            }
            if vertex.y as usize > max_y {
                max_y = vertex.y as usize;
            }
        }

        for y in min_y..=max_y {
            let mut nodes = Vec::new();
            let mut j = n - 1;
            for i in 0..n {
                let vi = &vertices[i];
                let vj = &vertices[j];

                if (vi.y <= y as f32 && vj.y > y as f32) || (vj.y <= y as f32 && vi.y > y as f32) {
                    let x = (vi.x + (y as f32 - vi.y) / (vj.y - vi.y) * (vj.x - vi.x)) as usize;
                    nodes.push(x);
                }
                j = i;
            }

            nodes.sort();

            for k in (0..nodes.len()).step_by(2) {
                if k + 1 >= nodes.len() {
                    break;
                }
                for x in nodes[k]..=nodes[k + 1] {
                    self.point(x as isize, y as isize);
                }
            }
        }
    }
}
