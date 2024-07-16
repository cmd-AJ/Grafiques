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


    let verticespol4 = vec![
        Vec3::new(413.0, 177.0, 0.0),
        Vec3::new(448.0, 159.0, 0.0),
        Vec3::new(502.0, 88.0, 0.0),
        Vec3::new(553.0, 53.0, 0.0),
        Vec3::new(535.0, 36.0, 0.0),
        Vec3::new(676.0, 37.0, 0.0),
        Vec3::new(660.0, 52.0, 0.0),
        Vec3::new(750.0, 145.0, 0.0),
        Vec3::new(761.0, 179.0, 0.0),
        Vec3::new(672.0, 192.0, 0.0),
        Vec3::new(659.0, 214.0, 0.0),
        Vec3::new(615.0, 214.0, 0.0),
        Vec3::new(632.0, 230.0, 0.0),
        Vec3::new(580.0, 230.0, 0.0),
        Vec3::new(597.0, 215.0, 0.0),
        Vec3::new(552.0, 214.0, 0.0),
        Vec3::new(517.0, 144.0, 0.0),
        Vec3::new(466.0, 180.0, 0.0),
    ];

    fb.set_foreground_color(0x00FF00);
    fb.fill_polygon(&verticespol4);
    fb.set_foreground_color(0xFFFFFF);
    fb.draw_polygon(&verticespol4);


    let poli5 = vec![
        Vec3::new(682.0, 175.0, 0.0),
        Vec3::new(708.0, 120.0, 0.0),
        Vec3::new(735.0, 148.0, 0.0),
        Vec3::new(739.0, 170.0, 0.0),
    ];

    fb.set_foreground_color(0xccc2ff);
    fb.fill_polygon(&poli5);
    fb.set_foreground_color(0xFFFFFF);
    fb.draw_polygon(&poli5);

    fb.render_buffer("filled_polygon.bmp").unwrap();
}
