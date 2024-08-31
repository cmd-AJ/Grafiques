use castingray::cast_ray;
mod sphere;
use colors::Color;
use sphere::Sphere;
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use nalgebra_glm::{normalize, Vec3};
mod framebuffer;
use framebuffer::Framebuffer;
mod castingray;
mod rayintersect;
mod colors;
use rayintersect::{Intersect,RayIntersect,Material};

pub fn render(framebuffer: &mut Framebuffer, objects: &[Sphere]) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            // Map the pixel coordinate to screen space [-1, 1]
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            // Adjust for aspect ratio
            let adjusted_x = screen_x * aspect_ratio;

            // Calculate the direction of the ray for this pixel
            let ray_direction = normalize(&Vec3::new(adjusted_x, screen_y, -1.0));

            // Cast the ray and get the pixel color
            let origin = Vec3::new(0.0, 0.0, 0.0); // Camera position
            let pixel_color = cast_ray(&origin, &ray_direction, objects);

            // Draw the pixel on screen with the returned color
            framebuffer.set_foreground_color(pixel_color.to_hex());
            framebuffer.point(x, y);
        }
    }
}

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 800;
    let framebuffer_height = 600;
    let frame_delay = Duration::from_millis(16);
    let fps = 40;

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "KOALONSON ONSON",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    let marmle = Material{
        diffuse: Color::new(250, 250, 250),
    };

    let ivory = Material{
        diffuse: Color::new(2, 0, 5),
    };
    let mout = Material{
        diffuse: Color::new(60, 60, 60),
    };

    let giz = Material{
        diffuse: Color::new(152, 147, 140)
    };
    let griz = Material{
        diffuse: Color::new(120, 120, 120)
    };


    let objects = [
        Sphere {
            center: Vec3::new(0.0, 0.0, -5.0), // Move the sphere away from the camera
            radius: 3.3,
            material: giz
        },
        Sphere {
            center: Vec3::new(0.0, -0.6, -1.5), // Move the sphere away from the camera
            radius: 0.4,
            material: ivory
        },
        Sphere {
            center: Vec3::new(0.0, -0.73, -1.6), // Move the sphere away from the camera
            radius: 0.4,
            material: mout
        },
        Sphere {
            center: Vec3::new(-0.30, 0.24, -1.1), // Move the sphere away from the camera
            radius: 0.08,
            material: ivory
        },
        Sphere {
            center: Vec3::new(0.22, 0.20, -1.0), // Move the sphere away from the camera
            radius: 0.02,
            material: marmle
        },
        Sphere {
            center: Vec3::new(-0.26, 0.20, -1.0), // Move the sphere away from the camera
            radius: 0.02,
            material: marmle
        },
        Sphere {
            center: Vec3::new(0.26, 0.24, -1.1), // Move the sphere away from the camera
            radius: 0.08,
            material: ivory
        },
        Sphere {
            center: Vec3::new(-3.0, 2.0, -5.1), // Move the sphere away from the camera
            radius: 1.8,
            material: giz
        },
        Sphere {
            center: Vec3::new(-2.8, 1.8, -4.6), // Move the sphere away from the camera
            radius: 1.4,
            material: griz
        },
        Sphere {
            center: Vec3::new(3.0, 2.0, -5.1), // Move the sphere away from the camera
            radius: 1.8,
            material: giz
        },
        Sphere {
            center: Vec3::new(2.8, 1.8, -4.6), // Move the sphere away from the camera
            radius: 1.4,
            material: griz
        },



    ];
   
    while window.is_open() && !window.is_key_down(Key::Escape) {
        framebuffer.clear();
        render(&mut framebuffer, &objects);
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();
        std::thread::sleep(frame_delay);
    }
}
