use crate::bmp::write_bmp_file;

pub struct Framebuffer {
    pub buffer: Vec<u32>,
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

    pub fn point(&mut self, x: usize, y: usize) {
        if x < self.width as usize && y < self.height as usize {
            let pix_pos = (y * self.width as usize + x) as usize;
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

   
}
