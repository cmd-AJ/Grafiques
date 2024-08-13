use nalgebra_glm::{Vec3, dot};
use crate::ray_intersect::{RayIntersect, Intersect, Material};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl RayIntersect for Sphere {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        let oc = ray_origin - self.center;

        let a = dot(ray_direction, ray_direction);
        let b = 2.0 * dot(&oc, ray_direction);
        let c = dot(&oc, &oc) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            Intersect::empty()
        } else {
            let dist = (-b - discriminant.sqrt()) / (2.0 * a);
            let intersection_point = ray_origin + ray_direction * dist;
            let normal = (intersection_point - self.center) / self.radius;

            Intersect::new(intersection_point, normal, dist, self.material)
        }
    }
}
