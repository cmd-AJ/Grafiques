mod framebuffer;
mod line;
mod bmp;

use nalgebra_glm::Vec3;
use crate::framebuffer::Framebuffer;

fn main() {
    let mut fb = Framebuffer::new(800, 600);
    fb.clear();

    fb.set_background_color(0xccc2ff);
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
    
    fb.set_foreground_color(0xFFFF00);
    fb.fill_polygon(&vertices);
    fb.set_foreground_color(0xFFFFFF);
    fb.draw_polygon(&vertices);
    

    let verticespol2 = vec![
        Vec3::new(321.0, 335.0, 0.0),
        Vec3::new(288.0, 286.0, 0.0),
        Vec3::new(339.0, 251.0, 0.0),
        Vec3::new(374.0, 302.0, 0.0),
    ];
    
    fb.set_foreground_color(0x0000FF);
    fb.fill_polygon(&verticespol2);
    fb.set_foreground_color(0xFFFFFF);
    fb.draw_polygon(&verticespol2);
    
    let verticespol3 = vec![
        Vec3::new(377.0, 249.0, 0.0),
        Vec3::new(411.0, 197.0, 0.0),
        Vec3::new(436.0, 249.0, 0.0),
    ];

    fb.set_foreground_color(0xFF0000);
    fb.fill_polygon(&verticespol3);
    fb.set_foreground_color(0xFFFFFF);
    fb.draw_polygon(&verticespol3);

    fb.render_buffer("filled_polygon.bmp").unwrap();
}
