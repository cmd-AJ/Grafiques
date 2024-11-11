use colors::Color;
use fastnoise_lite::{FastNoiseLite, NoiseType};
use fragment::Fragment;
use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{dot, Mat4, Vec3};
use std::{f32::consts::PI, time::Duration};
use vertex::Vertex;
mod framebuffer;
use framebuffer::Framebuffer;
mod colors;
mod fragment;
mod uniform;
mod vertex;
use uniform::{
    create_model_matrix, create_noise, create_perspective_matrix, create_view_matrix,
    create_viewport_matrix, render, Uniforms,
};
mod obj;
mod vertexshader;
use obj::Obj;
mod camera;
use camera::Camera;

fn calculate_bounding_box(v1: &Vec3, v2: &Vec3, v3: &Vec3) -> (i32, i32, i32, i32) {
    let min_x = v1.x.min(v2.x).min(v3.x).floor() as i32;
    let min_y = v1.y.min(v2.y).min(v3.y).floor() as i32;
    let max_x = v1.x.max(v2.x).max(v3.x).ceil() as i32;
    let max_y = v1.y.max(v2.y).max(v3.y).ceil() as i32;
    (min_x, min_y, max_x, max_y)
}

fn barycentric_coordinates(
    point: &Vec3,
    v1: &Vec3,
    v2: &Vec3,
    v3: &Vec3,
    area: f32,
) -> (f32, f32, f32) {
    let w1 = edge_function(v2, v3, point) / area;
    let w2 = edge_function(v3, v1, point) / area;
    let w3 = edge_function(v1, v2, point) / area;
    (w1, w2, w3)
}

pub fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();
    let (a, b, c) = (
        v1.transformed_position,
        v2.transformed_position,
        v3.transformed_position,
    );

    let (min_x, min_y, max_x, max_y) = calculate_bounding_box(&a, &b, &c);

    let light_dir = Vec3::new(-0.5, 0.9, 0.0);

    let triangle_area = edge_function(&a, &b, &c);

    // Iterate over each pixel in the bounding box
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let point = Vec3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0);

            // Calculate barycentric coordinates
            let (w1, w2, w3) = barycentric_coordinates(&point, &a, &b, &c, triangle_area);

            // Check if the point is inside the triangle
            if w1 >= 0.0 && w1 <= 1.0 && w2 >= 0.0 && w2 <= 1.0 && w3 >= 0.0 && w3 <= 1.0 {
                // Interpolate normal
                // let normal = v1.transformed_normal * w1 + v2.transformed_normal * w2 + v3.transformed_normal * w3;
                let normal = v1.transformed_normal;
                let normal = normal.normalize();

                // Calculate lighting intensity
                let intensity = dot(&normal, &light_dir).max(0.0);

                // Create a gray color and apply lighting
                let base_color = Color::new(100, 100, 100); // Medium gray
                let lit_color = base_color.adjust_brightness(intensity);

                // Interpolate depth
                let depth = a.z * w1 + b.z * w2 + c.z * w3;

                let vertex_position = v1.position * w1 + v2.position * w2 + v3.position * w3;

                fragments.push(Fragment::new(
                    x as f32,
                    y as f32,
                    lit_color,
                    depth,
                    normal,
                    intensity,
                    vertex_position,
                ));
            }
        }
    }

    fragments
}

fn edge_function(a: &Vec3, b: &Vec3, c: &Vec3) -> f32 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

fn main() {
    let window_width = 800;
    let window_height = 600;
    let mut framebuffer = Framebuffer::new(window_width, window_height);
    let start_time = std::time::Instant::now();

    let mut window = Window::new(
        "Mother Zeta Fallut 3",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    let mut obj = Obj::load("./assets/sphere-1.obj").expect("Failed to load .obj file");
    let mut vertex_array = obj.get_vertex_array();

    let mut translation = Vec3::new(0.0, 0.0, 0.0);
    let scale_factor = 1.0f32;
    let mut rotation_angles = Vec3::new(0.0, 0.0, 0.0);

    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 5.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        false,
    );

    //CAMARA
    let mut eye = Vec3::new(0.0, 0.0, 5.0);
    let mut center = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);

    let mut blend_type = "2";

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let elapsed = start_time.elapsed();
        let time = elapsed.as_secs_f32() as u32; //

        let movement_speed = 1.0;
        let rotation_speed = PI / 50.0;
        let zoom_speed = 0.1;

        //  camera orbit controls
        if window.is_key_down(Key::Left) {
            camera.orbit(rotation_speed, 0.0);
        }
        if window.is_key_down(Key::Right) {
            camera.orbit(-rotation_speed, 0.0);
        }
        if window.is_key_down(Key::W) {
            camera.orbit(0.0, -rotation_speed);
        }
        if window.is_key_down(Key::S) {
            camera.orbit(0.0, rotation_speed);
        }

        // Camera movement controls
        let mut movement = Vec3::new(0.0, 0.0, 0.0);
        if window.is_key_down(Key::A) {
            movement.x -= movement_speed;
        }
        if window.is_key_down(Key::D) {
            movement.x += movement_speed;
        }
        if window.is_key_down(Key::Q) {
            movement.y += movement_speed;
        }
        if window.is_key_down(Key::E) {
            movement.y -= movement_speed;
        }
        if movement.magnitude() > 0.0 {
            camera.move_center(movement);
        }

        // Camera zoom controls
        if window.is_key_down(Key::Up) {
            camera.zoom(zoom_speed);
        }
        if window.is_key_down(Key::Down) {
            camera.zoom(-zoom_speed);
        }

        //SATURN
        if window.is_key_down(Key::NumPad2) {
            obj = Obj::load("./assets/sphere-1.obj").expect("Failed to load .obj file");
            vertex_array = obj.get_vertex_array();
            blend_type = "2";
        }

        //SOL
        if window.is_key_down(Key::NumPad1) {
            obj = Obj::load("./assets/SATURN.obj").expect("Failed to load .obj file");
            vertex_array = obj.get_vertex_array();
            blend_type = "1";
        }

        //neptuno
        if window.is_key_down(Key::NumPad3) {
            obj = Obj::load("./assets/sphere-1.obj").expect("Failed to load .obj file");
            vertex_array = obj.get_vertex_array();
            blend_type = "3";
        }

        if window.is_key_down(Key::NumPad4) {
            obj = Obj::load("./assets/rockos.obj").expect("Failed to load .obj file");
            vertex_array = obj.get_vertex_array();
            blend_type = "4";
        }

        if window.is_key_down(Key::NumPad5) {
            obj = Obj::load("./assets/sphere-1.obj").expect("Failed to load .obj file");
            vertex_array = obj.get_vertex_array();
            blend_type = "5";
        }


        if window.is_key_down(Key::NumPad6) {
            obj = Obj::load("./assets/sphere-1.obj").expect("Failed to load .obj file");
            vertex_array = obj.get_vertex_array();
            blend_type = "6";
        }


        if window.is_key_down(Key::NumPad7) {
            obj = Obj::load("./assets/sphere-1.obj").expect("Failed to load .obj file");
            vertex_array = obj.get_vertex_array();
            blend_type = "7";
        }


        framebuffer.clear(); // Clear the framebuffer for each frame

        let model_matrix = create_model_matrix(translation, scale_factor, rotation_angles);
        let view_matrix = create_view_matrix(camera.eye, camera.center, camera.up); // Adjust for camera if needed
        let projection_matrix =
            create_perspective_matrix(window_width as f32, window_height as f32); // Set up perspective projection
        let viewport_matrix = create_viewport_matrix(window_width as f32, window_height as f32);
        let mut noise = create_noise(&blend_type);

        let uniforms = Uniforms {
            model_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix,
            time,
            noise: noise,
        };

        render(&mut framebuffer, &uniforms, &vertex_array, &blend_type); // Render using the framebuffer

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer.width, framebuffer.height)
            .unwrap();
        std::thread::sleep(Duration::from_millis(5)); // Control frame rate
    }
}
