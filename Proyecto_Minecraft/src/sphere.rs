use std::f32::consts::PI;

use nalgebra_glm::{dot, normalize, Vec3};
use crate::rayintersect::{Intersect, RayIntersect};
use crate::material::Material;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

pub trait Rayintersect {
    fn ray_intersect(&self, origin: &Vec3, direction: &Vec3) -> Intersect;
}

impl Sphere {
    fn get_uv(&self, point: &Vec3) -> (f32, f32){

        let normalized = ( *point - self.center)/ self.radius;

        let theta = (-normalized.y).acos();
        let phi = (-normalized.z).atan2(normalized.x) + PI;

        let u = phi / (2.0 * PI);
        let v = theta / PI;

        (u,v)

    }
} 

impl Rayintersect for Sphere {
    fn ray_intersect(&self, origin: &Vec3, direction: &Vec3) -> Intersect {
        let oc = origin - self.center;
        let a = dot(direction, direction);
        let b = 2.0 * dot(&oc, direction);
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            // Calculate the two possible intersection points
            let sqrt_discriminant = discriminant.sqrt();
            let t1 = (-b - sqrt_discriminant) / (2.0 * a);
            let t2 = (-b + sqrt_discriminant) / (2.0 * a);

            // Choose the closest positive intersection point
            let t = if t1 > 0.0 { t1 } else { t2 };

            if t > 0.0 {
                let point = origin + direction * t;
                let normal = (point - self.center).normalize();
                let distance = t;

                let (u,v)  = if self.material.texture.is_some() {
                    self.get_uv(&point)                    
                } else {
                    (0.0,0.0)
                };

                return Intersect::new( point, normal, distance, self.material.clone(), u, v);
            }
        }

        Intersect::empty()
    }
}
