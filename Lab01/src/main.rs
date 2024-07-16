mod framebuffer;
mod line;
mod bmp;

use nalgebra_glm::Vec3;
use crate::framebuffer::Framebuffer;

fn main() {
    let mut fb = Framebuffer::new(500, 500);
    fb.clear();

    fb.set_background_color(0xFFFFFF);
    fb.set_foreground_color(0x000000);

    let vertices = vec![
        Vec3::new(165.0, 380.0, 0.0),
        Vec3::new(185.0, 360.0, 0.0),
        Vec3::new(180.0, 330.0, 0.0),
        Vec3::new(207.0, 345.0, 0.0),
        Vec3::new(233.0, 330.0, 0.0),
        Vec3::new(230.0, 360.0, 0.0),
        Vec3::new(250.0, 380.0, 0.0),
        Vec3::new(220.0, 385.0, 0.0),
        Vec3::new(205.0, 410.0, 0.0),
        Vec3::new(193.0, 383.0, 0.0),
    ];
    
    fb.draw_polygon(&vertices);
    fb.fill_polygon(&vertices);

    fb.render_buffer("filled_polygon.bmp").unwrap();
}
