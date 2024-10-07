use castingray::cast_ray;
use colors::Color;
use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::Vec3;
use once_cell::sync::Lazy;
use std::{
    f32::consts::PI,
    sync::Arc,
    time::{Duration, Instant},
};

mod camera;
mod castingray;
mod colors;
mod cube;
mod framebuffer;
mod light;
mod material;
mod myobjects;
mod r_stations;
mod rayintersect;
mod shadow;
mod texture;

use camera::Camera;
use cube::Cube;
use framebuffer::Framebuffer;
use light::Light;
use material::Material;
use myobjects::loadobjects;
use texture::Texture;

static WATER: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/water.png")));

pub fn render(framebuffer: &mut Framebuffer, objects: &[Cube], camera: &Camera, lights: &[Light], skybox_color: Color) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;
    let fov = PI / 3.0; // Adjust if needed
    let perspective_scale = (fov / 2.0).tan();

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
            let pixel_color = cast_ray(&camera.eye, &rotated_direction, objects, lights, 0, skybox_color);

            // Draw the pixel on screen with the returned color
            framebuffer.set_foreground_color(pixel_color.to_hex());
            framebuffer.point(x, y);
        }
    }
}

fn main() {
    let window_width = 600;
    let window_height = 400;
    let framebuffer_width = 600;
    let framebuffer_height = 400;
    let frame_delay = Duration::from_millis(16);
    let fps = 0;

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        &format!("KOALONSON ONSON - FPS: {}", fps),
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    // Initialize water material
    let mut water = Material::new_with_text(0.0, [1.0, 0.0, 0.1, 0.0], 0.0, WATER.clone());

    // Set initial albedo property
    water.albedo[2] = 0.5; // Initial value

    let mut objects = loadobjects(water);

    let mut lights = vec![
        Light::new(Vec3::new(-5.0, 5.0, 5.0), Color::new(255, 255, 255), 1.0),
        Light::new(Vec3::new(2.0, 0.8, -1.4), Color::new(144, 16, 235), 0.18),
    ];

    let mut camera = Camera::new(
        Vec3::new(0.1, 0.1, 5.0),
        Vec3::new(1.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        true,
    );

    let mut last_time = Instant::now();
    let mut frame_count = 0;
    let rotation_speed = PI / 50.0;
    let zoom_speed = 0.1;

    // Timer to keep track of albedo changes
    let mut albedo_timer = Instant::now();
    let mut albedo_state = 0; // To toggle between albedo states
    let mut current_skybox_color = Color::skybox_color_j(); // Set initial skybox color

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::Left) {
            camera.orbit(rotation_speed, 0.0);
        }

        if window.is_key_down(Key::Right) {
            camera.orbit(-rotation_speed, 0.0);
        };
        if window.is_key_down(Key::Up) {
            camera.orbit(0.0, -rotation_speed);
        }

        if window.is_key_down(Key::Down) {
            camera.orbit(0.0, rotation_speed);
        }

        if window.is_key_down(Key::NumPadMinus) {
            camera.zoom(-zoom_speed);
        }

        if window.is_key_down(Key::NumPadPlus) {
            camera.zoom(zoom_speed);
        }

        if window.is_key_down(Key::J) {
            current_skybox_color = Color::skybox_color_j();
            lights[0] = Light::new(Vec3::new(-5.0, 5.0, 5.0), Color::new(255, 255, 255), 1.0);
        }
        if window.is_key_down(Key::K) {
            current_skybox_color = Color::skybox_color_k();
            lights[0] = Light::new(Vec3::new(-5.0, 5.0, 5.0), Color::new(255, 255, 255), 0.6);
        }
        if window.is_key_down(Key::L) {
            current_skybox_color = Color::skybox_color_l();
            lights[0] = Light::new(Vec3::new(-5.0, 5.0, 5.0), Color::new(255, 255, 255), 0.3);
        }

        

        // Check if 10 seconds or 5 seconds have passed to change albedo
        if albedo_timer.elapsed().as_secs() >= 5 {
            albedo_timer = Instant::now(); // Reset timer

            // Change albedo based on the current state
            match albedo_state {
                0 => {
                    objects[104].material.albedo[2] = 0.3; // Set to 0.5 at index 104
                    objects[105].material.albedo[2] = 0.3; // Set to 0.5 at index 105
                    albedo_state = 1; // Move to the next state
                }
                1 => {
                    objects[104].material.albedo[2] = 0.5; // Set to 0.5 at index 104
                    objects[105].material.albedo[2] = 0.5;
                    albedo_state = 0; // Reset to the initial state
                }
                _ => {}
            }
        }

        if camera.check_change() {
            framebuffer.clear();
            render(&mut framebuffer, &objects, &mut camera, &lights,  current_skybox_color);
        }

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();
        std::thread::sleep(frame_delay);

        // Calculate FPS every second
        frame_count += 1;
        let now = Instant::now();

        // Check if 1 second has passed
        if now.duration_since(last_time).as_secs() >= 1 {
            let fps = frame_count; // The number of frames counted over one second

            // Reset frame count and last_time for the next second
            frame_count = 0;
            last_time = now;

            // Update window title with FPS
            let title = format!("KOALONSON ONSON - FPS: {}", fps);
            window.set_title(&title);
        }
    }
}
