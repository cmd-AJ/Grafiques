use crate::{cube::Cube, light::Light, rayintersect::{Intersect, RayIntersect}};



pub fn cast_shadow(
    intersect: &Intersect,
    light: &Light,
    objects: &[Cube],
) -> f32 {
    let light_dir = (light.position - intersect.point).normalize();
    let bias = 0.001; 
    let shadow_ray_origin = intersect.point + intersect.normal * bias;
    
    let mut shadow_intensity = 0.0;

    for object in objects {
        let shadow_intersect = object.ray_intersect(&shadow_ray_origin, &light_dir);
        if shadow_intersect.is_intersecting {
            shadow_intensity = 0.4;
            break;
        }
    }

    shadow_intensity
}