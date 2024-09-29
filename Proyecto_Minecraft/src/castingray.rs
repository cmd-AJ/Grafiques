use nalgebra_glm::Vec3;
use crate::colors::Color;
use crate::reflection::{self, reflection};
use crate::shadow::{self, cast_shadow};
use crate::sphere::{Rayintersect, Sphere};
use crate::rayintersect::{Intersect, RayIntersect};
use crate::light::Light;


pub fn cast_ray(origin: &Vec3, direction: &Vec3, objects: &[Sphere], light: &Light,) -> Color {
    let mut intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;

    for object in objects {
        let i = object.ray_intersect(origin, direction);
        if i.is_intersecting 
        && i.distance < zbuffer{
            zbuffer = i.distance;
            intersect = i;
        }
    }
    if !(intersect.is_intersecting){
        return Color::new(89, 0, 12);
    } 

    let light_dir = (light.position - intersect.point).normalize();
    let view_dir = (origin - intersect.point).normalize();
    let intensity = light.intensity;
    let reflect_dir = reflection(&-light_dir, &intersect.normal).normalize();
    let shadow_intensity = cast_shadow(&intersect, light, objects);
    let light_intensity = light.intensity * (1.0 - shadow_intensity);

    let diffuse_intensity = intersect.normal.dot(&light_dir);
    let difuse = intersect.material.diffuse.adjust_brightness( intersect.material.albedo[0] * (diffuse_intensity * intensity));

    let specular_intensity = view_dir.dot(&reflect_dir).max(0.0).powf(intersect.material.specular);
    let specular = light.color.adjust_brightness(intersect.material.albedo[1] * specular_intensity * light_intensity)  ; 

    return difuse.blend(&specular);
        
    
}
