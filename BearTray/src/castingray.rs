use nalgebra_glm::Vec3;

pub struct Object {
    // Define the properties of your Object, e.g., position, size, color, etc.
    pub position: Vec3,
    pub radius: f32,
    pub color: u32,
}

pub fn cast_ray(origin: &Vec3, direction: &Vec3, objects: &[Object]) -> u32 {
    // Implement ray-object intersection logic
    // Return the color of the closest intersected object, or a background color
    let mut closest_distance = f32::INFINITY;
    let mut pixel_color = 0x000000; // Default background color (black)

    for object in objects {
        // Calculate intersection
        let oc = origin - object.position;
        let a = direction.dot(&direction);
        let b = 2.0 * oc.dot(&direction);
        let c = oc.dot(&oc) - object.radius * object.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            let distance = (-b - discriminant.sqrt()) / (2.0 * a);
            if distance < closest_distance {
                closest_distance = distance;
                pixel_color = object.color;
            }
        }
    }

    pixel_color
}
