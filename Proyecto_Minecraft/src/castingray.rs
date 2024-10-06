use nalgebra_glm::Vec3;
use crate::colors::Color;
use crate::cube::Cube;
use crate::shadow::cast_shadow;
use crate::rayintersect::{Intersect, RayIntersect};
use crate::light::Light;
use crate::r_stations::{offset_origin, reflection, refract};

pub fn cast_ray(origin: &Vec3, direction: &Vec3, objects: &[Cube], light: &Light, depth: u32) -> Color {
    let mut intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;
    let mut intersected_object: Option<&Cube> = None; // Track the intersected object

    if depth > 3 {
        return Color::skybox_color();
    }

    // Loop through objects to find intersections
    for object in objects {
        let i = object.ray_intersect(origin, direction);
        if i.is_intersecting && i.distance < zbuffer {
            zbuffer = i.distance;
            intersect = i;
            intersected_object = Some(object); // Store the intersected object
        }
    }

    // If no intersection, return the skybox color
    if !intersect.is_intersecting {
        return Color::skybox_color();
    }

    // Get the UV coordinates from the intersected cube
    let (u, v) = intersected_object.unwrap().get_uv(&intersect.point, &intersect.normal);
    // Get the color based on the UV coordinates
    let diffuse_color = intersected_object.unwrap().material.get_diffuse(u, v);

    let light_dir = (light.position - intersect.point).normalize();
    let view_dir = (origin - intersect.point).normalize();
    let reflect_dir = reflection(&-light_dir, &intersect.normal).normalize();
    let shadow_intensity = cast_shadow(&intersect, light, objects);
    let light_intensity = light.intensity * (1.0 - shadow_intensity);

    let diffuse_intensity = intersect.normal.dot(&light_dir).max(0.0);

    let adjusted_diffuse_color = diffuse_color.adjust_brightness(intersect.material.albedo[0] * (diffuse_intensity * light_intensity));

    let specular_intensity = view_dir.dot(&reflect_dir).max(0.0).powf(intersect.material.specular);
    let specular = light.color.adjust_brightness(intersect.material.albedo[1] * specular_intensity * light_intensity);

    let mut reflect_color = Color::black();
    let reflectivity = intersect.material.albedo[2];
    if reflectivity > 0.0 {
        let reflect_dir = reflection(&direction, &intersect.normal).normalize();
        let reflect_origin = offset_origin(&intersect, &reflect_dir);
        reflect_color = cast_ray(&reflect_origin, &reflect_dir, objects, light, depth + 1);
    }

    let mut refract_color = Color::black();
    let transparency = intersect.material.albedo[3];
    if transparency > 0.0 {
        let refract_dir = refract(&direction, &intersect.normal, intersect.material.refraction_index);
        let refract_origin = offset_origin(&intersect, &refract_dir);
        refract_color = cast_ray(&refract_origin, &refract_dir, objects, light, depth + 1);
    }

    return adjusted_diffuse_color.blend(&specular)
        .adjust_brightness(1.0 - reflectivity - transparency)
        .blend(&reflect_color.adjust_brightness(reflectivity))
        .blend(&refract_color.adjust_brightness(transparency));
}
