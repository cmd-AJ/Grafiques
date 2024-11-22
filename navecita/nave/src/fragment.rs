// fragment.rs

use std::f32::consts::PI;

use nalgebra_glm::{fract, sin, Vec2, Vec3};
use crate::{colors::Color, uniform::{self, Uniforms}};
use crate::vertexshader::calculate_tangent_lighting;

pub struct Fragment {
    pub position: Vec2,
    pub color: Color,
    pub depth: f32,
    pub normal: Vec3,
    pub intensity: f32,
    pub vertex_position: Vec3,
    pub tex_coords: Vec2,
}

impl Fragment {
    pub fn new(x: f32, y: f32, color: Color, depth: f32, normal: Vec3, intensity:f32, vertex_position: Vec3,tex_coords: Vec2,) -> Self {
        Fragment {
            position: Vec2::new(x, y),
            color,
            depth,
            normal,
            intensity,
            vertex_position,
            tex_coords

        }
    }

}






//Para hacer las combinaciones de los planetas

pub fn fragment_shader(fragment: &Fragment, uniform: &Uniforms) -> Color {
    // Central point of the circles (e.g., the origin or center of the viewport)
    let center = Vec3::new(0.0, 0.0, 0.0); // Adjust as needed for the center of your circles

    // Calculate the radial distance from the center
    let distance_from_center = (fragment.vertex_position - center).magnitude();

    // Colors to cycle through for the concentric circles
    let colors = [
        Color::new(220, 205, 150),
        Color::new(210, 180, 140),
        Color::new(160, 140, 110), // Color for outer stripes
        Color::new(180, 170, 155),
        Color::new(200, 190, 170),
    ];

    // Define the width of each circular band
    let circle_width = 0.15;
    let circle_index_float = (distance_from_center / circle_width).abs();

    // Define the range for the striped pattern
    let stripe_radius_start = 0.50;
    let stripe_radius_end = 0.55;

    // Check if distance falls within the stripe range
    if distance_from_center >= stripe_radius_start && distance_from_center <= stripe_radius_end {
        // Generate a stripe pattern within the specified range
        let stripe_width = 0.02; // Width of each stripe
        let stripe_float = (fragment.vertex_position.y / stripe_width).abs();
        
        // Alternate between two colors for the stripes
        let stripe_index = (stripe_float as usize) % 2;
        let stripe_colors = [Color::new(200, 150, 100), Color::new(150, 100, 50)];

        return stripe_colors[stripe_index].adjust_brightness(fragment.intensity);
    }

    // Otherwise, apply the circular pattern outside the stripe range
    let circle_index = (circle_index_float as usize) % colors.len();
    let next_index = (circle_index + 1) % colors.len();
    let t = circle_index_float.fract();

    // Interpolate and adjust brightness based on fragment intensity
    colors[circle_index].lerp(&colors[next_index], t).adjust_brightness(fragment.intensity)
}


pub fn static_pattern_shader( fragment: &Fragment, uniform: &Uniforms) -> Color {
    let zoom = 250.0;
    let ox = 0.0;
    let oy = 0.0;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;

    let variable_time = (uniform.time % 50) as f32;

    let noise_value = uniform.noise.get_noise_2d(
        (x + ox) * zoom + variable_time, 
        (y + oy) * zoom + variable_time
    );

    let spot_threshold = 0.5;
    let base_color = Color::new(220, 220, 220); // Neptune's base blue
    let spot_color = cloudwind(fragment); // Assume this function provides a spot color
    let base_color2 =   Color::new(180, 180, 180); // White color for glow
    let glow = glow_shader(fragment); // Assume this function provides a glow color
    
    // Interpolating colors based on noise_value using fract
    let noise_interpolated_color = if noise_value < spot_threshold {
        if ((noise_value * 10.0) as usize % 2) == 1 {
            // If within a time range, return base_color2 or a glowing version
            if (0.0..=20.0).contains(&variable_time) {
                base_color2
            } else {
                let glow_color = base_color2.blend_screen(&glow); // Blend the glow effect
                glow_color
            }
        } else {
            spot_color // Use the spot color if above threshold
        }
    } else {
        base_color // Default color if noise_value is greater than threshold
    };

    // Fractal smooth interpolation based on the noise value for smooth blending
    let t = noise_value.fract(); // Get the fractional part of the noise value
    let final_color = base_color.lerp(&noise_interpolated_color, t); // Linearly interpolate between the colors

    final_color.adjust_brightness(fragment.intensity) // Adjust brightness based on fragment intensity
}

pub fn background_color_neptuno( _fragment: &Fragment) -> Color {
    Color::new(28, 56, 191)
}


pub fn combining_nubes_shader(fragment: &Fragment, uniform:&Uniforms) -> Color{
    let zoom = 250.0;
    let ox = 0.0;
    let oy = 0.0;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;

    let variable_time = (uniform.time % 50) as f32;

    let noise_value = uniform.noise.get_noise_2d(
        (x + ox) * zoom + variable_time, 
        (y + oy) * zoom + variable_time
    );

    let spot_threshold = 0.5;
    let base_color = Color::new(87, 135, 191); // Neptune's base blue
    let spot_color = background_color_neptuno(fragment); // Assume this function provides a spot color
    let base_color2 = Color::new(255, 255, 255); // White color for glow
    let glow = glow_shader(fragment); // Assume this function provides a glow color
    
    // Interpolating colors based on noise_value using fract
    let noise_interpolated_color = if noise_value < spot_threshold {
        if ((noise_value * 10.0) as usize % 2) == 1 {
            // If within a time range, return base_color2 or a glowing version
            if (0.0..=20.0).contains(&variable_time) {
                base_color2
            } else {
                let glow_color = base_color2.blend_screen(&glow); // Blend the glow effect
                glow_color
            }
        } else {
            spot_color // Use the spot color if above threshold
        }
    } else {
        base_color // Default color if noise_value is greater than threshold
    };

    // Fractal smooth interpolation based on the noise value for smooth blending
    let t = noise_value.fract(); // Get the fractional part of the noise value
    let final_color = base_color.lerp(&noise_interpolated_color, t); // Linearly interpolate between the colors

    final_color.adjust_brightness(fragment.intensity) // Adjust brightness based on fragment intensity
}


pub fn backgroundsea( _fragment: &Fragment) -> Color {
    Color::new(0, 105, 148)
}

pub fn earth( _fragment: &Fragment) -> Color {
    Color::new (34, 139, 34)
}

pub fn cloudwind( _fragment: &Fragment) -> Color {
    Color::new(255, 255, 255)
}


pub fn combining_eath(fragment: &Fragment, uniform:&Uniforms) -> Color{
    let zoom = 250.0;
    let ox = 0.0;
    let oy = 0.0;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;

    let variable_time = (uniform.time % 50) as f32;

    let noise_value = uniform.noise.get_noise_2d(
        (x + ox) * zoom + variable_time, 
        (y + oy) * zoom + variable_time
    );


    let spot_threshold = 0.5;
    let base_color = earth(fragment);
    let spot_color = backgroundsea(fragment);
    let base_color2 = cloudwind(fragment);
    
    
    let noise_color = if noise_value < spot_threshold{
        
        if ((noise_value * 10.0) as usize % 2) == 1 {

            if (0.0..=20.0).contains(&variable_time) {
                base_color2
            }
            else {
                base_color2
            }

        }
        else {
            spot_color    
        }        
        
    } else{

        base_color
        
    };


    noise_color.adjust_brightness(fragment.intensity)
    

}




pub fn survivingmars(fragment: &Fragment, uniform:&Uniforms) -> Color{
    let zoom = 250.0;
    let ox = 0.0;
    let oy = 0.0;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;

    let variable_time = (uniform.time % 50) as f32;

    let noise_value = uniform.noise.get_noise_2d(
        (x + ox) * zoom + variable_time, 
        (y + oy) * zoom + variable_time
    );


    let spot_threshold = 0.5;
    let base_color = Color::new (139, 69, 19);
    let base_color2 = Color::new(205, 92, 92);
    
    
    let noise_color = if noise_value < spot_threshold{
        base_color2
        
    } else{

        base_color
        
    };


    noise_color.adjust_brightness(fragment.intensity)
    

}


   

pub fn glow_shader(fragment: &Fragment) -> Color {
    let y = fragment.vertex_position.y;
    let stripe_width = 0.2;
    
    let glow_size = 0.25;


    let distance_to_center = (y % stripe_width - stripe_width/2.0);
    let glow_intensity = ((1.0 - (distance_to_center / glow_size).min(1.0)) * PI /2.0).sin();

    Color::new(
        (glow_intensity * 255.0) as u8,
        (0.05 * glow_intensity * 255.0) as u8, 
        (0.05 *glow_intensity*255.0) as u8
    )

}

pub fn background_shader_sun(_fragment: &Fragment) -> Color{

    Color::new(209, 64, 9)

}

pub fn random_color_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color{


    let intensity = calculate_tangent_lighting(fragment);
    let seed = uniforms.time as usize;

    let galaxy_colors = vec![
        Color::new(140, 140, 140), // Medium gray
        Color::new(100, 100, 100), // Medium dark gray
        Color::new(60, 60, 60),    // Dark gray
        Color::new(30, 30, 30),    // Very dark gray   // Black (darkest gray)
    ];

    let random_color = galaxy_colors[seed % galaxy_colors.len()];

    random_color.adjust_brightness(intensity/0.1)

}



pub fn sun_shader(fragment: &Fragment, uniform:&Uniforms) -> Color{
    let zoom = 240.0;
    let ox = 0.0;
    let oy = 0.0;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;

    let variable_time = (uniform.time % 50) as f32;

    let noise_value = uniform.noise.get_noise_2d(
        (x + ox) * zoom + variable_time, 
        (y + oy) * zoom + variable_time
    );


    let spot_threshold = 0.5;
    let spot_color = background_shader_sun(fragment);
    let base_color = Color::new(255, 255, 0);
    let base_color2 = Color::new(255, 255, 255);
    let glow = glow_shader(fragment);
    
    
    let noise_color = if noise_value < spot_threshold{
        
        if ((noise_value * 10.0) as usize % 2) == 1 {

            if (0.0..=20.0).contains(&variable_time) {
                base_color2
            }
            else {
                let glow_color = base_color2.blend_screen(&glow);
                glow_color
            }

        }
        else {
            spot_color    
        }        
        
    } else{

        base_color
        
    };


    noise_color
}