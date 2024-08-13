use colors::Color;
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use nalgebra_glm::Vec3;

mod raytracing;
mod framebuffer;
mod sphere;
mod ray_intersect;
mod colors;
use framebuffer::Framebuffer;
use sphere::Sphere;
use ray_intersect::{Material, RayIntersect};
use raytracing::render;
mod castingray;

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 200;
    let framebuffer_height = 100;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "3D Objects",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    let red_material = Material {
        diffuse: Color::new(255, 0, 0),
    };
    let green_material = Material {
        diffuse: Color::new(0, 255, 0),
    };
    let blue_material = Material {
        diffuse: Color::new(0, 0, 255),
    };

    let spheres: Vec<Box<dyn RayIntersect>> = vec![
        Box::new(Sphere {
            center: Vec3::new(0.0, 0.0, -5.0),
            radius: 1.0,
            material: red_material,
        }),
        Box::new(Sphere {
            center: Vec3::new(2.0, 1.0, -6.0),
            radius: 0.5,
            material: green_material,
        }),
        Box::new(Sphere {
            center: Vec3::new(-2.0, -1.0, -7.0),
            radius: 0.75,
            material: blue_material,
        }),
    ];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        framebuffer.clear();
        render(&mut framebuffer, &spheres);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        if window.is_key_down(Key::Enter) {
            break;
        }

        std::thread::sleep(frame_delay);
    }
}
