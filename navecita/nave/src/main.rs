use colors::Color;
use fragment::Fragment;
use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{dot, scaling, translation, Mat4, Vec3};
use vertex::Vertex;
use std::time::Duration;
mod framebuffer;
use framebuffer::Framebuffer;
mod castingray;
mod colors;
mod fragment;
mod line;
mod rayintersect;
mod uniform;
mod vertex;
use uniform::Uniforms;
mod obj;
mod vertexshader;
use obj::Obj;

fn create_model_matrix(translation_vec: Vec3, scale_factor: f32, rotation_angles: Vec3) -> Mat4 {
    // Create the translation matrix
    let translation_matrix = translation(&translation_vec);

    // Create the scaling matrix
    let scaling_matrix = scaling(&Vec3::new(scale_factor, scale_factor, scale_factor));

    // Create rotation matrices for each axis (assuming rotation angles are in radians)
    let rotation_x = nalgebra_glm::rotation(rotation_angles.x, &Vec3::x_axis());
    let rotation_y = nalgebra_glm::rotation(rotation_angles.y, &Vec3::y_axis());
    let rotation_z = nalgebra_glm::rotation(rotation_angles.z, &Vec3::z_axis());

    // Combine the transformations: scaling -> rotation -> translation
    let model_matrix = translation_matrix * rotation_z * rotation_y * rotation_x * scaling_matrix;

    model_matrix
}

fn calculate_bounding_box(v1: &Vec3, v2: &Vec3, v3: &Vec3) -> (i32, i32, i32, i32) {
    let min_x = v1.x.min(v2.x).min(v3.x).floor() as i32;
    let min_y = v1.y.min(v2.y).min(v3.y).floor() as i32;
    let max_x = v1.x.min(v2.x).min(v3.x).floor() as i32;
    let max_y = v1.y.min(v2.y).min(v3.y).floor() as i32;

    (min_x, min_y, max_x, max_y)
}

fn barycentric_coordinates(p: &Vec3, a: &Vec3, b: &Vec3, c: &Vec3) -> (f32, f32, f32) {
    let v0 = *b - *a;
    let v1 = *c - *a;
    let v2 = *p - *a;

    let d00 = v0.dot(&v0);
    let d01 = v0.dot(&v1);
    let d11 = v1.dot(&v1);
    let d20 = v2.dot(&v0);
    let d21 = v2.dot(&v1);

    let denom = d00 * d11 - d01 * d01;
    let v = (d11 * d20 - d01 * d21) / denom;
    let w = (d00 * d21 - d01 * d20) / denom;
    let u = 1.0 - v - w;

    (u, v, w)
}


fn triangle( v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {

    let mut fragment = Vec::new();
    let (a,b,c) = ( v1.transformed_position, v2.transformed_position, v3.transformed_position);

    let (min_x, min_y, max_x, max_y) = calculate_bounding_box( &a,&b, &c );

    for y in min_y..=max_y {
        for x in min_x..=max_x{
            let point = Vec3::new(x as f32, y as f32,0.0);

            let (w1, w2, w3) = barycentric_coordinates( &point, &a, &b, &c);

            if  w1>= 0.0 && w1 <= 1.0 &&
                w2>= 0.0 && w2 <= 1.0 &&
                w3>= 0.0 && w3 <= 1.0
             {

                let light_dir = Vec3::new(0.0, 0.0, -1.0);

                let normal = v1.transformed_normal;
                let normal = normal.normalize();

                let intensity = dot(&normal, &light_dir).max(0.0);

                let base_color = Color::new(100, 100, 100);
                let lit_color = base_color.adjust_brightness(intensity);

                fragment.push(Fragment::new(x as f32, y as f32, lit_color, 0.0));   
            }

        }
    }

    fragment
    
}



fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 800;
    let framebuffer_height = 600;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    framebuffer.set_background_color(0xFFFFFF);

    let mut window = Window::new(
        "KOALONSON ONSON",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    // Load the 3D object
    let obj = obj::Obj::load("./assets/xae21.obj").expect("Failed to load .obj file");
    let vertex_array = obj.get_vertex_array();

    // Initial translation
    let mut translation = Vec3::new(0.0, 0.0, 0.0);
    let scale_factor = 20.0;
    let mut rotation_angles = Vec3::new(0.0, 0.0, 0.0); // Make this mutable

    while window.is_open() && !window.is_key_down(Key::Escape) {
        framebuffer.clear();
        // Handle key presses for translation
        if window.is_key_down(Key::Up) {
            translation.y -= 1.0; // Move up
        }
        if window.is_key_down(Key::Down) {
            translation.y += 1.0; // Move down
        }
        if window.is_key_down(Key::Left) {
            translation.x -= 1.0; // Move left
        }
        if window.is_key_down(Key::Right) {
            translation.x += 1.0; // Move right
        }

        // Handle rotation
        if window.is_key_down(Key::W) {
            // Rotate up
            rotation_angles.x += 0.1; // Increase pitch
        }
        if window.is_key_down(Key::S) {
            // Rotate down
            rotation_angles.x -= 0.1; // Decrease pitch
        }
        if window.is_key_down(Key::A) {
            // Rotate left
            rotation_angles.y -= 0.1; // Decrease yaw
        }
        if window.is_key_down(Key::D) {
            // Rotate right
            rotation_angles.y += 0.1; // Increase yaw
        }

        // Create the model matrix with updated translation
        let model_matrix = create_model_matrix(translation, scale_factor, rotation_angles);
        // Update uniforms
        let view_matrix = Mat4::identity(); // Simple identity matrix for now
        let projection_matrix = Mat4::identity(); // Simple identity matrix for now
        let uniforms = uniform::Uniforms {
            model_matrix,
            view_matrix,
            projection_matrix,
        };

        // Vertex Processing Stage
        let transformed_vertices: Vec<vertex::Vertex> = vertex_array
            .iter()
            .map(|v| vertexshader::vertex_shader(v, &uniforms))
            .collect();

        // Primitive Assembly and Rasterization
        for triangle_indices in obj.indices.chunks(3) {
            if triangle_indices.len() < 3 {
                continue; // Skip incomplete triangles
            }

            let v0 = &transformed_vertices[triangle_indices[0] as usize];
            let v1 = &transformed_vertices[triangle_indices[1] as usize];
            let v2 = &transformed_vertices[triangle_indices[2] as usize];

            // Calculate the bounding box of the triangle
            let (min_x, min_y, max_x, max_y) =
                calculate_bounding_box(&v0.position, &v1.position, &v2.position);

            // Loop through each pixel in the bounding box
            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    let p = Vec3::new(x as f32, y as f32, 0.0); // Current pixel position

                    // Compute barycentric coordinates for the pixel
                    let (u, v, w) =
                        barycentric_coordinates(&p, &v0.position, &v1.position, &v2.position);

                    // Check if the pixel is inside the triangle
                    if u >= 0.0 && v >= 0.0 && w >= 0.0 && u + v + w <= 1.0 {
                        // Interpolate attributes (e.g., color) using barycentric coordinates
                        let interpolated_color = v0.color.lerp(&v1.color, v).lerp(&v2.color, w);

                        // Set the pixel color in the framebuffer
                        framebuffer.set_foreground_color(interpolated_color.to_hex());
                        framebuffer.point(x as usize, y as usize);
                    }
                }
            }
        }

        // Update the window with the framebuffer
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
