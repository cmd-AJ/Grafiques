use nalgebra_glm::Vec3;
use crate::framebuffer::Framebuffer;

pub trait Vecline {
    fn line(&mut self, start: Vec3, end: Vec3);
}

impl Vecline for Framebuffer {
    fn line(&mut self, start: Vec3, end: Vec3) {
        let x1 = start.x as usize;
        let y1 = start.y as usize;
        let x2 = end.x as usize;
        let y2 = end.y as usize;

        let dx = if x2 > x1 { x2 - x1 } else { x1 - x2 };
        let dy = if y2 > y1 { y2 - y1 } else { y1 - y2 };
        let sx = if x1 < x2 { 1 } else { usize::MAX }; // usize::MAX as a substitute for -1
        let sy = if y1 < y2 { 1 } else { usize::MAX }; // usize::MAX as a substitute for -1
        let mut err = if dx > dy { (dx as isize) / 2 } else { -(dy as isize) / 2 };

        let mut x = x1;
        let mut y = y1;

        loop {
            self.point(x as isize, y as isize);

            if x == x2 && y == y2 {
                break;
            }

            let e2 = err;

            if e2 > -(dx as isize) {
                err -= dy as isize;
                if sx == usize::MAX {
                    x = x.wrapping_sub(1);
                } else {
                    x += sx;
                }
            }

            if e2 < dy as isize {
                err += dx as isize;
                if sy == usize::MAX {
                    y = y.wrapping_sub(1);
                } else {
                    y += sy;
                }
            }
        }
    }
}
