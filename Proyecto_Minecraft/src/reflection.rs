use nalgebra_glm::Vec3;



pub fn reflection(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal)  * normal
}

