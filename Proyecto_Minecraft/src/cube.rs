use nalgebra_glm::Vec3;
use crate::rayintersect::{Intersect, RayIntersect};
use crate::material::Material;

pub struct Cube {
    pub center: Vec3,
    pub size: f32,  // Size represents the length of each side of the cube
    pub material: Material,
}
impl RayIntersect for Cube {

    fn get_uv(&self, point: &Vec3, normal: &Vec3) -> (f32, f32) {
        let half_size = self.size / 2.0;
        let local_point = *point - self.center;
    
        // Define the scale for the front, back, and bottom faces
        let front_scale = 0.26; // Scale for front and back faces
        let back_scale = 0.26; // Sc5le for front and back faces
        let bottom_scale = 0.26; // Scale for bottom face
        let side_scale = 0.26; // Scale for side faces
    
        if normal.z.abs() > 0.9 {
            // Front face (Z-axis)
            let u = (local_point.x + half_size) / self.size * back_scale + (1.0 - back_scale) / 2.0; // Adjusted for front face
            let v = (local_point.y + half_size) / self.size * back_scale + 0.70 * (1.0 - back_scale) / 2.0; // Adjusted for front face
            (u, v)  //SUBIR VALOR SIGNIFICA BAJAR 
    
        } else if normal.z.abs() < -0.9 {
            // Back face (Z-axis)
            let u = (local_point.x + half_size) / self.size * back_scale + (1.0 - back_scale) / 2.0; // Adjusted for back face
            let v = (local_point.y + half_size) / self.size * back_scale + 0.70 * (1.0 - back_scale); // Adjusted for back face
            (u, v)
    
        } else if normal.y.abs() > 0.0 {
            // Top face (Y-axis)
            let u = (local_point.x + half_size) / self.size * front_scale + (1.0 - (front_scale-0.01)) / 2.0; // Adjusted for top face
            let v = (local_point.z + half_size) / self.size * front_scale + 0.7 * (1.0-front_scale); // Top face
            (u, v)
    
        } else if normal.y.abs() < -0.0 {
            // Bottom face (Y-axis)
            let u = (local_point.x + half_size) / self.size * bottom_scale + (1.0 - bottom_scale) / 2.0; // Adjusted for bottom face
            let v = (local_point.z + half_size) / self.size * bottom_scale + 0.1 * (1.0 - bottom_scale); // Adjusted for bottom face
            (u, v)
    
        } else if normal.x.abs() > 0.0 {
            // Right face (X-axis)
            let u = (local_point.z + half_size) / self.size * side_scale + 0.5; // Adjusted for right face
            let v = (local_point.y + half_size) / self.size * side_scale + 0.25; // Adjusted for right face
            (u, v)
    
        } else {
            // Left face (X-axis)
            let u = (local_point.z + half_size) / self.size * side_scale; // Adjusted for left face
            let v = (local_point.y + half_size) / self.size * side_scale + 0.25; // Adjusted for left face
            (u, v)
        }
        
    }
    

    fn ray_intersect(&self, origin: &Vec3, direction: &Vec3) -> Intersect {
        let half_size = self.size / 2.0;
        let min_bound = self.center - Vec3::new(half_size, half_size, half_size);
        let max_bound = self.center + Vec3::new(half_size, half_size, half_size);

        let tmin = (min_bound - origin).component_div(direction);
        let tmax = (max_bound - origin).component_div(direction);

        let t1 = tmin.zip_map(&tmax, |a, b| a.min(b));
        let t2 = tmin.zip_map(&tmax, |a, b| a.max(b));

        let t_near = t1.x.max(t1.y).max(t1.z);
        let t_far = t2.x.min(t2.y).min(t2.z);

        if t_near < t_far && t_far > 0.0 {
            let distance = t_near;
            let point = origin + direction * distance;
            let normal = if point.x > max_bound.x - 0.0001 {
                Vec3::new(1.0, 0.0, 0.0) // Right face
            } else if point.x < min_bound.x + 0.0001 {
                Vec3::new(-1.0, 0.0, 0.0) // Left face
            } else if point.y > max_bound.y - 0.0001 {
                Vec3::new(0.0, 1.0, 0.0) // Top face
            } else if point.y < min_bound.y + 0.0001 {
                Vec3::new(0.0, -1.0, 0.0) // Bottom face
            } else if point.z > max_bound.z - 0.0001 {
                Vec3::new(0.0, 0.0, 1.0) // Front face
            } else {
                Vec3::new(0.0, 0.0, -1.0) // Back face
            };

            let (u, v) = self.get_uv(&point, &normal);

            return Intersect::new(point, normal, distance, self.material.clone(), u, v);
        }

        Intersect::empty()
    }
}
