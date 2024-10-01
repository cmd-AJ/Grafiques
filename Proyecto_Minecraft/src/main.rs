use castingray::cast_ray;

use colors::Color;

use minifb::{Key, Window, WindowOptions};
use std::{f32::consts::PI, sync::Arc ,time::{Duration, Instant}};
use nalgebra_glm::Vec3;
mod framebuffer;
use framebuffer::Framebuffer;
mod castingray;
mod rayintersect;
mod colors;
mod material;
use material::Material;
mod camera;
use camera::Camera;
mod light;
use light::Light;
mod shadow;
mod r_stations;
mod texture;
use texture::Texture;
use once_cell::sync::Lazy;
mod cube;
use cube::Cube;


static COBBLESTONE: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/acacia.png")));
static ACACIALOG: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/log_acacia.png")));





pub fn render(framebuffer: &mut Framebuffer, objects: &[Cube], camera: &Camera, light: &Light) {
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
            let pixel_color = cast_ray(&camera.eye, &rotated_direction, objects, &light, 0);

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
    let fps = 0;

    

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        &format!("KOALONSON ONSON - FPS: {}", fps), 
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();



    //transparency es (3)
    //reflection (2)
    //albedo (1)
    //refraction 4
    let marmle = Material::new(
        Color::new(250, 250, 250),
        50.0,
        [0.3, 0.1, 0.6, 0.0],
        0.0,
        
    );

    let ivorys = Material::new_with_text(
        0.0,
        [1.0,0.0, 0.0, 0.0],
        0.0,
        COBBLESTONE.clone()

    );
    // rubber
    let mout = Material::new(
        Color::new(60, 60, 60),
        10.0,
        [0.9,0.1, 0.6, 0.0],
        0.0,
    );

    let giz = Material::new(
        Color::new(255, 255, 255),
        1425.0,
        [0.0, 10.0, 0.5, 0.5],
        0.3,
    );

    let griz = Material::new(
       Color::new(120, 120, 120),
       1.0,
       [0.9,0.0, 0.00, 0.0],
       0.0,
    );


    let objects = [
        Cube {
            center: Vec3::new(0.0, 0.0, -2.0),
            size: 1.0,
            material: marmle
        },
        Cube {
            center: Vec3::new(1.0, 1.0, -2.0),
            size: 1.0,
            material: ivorys
        }


    ];

    let light = Light::new(
        Vec3::new(-5.0, 0.0, 5.0)
        , Color::new(255, 255, 255)
        , 1.0
    );

    let mut  camera = Camera::new(
        Vec3::new(0.1, 0.1, 5.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        true
    );

        let mut last_time = Instant::now();
    let mut frame_count = 0;
    let rotation_speed = PI/50.0;
    let zoom_speed = 0.1;

   
    while window.is_open() && !window.is_key_down(Key::Escape) {

        if window.is_key_down(Key::Left) {
            camera.orbit(rotation_speed, 0.0);
        }

        if window.is_key_down(Key::Right) {
            camera.orbit(-rotation_speed, 0.0);
        }
;
        if window.is_key_down(Key::Up) {
            camera.orbit(0.0,-rotation_speed);
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

        if camera.check_change() {
            framebuffer.clear();
            render(&mut framebuffer, &objects, &mut camera, &light);   
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
              let fps = frame_count;   // The number of frames counted over one second
  
              // Reset frame count and last_time for the next second
              frame_count = 0;
              last_time = now;
  
              // Update window title with FPS
              let title = format!("KOALONSON ONSON - FPS: {}", fps);
              window.set_title(&title);
          }
    }
}
