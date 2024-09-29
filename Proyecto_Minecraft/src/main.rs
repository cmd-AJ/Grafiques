use castingray::cast_ray;
mod sphere;
use colors::Color;
use sphere::Sphere;
use minifb::{Key, Window, WindowOptions};
use std::{f32::consts::PI, time::Duration};
use nalgebra_glm::{normalize, Vec3};
mod framebuffer;
use framebuffer::Framebuffer;
mod castingray;
mod rayintersect;
mod colors;
use rayintersect::{Intersect,RayIntersect};
mod material;
use material::Material;
mod camera;
use camera::Camera;
mod light;
use light::Light;
mod reflection;
mod shadow;


pub fn render(framebuffer: &mut Framebuffer, objects: &[Sphere], camera: &Camera, light: &Light) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;
    let fov = PI/3.0;
    let perspective_scale = (fov/2.0).tan();

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            // Map the pixel coordinate to screen space [-1, 1]
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            let screen_x = screen_x * aspect_ratio * perspective_scale;
            let screen_y = screen_y * perspective_scale;

            // Calculate the direction of the ray for this pixel
            let ray_direction = &Vec3::new(screen_x, screen_y, -1.0).normalize();
            let rotated_direction = camera.basis_change(&ray_direction);


            // Cast the ray and get the pixel color
            let pixel_color = cast_ray(&camera.eye, &rotated_direction, objects, &light);

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

    let marmle = Material::new(
        Color::new(250, 250, 250),
        50.0,
        [0.3,0.1],
    );

    let ivory = Material::new(
        Color::new(2, 0, 5),
        50.0,
        [0.3,0.1],
    );
    // rubber
    let mout = Material::new(
        Color::new(60, 60, 60),
        10.0,
        [0.9,0.1]
    );

    let giz = Material::new(
        Color::new(152, 147, 140),
        10.0,
        [0.9,0.1]
    );
    let griz = Material::new(
       Color::new(120, 120, 120),
       10.0,
       [0.9,0.1]
    );


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

    let light = Light::new(
        Vec3::new(0.0, 0.0, 5.0)
        , Color::new(255, 255, 255)
        , 1.0
    );

    let mut  camera = Camera::new(
        Vec3::new(0.1, 0.1, 5.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let rotation_speed = PI/50.0;

   
    while window.is_open() && !window.is_key_down(Key::Escape) {

        if window.is_key_down(Key::Left) {
            camera.orbit(rotation_speed, 0.0);
        }

        if window.is_key_down(Key::Right) {
            camera.orbit(-rotation_speed, 0.0);
        }

        if window.is_key_down(Key::Up) {
            camera.orbit(0.0,-rotation_speed);
        }

        if window.is_key_down(Key::Down) {
            camera.orbit(0.0, rotation_speed);
        }

        
        framebuffer.clear();
        render(&mut framebuffer, &objects, &mut camera, &light);
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();
        std::thread::sleep(frame_delay);
    }
}
