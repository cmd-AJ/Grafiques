use nalgebra_glm::{Vec3, dot};
use crate::rayintersect::{Intersect, Material, RayIntersect};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

pub trait Rayintersect {
    fn ray_intersect(&self, origin: &Vec3, direction: &Vec3) -> Intersect;
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

                return Intersect::new(distance, self.material);
            }
        }

        Intersect::empty()
    }
}
