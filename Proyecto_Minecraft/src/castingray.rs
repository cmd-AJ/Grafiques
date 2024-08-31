use nalgebra_glm::{Vec3};
use crate::colors::Color;
use crate::sphere::{Rayintersect, Sphere};
use crate::rayintersect::{Intersect, RayIntersect, Material};

pub fn cast_ray(origin: &Vec3, direction: &Vec3, objects: &[Sphere]) -> Color {
    let mut intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;
    let mut pixel_color = 0x123442; // Default background color

    for object in objects {
        let i = object.ray_intersect(origin, direction);
        if i.is_intersecting 
        && i.distance < zbuffer{
            zbuffer = i.distance;
            intersect = i;
        }
    }
    if !(intersect.is_intersecting){
        return Color::new(89, 0, 152);
    } 
    let difuse = intersect.material.diffuse;

    return difuse;
        
    
}
