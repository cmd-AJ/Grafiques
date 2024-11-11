use std::f32::consts::PI;

use crate::fragment::{combining_eath, combining_nubes_shader, fragment_shader, neon_light_shader, random_color_shader, static_pattern_shader, sun_shader, survivingmars
};
use crate::framebuffer::Framebuffer;
use crate::triangle;
use crate::vertex::Vertex;
use crate::vertexshader::vertex_shader;
use fastnoise_lite::{FastNoiseLite, NoiseType};
use nalgebra_glm::{look_at, perspective, scaling, translation, Mat4, Vec3};

pub struct Uniforms {
    pub model_matrix: Mat4,
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
    pub viewport_matrix: Mat4,
    pub time: u32,
    pub noise: FastNoiseLite,
}

pub fn render(
    framebuffer: &mut Framebuffer,
    uniforms: &Uniforms,
    vertex_array: &[Vertex],
    blend_mode: &str,
) {
    // Vertex Shader Stage
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    // Primitive Assembly Stage
    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    // Rasterization Stage
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    // Fragment Processing Stage
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;
        if x < framebuffer.width && y < framebuffer.height {
            // let shaded_color = fragment_shader(&fragment, &uniforms);

            //Saturno
            if blend_mode == "1" {
                let shaded_color = fragment_shader(&fragment, &uniforms);
                let color = shaded_color.to_hex();
                framebuffer.point(x, y, color, fragment.depth);
            }

            //SOL
            if blend_mode == "2" {
                let shaded_color = sun_shader(&fragment, &uniforms);
                let color = shaded_color.to_hex();
                framebuffer.point(x, y, color, fragment.depth);
            }

            if blend_mode == "3" {
                let shaded_color = combining_nubes_shader(&fragment, &uniforms);
                let color = shaded_color.to_hex();
                framebuffer.point(x, y, color, fragment.depth);
            }

                //Tierra
            if blend_mode == "4" {
                let shaded_color = combining_eath(&fragment, &uniforms);
                let color = shaded_color.to_hex();
                framebuffer.point(x, y, color, fragment.depth);
            }
            if blend_mode == "5" {
                let shaded_color = survivingmars(&fragment, &uniforms);
                let color = shaded_color.to_hex();
                framebuffer.point(x, y, color, fragment.depth);
            }

            if blend_mode == "6" {
                let shaded_color = random_color_shader(&fragment, &uniforms);
                let color = shaded_color.to_hex();
                framebuffer.point(x, y, color, fragment.depth);
            }

            if blend_mode == "7" {
                let shaded_color = static_pattern_shader(&fragment, &uniforms);
                let color = shaded_color.to_hex();
                framebuffer.point(x, y, color, fragment.depth);
            }
        }
    }
}

pub fn create_noise(blend_mode: &str) -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(1338);
    if blend_mode == "2" {
        noise.set_noise_type(Some(NoiseType::OpenSimplex2));
        noise.frequency = 0.202;
    }
    if blend_mode == "3" {
        noise = FastNoiseLite::with_seed(2);
        noise.set_noise_type(Some(NoiseType::OpenSimplex2));
        noise.frequency = 0.007;
 
    }
    if blend_mode == "4" {
        noise = FastNoiseLite::with_seed(1652);
        noise.set_noise_type(Some(NoiseType::OpenSimplex2));
        noise.frequency = 0.0040;
    }
    if blend_mode == "5" {
        noise = FastNoiseLite::with_seed(1205);
        noise.set_noise_type(Some(NoiseType::OpenSimplex2));
        noise.frequency = 0.0060;
    }

    if blend_mode == "7" {
        noise = FastNoiseLite::with_seed(1100);
        noise.set_noise_type(Some(NoiseType::OpenSimplex2));
        noise.frequency = 0.069;
    }

    noise

}

pub fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    let rotation_matrix_x = Mat4::new(
        1.0, 0.0, 0.0, 0.0, 0.0, cos_x, -sin_x, 0.0, 0.0, sin_x, cos_x, 0.0, 0.0, 0.0, 0.0, 1.0,
    );

    let rotation_matrix_y = Mat4::new(
        cos_y, 0.0, sin_y, 0.0, 0.0, 1.0, 0.0, 0.0, -sin_y, 0.0, cos_y, 0.0, 0.0, 0.0, 0.0, 1.0,
    );

    let rotation_matrix_z = Mat4::new(
        cos_z, -sin_z, 0.0, 0.0, sin_z, cos_z, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    let transform_matrix = Mat4::new(
        scale,
        0.0,
        0.0,
        translation.x,
        0.0,
        scale,
        0.0,
        translation.y,
        0.0,
        0.0,
        scale,
        translation.z,
        0.0,
        0.0,
        0.0,
        1.0,
    );

    transform_matrix * rotation_matrix
}

pub fn create_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    look_at(&eye, &center, &up)
}

pub fn create_perspective_matrix(window_width: f32, window_height: f32) -> Mat4 {
    let fov = 45.0 * PI / 180.0;
    let aspect_ratio = window_width / window_height;
    let near = 0.1;
    let far = 1000.0;

    perspective(fov, aspect_ratio, near, far)
}

pub fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0,
        0.0,
        0.0,
        width / 2.0,
        0.0,
        -height / 2.0,
        0.0,
        height / 2.0,
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    )
}
