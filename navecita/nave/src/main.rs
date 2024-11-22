use colors::Color;
use core::time;
use fastnoise_lite::{FastNoiseLite, NoiseType};
use fragment::Fragment;
use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{dot, Mat4, Vec2, Vec3, Vec4};
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
mod normal_map;
mod object;
use normal_map::init_normal_map;
use object::ObjectInPos;

fn calculate_bounding_box(v1: &Vec3, v2: &Vec3, v3: &Vec3) -> (i32, i32, i32, i32) {
    let min_x = v1.x.min(v2.x).min(v3.x).floor() as i32;
    let min_y = v1.y.min(v2.y).min(v3.y).floor() as i32;
    let max_x = v1.x.max(v2.x).max(v3.x).ceil() as i32;
    let max_y = v1.y.max(v2.y).max(v3.y).ceil() as i32;
    (min_x, min_y, max_x, max_y)
}

fn is_camera_inside_object(camera: &Camera, object: &ObjectInPos) -> bool {
    let (object_pos, object_radius) = object.bounding_sphere();

    // Calculate the distance between the camera and the object's center
    let distance = (camera.eye - object_pos).magnitude();

    // Check if the camera is inside or too close to the object
    distance < object_radius
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

fn calculate_orbit(center: Vec3, radius: f32, angle: f32, axis: char) -> Vec3 {
    match axis {
        'y' => Vec3::new(
            center.x + radius * angle.cos(),
            center.y, 
            center.z + radius * angle.sin(),
        ),
        _ => center,
    }
}

fn project_point(
    point: Vec3,
    view_matrix: &Mat4,
    projection_matrix: &Mat4,
    viewport_matrix: &Mat4,
) -> Vec3 {
    let point_homogeneous = Vec4::new(point.x, point.y, point.z, 1.0);
    let screen_space = viewport_matrix * projection_matrix * view_matrix * point_homogeneous;
    // Perform perspective divide
    let x = screen_space.x / screen_space.w;
    let y = screen_space.y / screen_space.w;
    let z = screen_space.z / screen_space.w;
    Vec3::new(x, y, z)
}

pub fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();
    let (a, b, c) = (
        v1.transformed_position,
        v2.transformed_position,
        v3.transformed_position,
    );

    let (t1, t2, t3) = (v1.tex_coords, v2.tex_coords, v3.tex_coords);

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

                let tex_u = t1.x * w1 + t2.x * w2 + t3.x * w3;
                let tex_v = t1.y * w1 + t2.y * w2 + t3.y * w3;

                fragments.push(Fragment::new(
                    x as f32,
                    y as f32,
                    lit_color,
                    depth,
                    normal,
                    intensity,
                    vertex_position,
                    Vec2::new(tex_u, tex_v),
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
    let window_width = 1200;
    let window_height = 900;
    let mut framebuffer = Framebuffer::new(window_width, window_height);
    let start_time = std::time::Instant::now();
    let mut static_pattern =
        Framebuffer::generate_static_pattern(window_width, window_height, 0.001);

    let mut objects: Vec<ObjectInPos> = Vec::new();
    let orbital_period = 5.0;
    let parameters = vec![
        (
            10.0,
            "./assets/sphere-1.obj",
            "6",
            Vec3::new(0.0, 0.0, 40.0),
            Vec3::new(0.0, 0.0, 0.0),
        ), // Numpad 6
        (
            20.0,
            "./assets/sphere-1.obj",
            "2",
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
        ), // Numpad 2

        (
            6.0,
            "./assets/SATURN.obj",
            "1",
            Vec3::new(0.0, 200.0, 150.0),
            Vec3::new(0.0, 0.0, 0.0),
        ), // Numpad 1
        (
            8.0,
            "./assets/sphere-1.obj",
            "3",
            Vec3::new(0.0, 0.0, 30.0),
            Vec3::new(0.0, 0.0, 0.0),
        ), // Numpad 3
     
        (
            4.0,
            "./assets/rockos.obj",
            "4",
            Vec3::new(0.0, 00.0, 60.0),
            Vec3::new(0.0, 0.0, 0.0),
        ), // Numpad 4
        (
            3.0,
            "./assets/sphere-1.obj",
            "5",
            Vec3::new(0.0, 0.0, 50.0),
            Vec3::new(0.0, 0.0, 0.0),
        ), // Numpad 5
        
        (
            3.0,
            "./assets/sphere-1.obj",
            "7",
            Vec3::new(0.0, 0.0, 60.0),
            Vec3::new(0.0, 0.0, 0.0),
        ), // Numpad 7
        (
            0.2f32,
            "./assets/xae21.obj",
            "7",
            Vec3::new(40.0, 0.0, -240.0),
            Vec3::new(0.0, 0.0, 0.0),
        ),
    ];

    init_normal_map("assets/normalmap.png").expect("Failed To load normal map");

    // Iterate through the parameters and create ObjectInPos for each entry
    for param in parameters {
        let (scale_factor, obj_path, blend_type, position, rotation) = param;
        let object = ObjectInPos::new(scale_factor, obj_path, blend_type, position, rotation);
        objects.push(object); // Add the object to the vector
    }

    let mut window = Window::new(
        "Mother Zeta Fallut 3",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    let mut obj = Obj::load("./assets/xae21.obj").expect("Failed to load .obj file");
    let mut vertex_array = obj.get_vertex_array();

    let mut scale_factor = 0.2f32;
    let mut rotation_angles = Vec3::new(0.0, 0.0, 0.0);

    let mut camera =  Camera::new(
        Vec3::new(100.0, 140.0, -250.0),
        Vec3::new(17.0, 0.0, 11.0),
        Vec3::new(0.0, 1.0, 0.0),
        false,
    );


    //CAMARA
    let mut eye = Vec3::new(0.0, 0.0, 5.0);
    let mut center = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);

    let mut angle = 0.0; // Initialize the angle to 0
    let full_revolution_time = 5.0; // Time for one full revolution (in seconds)
    let angular_step = (2.0 * PI) / (full_revolution_time * 260.0); // Increment per frame for

    let mut blend_type = "2";
    let initial_positions: Vec<Vec3> = objects.iter().map(|obj| obj.position).collect();
    let mut is_move = true;
    let mut last_mouse_pos = None; // Store the previous mouse position

    while window.is_open() && !window.is_key_down(Key::Escape) {
       
        let elapsed = start_time.elapsed();
        let time = elapsed.as_secs_f32() as u32; //

        let movement_speed = 1.0;
        let rotation_speed = PI / 50.0;
        let zoom_speed = 0.4;

        angle += angular_step;

        if angle > 2.0 * PI {
            angle -= 2.0 * PI;
        }

        //  camera orbit controls
        let mut movement = Vec3::new(0.0, 0.0, 0.0);

        if window.is_key_down(Key::W) {
            camera.orbit(0.0, -rotation_speed);

        }
        if window.is_key_down(Key::S) {
            camera.orbit(0.0, rotation_speed);
            
        }

        if window.is_key_down(Key::A) {
            
            camera.orbit(rotation_speed, 0.0);
        }
        if window.is_key_down(Key::D) {
            camera.orbit(-rotation_speed, 0.0);
            
        }

        // Camera movement controls

        if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(minifb::MouseMode::Clamp) {
            if let Some((last_x, last_y)) = last_mouse_pos {
                // Calculate the mouse delta
                let delta_x = mouse_x - last_x;
                let delta_y = mouse_y - last_y;

               
                let mouse_sensitivity = 0.1; // Adjust sensitivity as needed
                camera.move_center(Vec3::new(
                    delta_x as f32 * mouse_sensitivity,
                    delta_y as f32 * mouse_sensitivity, // Remove the `-1.0` if it's not needed
                    0.0,
                ));
            }

            // Update the last mouse position
            last_mouse_pos = Some((mouse_x, mouse_y));
        } else {
            // If no mouse position is detected, reset the last position
            last_mouse_pos = None;
        }

        if let Some((_, vertical_scroll)) = window.get_scroll_wheel() {
            let zoom_speed: f32 = 0.1;
            if vertical_scroll > 0.0 {
                // Scroll up
                camera.zoom(zoom_speed);
            } else if vertical_scroll < 0.0 {
                // Scroll down
                camera.zoom(-zoom_speed);
            }
        }

        if movement.magnitude() > 0.0 {
            camera.move_center(movement);
        }

        if window.is_key_down(Key::Down) {
            is_move = true;
            static_pattern =
                Framebuffer::generate_static_pattern(window_width, window_height, 0.0007);
            camera.zoom(zoom_speed);

            objects[7].position.z -= zoom_speed;

            // Update the camera's center to follow the object
            camera.center = objects[7].position;

            camera.eye = camera.center - Vec3::new(0.0, -1.8, 8.0); // Adjust eye to stay behind the object
        }

        if window.is_key_down(Key::Up) {
            is_move = true;
            static_pattern =
                Framebuffer::generate_static_pattern(window_width, window_height, 0.0007);
            camera.zoom(-zoom_speed);

            objects[7].position.z += zoom_speed;

            camera.center = objects[7].position;

            // Adjust the camera's eye position to maintain proper distance and direction
            camera.eye = camera.center - Vec3::new(0.0, -1.8, 8.0); // Adjust eye to stay behind the object
        }

        if window.is_key_down(Key::Right) {
            is_move = true;
            if objects[7].rotation.z > -0.2 {
                objects[7].rotation.z -= rotation_speed;
            }

            if objects[7].rotation.y > -0.2 {
                objects[7].rotation.y -= rotation_speed; // Yaw (counter-clockwise)
            }

            objects[7].position.x -= movement_speed; // Move object left
        }

        if window.is_key_down(Key::Left) {
             is_move = true;
            if objects[7].rotation.z < 0.2 {
                objects[7].rotation.z += rotation_speed;
            }

            if objects[7].rotation.y < 0.2 {
                objects[7].rotation.y += rotation_speed; // Yaw (clockwise)
            }

            objects[7].position.x += movement_speed; // Move object right
        }

        if window.is_key_down(Key::NumPadPlus) {
            objects[7].position.z += 1.0;
        }

        if window.is_key_down(Key::NumPad1) { //Sol

            camera = Camera::new(
                Vec3::new(26.306, 80.0, -18.0),
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(0.0, 10.0, 0.0),
                false,
            );
            
        }


        if window.is_key_down(Key::NumPad2) { //planeta nnm

            camera = Camera::new(
                Vec3::new(objects[0].position.x, 150.0, objects[0].position.z),
                Vec3::new(objects[0].position.x, 00.0, objects[0].position.z+1.0),
                Vec3::new(0.0, 10.0, 0.0),
                false,
            );

            is_move = false
            
        }

        if window.is_key_down(Key::NumPad3) { //Saturno

            camera = Camera::new(
                Vec3::new(objects[2].position.x, 150.0, objects[2].position.z),
                Vec3::new(objects[2].position.x, 00.0, objects[2].position.z+1.0),
                Vec3::new(0.0, 10.0, 0.0),
                false,
            );

            is_move = false
            
        }

        if window.is_key_down(Key::NumPad4) { //Sol

            camera = Camera::new(
                Vec3::new(objects[3].position.x, 150.0, objects[3].position.z),
                Vec3::new(objects[3].position.x, 00.0, objects[3].position.z+1.0),
                Vec3::new(0.0, 10.0, 0.0),
                false,
            );

            is_move = false
            
        }

        if window.is_key_down(Key::NumPad5) { //Sol

            camera = Camera::new(
                Vec3::new(objects[4].position.x, 150.0, objects[4].position.z),
                Vec3::new(objects[4].position.x, 00.0, objects[4].position.z+1.0),
                Vec3::new(0.0, 10.0, 0.0),
                false,
            );

            is_move = false
            
        }


        if window.is_key_down(Key::NumPad6) { //Sol

            camera = Camera::new(
                Vec3::new(objects[5].position.x, 150.0, objects[5].position.z),
                Vec3::new(objects[5].position.x, 00.0, objects[5].position.z+1.0),
                Vec3::new(0.0, 10.0, 0.0),
                false,
            );

            is_move = false
            
        }



        if window.is_key_down(Key::NumPad7) { //Sol

            camera = Camera::new(
                Vec3::new(objects[6].position.x, 150.0, objects[6].position.z),
                Vec3::new(objects[6].position.x, 00.0, objects[6].position.z+1.0),
                Vec3::new(0.0, 10.0, 0.0),
                false,
            );

            is_move = false
            
        }






        if window.is_key_down(Key::P) {
            is_move = true;
            camera = Camera::new(
                Vec3::new(0.0, 300.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(0.0, 1.0, 0.0),
                false,
            );
        }

        if is_move == true {
            for i in 0..objects.len() {
                if i != 1 && i != 7 { // Skip object 1 (the Sun)
                    let orbit_center = objects[1].position; 
            
                    // Increase radius significantly to separate objects more
                    let radius = (10.0 + i as f32 * 6.0) * objects[i].scale_factor; 
            
                    // Increase angle offset to spread objects across the orbit
                    let angle_offset = i as f32 * 0.8; 
            
                    // Update the position of the orbiting object
                    objects[i].position = calculate_orbit(orbit_center, radius, angle + angle_offset, 'y');
                }
            }

        }

      

        framebuffer.clear(); // Clear the framebuffer for each frame

        framebuffer.apply_static_pattern(&static_pattern);

        let view_matrix = create_view_matrix(camera.eye, camera.center, camera.up); // Adjust for camera if needed
        let projection_matrix =
            create_perspective_matrix(window_width as f32, window_height as f32); // Set up perspective projection
        let viewport_matrix = create_viewport_matrix(window_width as f32, window_height as f32);
        let mut noise = create_noise(&blend_type);

        for object in &objects {
            if !is_camera_inside_object(&camera, object) {
                // Create model matrix for each object based on its position, scale, and rotation
                let model_matrix =
                    create_model_matrix(object.position, object.scale_factor, object.rotation);

                // Set up the uniforms for the render
                let uniforms = Uniforms {
                    model_matrix,
                    view_matrix,
                    projection_matrix,
                    viewport_matrix,
                    time,
                    noise: create_noise(&object.blend_type),
                };

                // Render the object
                render(
                    &mut framebuffer,
                    &uniforms,
                    &object.vertex_array,
                    &object.blend_type,
                );
            }
        }

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer.width, framebuffer.height)
            .unwrap();
        std::thread::sleep(Duration::from_millis(1)); // Control frame rate
    }
}
