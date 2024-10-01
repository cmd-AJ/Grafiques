use nalgebra_glm::Vec3;
use crate::colors::Color;
use crate::material::Material;


#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Intersect {
    pub normal: Vec3,
    pub point: Vec3,
    pub distance: f32,
    pub is_intersecting: bool,
    pub material: Material,
    pub u: f32,
    pub v: f32,
}

impl Intersect {
    pub fn new( point: Vec3, normal: Vec3 ,distance: f32, material: Material, u:f32, v:f32) -> Self {
        Intersect {
            normal,
            point,
            distance,
            is_intersecting: true,
            material,
            u,
            v,
        }
    }

    pub fn empty() -> Self {
        Intersect {
            point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            distance: 0.0,
            is_intersecting: false,
            material: Material::black(),
            u: 0.0,
            v:0.0

        }
    }
}

pub trait RayIntersect {
  fn get_uv(&self, point: &Vec3, normal: &Vec3) -> (f32, f32);
  fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect;
}