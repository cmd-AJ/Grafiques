use colors::Color;
use fragment::Fragment;
use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{dot, Mat4, Vec3};
use std::time::Duration;
use vertex::Vertex;
mod framebuffer;
use framebuffer::Framebuffer;
mod colors;
mod fragment;
mod line;
mod uniform;
mod vertex;
use uniform::{create_model_matrix, create_perspective_matrix, create_view_matrix, create_viewport_matrix, render, Uniforms};
mod obj;
mod vertexshader;
use obj::Obj;

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

    let light_dir = Vec3::new(0.0, 1.0, 1.0);

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

                fragments.push(Fragment::new(x as f32, y as f32, lit_color, depth));
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

    let mut window = Window::new(
        "Mother Zeta Fallut 3",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    let obj = Obj::load("./assets/sphere.obj").expect("Failed to load .obj file");
    let vertex_array = obj.get_vertex_array();

    let mut translation = Vec3::new(0.0, 0.0, 0.0);
    let scale_factor =  1.0f32;
    let mut rotation_angles = Vec3::new(0.0, 0.0, 0.0);


    //CAMARA
    let mut eye = Vec3::new(0.0, 0.0, 5.0);
    let mut center = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);

    while window.is_open() && !window.is_key_down(Key::Escape) {

        handle_input(&window, &mut eye, &mut center);

        framebuffer.clear(); // Clear the framebuffer for each frame

        let model_matrix = create_model_matrix(translation, scale_factor, rotation_angles);
        let view_matrix = create_view_matrix(eye, center, up); // Adjust for camera if needed
        let projection_matrix =  create_perspective_matrix(window_width as f32, window_height as f32); // Set up perspective projection
        let viewport_matrix = create_viewport_matrix(window_width as f32, window_height as f32);

        let uniforms = Uniforms {
            model_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix
        };

        

        render(&mut framebuffer, &uniforms, &vertex_array); // Render using the framebuffer

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer.width, framebuffer.height)
            .unwrap();
        std::thread::sleep(Duration::from_millis(5)); // Control frame rate
    }
}

fn handle_input(window: &Window, eye: &mut Vec3, center: &mut Vec3) {
    let move_speed = 1.0;

    // Move the center (WASD keys)
    if window.is_key_down(Key::W) {
        center.y -= move_speed; // Move center up
    }
    if window.is_key_down(Key::S) {
        center.y += move_speed; // Move center down
    }
    if window.is_key_down(Key::A) {
        center.x -= move_speed; // Move center left
    }
    if window.is_key_down(Key::D) {
        center.x += move_speed; // Move center right
    }

    // Move the eye (arrow keys)
    if window.is_key_down(Key::Up) {
        eye.y -= move_speed; // Move eye up
    }
    if window.is_key_down(Key::Down) {
        eye.y += move_speed; // Move eye down
    }
    if window.is_key_down(Key::Left) {
        eye.x -= move_speed; // Move eye left
    }
    if window.is_key_down(Key::Right) {
        eye.x += move_speed; // Move eye right
    }

    // Optionally, add controls for moving the camera forward/backward
    if window.is_key_down(Key::Q) {
        eye.z -= move_speed; // Move eye closer
    }
    if window.is_key_down(Key::E) {
        eye.z += move_speed; // Move eye farther
    }
}