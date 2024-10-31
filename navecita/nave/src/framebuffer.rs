pub struct Framebuffer {
    pub buffer: Vec<u32>,
    pub z_buffer: Vec<f32>, // Z-buffer for depth
    pub width: usize,
    pub height: usize,
    background_color: u32,
    foreground_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let background_color = 0x191970;
        let foreground_color = 0xFF00FF;
        let buffer = vec![background_color; width * height];
        let z_buffer = vec![f32::INFINITY; width * height]; // Initialize depth buffer with infinity

        Self {
            buffer,
            z_buffer,
            width,
            height,
            background_color,
            foreground_color,
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(self.background_color);
        self.z_buffer.fill(f32::INFINITY); // Reset Z-buffer for each frame
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
        self.clear()
    }

    pub fn set_foreground_color(&mut self, color: u32) {
        self.foreground_color = color;
    }

    pub fn point(&mut self, x: usize, y: usize, color: u32, depth: f32) {
        if x < self.width && y < self.height {
            let pix_pos = y * self.width + x;

            // Update color and Z-buffer if the new depth is closer
            if depth < self.z_buffer[pix_pos] {
                self.z_buffer[pix_pos] = depth;
                self.buffer[pix_pos] = color;
            }
        }
    }
}
