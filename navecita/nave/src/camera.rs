use nalgebra_glm::{rotate_vec3, Vec3};
use std::f32::consts::PI;

pub struct Camera {
    pub eye: Vec3,
    pub center: Vec3,
    pub up: Vec3,
    pub has_change: bool,
}

impl Camera {
    pub fn new(eye: Vec3, center: Vec3, up: Vec3, has_change: bool) -> Self {
        Camera {
            eye,
            center,
            up,
            has_change,
        }
    }

    pub fn basis_change(&self, vector: &Vec3) -> Vec3 {
        let forward = (self.center - self.eye).normalize();
        let right = forward.cross(&self.up).normalize();
        let up = right.cross(&forward).normalize();

        let rotated = vector.x * right + vector.y * up + -vector.z * forward;
        rotated.normalize()
    }

    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        let radius_vector = self.eye - self.center;
        let radius = radius_vector.magnitude();

        let current_yaw = radius_vector.z.atan2(radius_vector.x);
        let radius_xz =
            (radius_vector.x * radius_vector.x + radius_vector.z * radius_vector.z).sqrt();
        let current_pitch = (-radius_vector.y).atan2(radius_xz);

        let new_yaw = (current_yaw + delta_yaw) % (2.0 * PI);
        let new_pitch = (current_pitch + delta_pitch).clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);

        let new_eye = self.center
            + Vec3::new(
                radius * new_yaw.cos() * new_pitch.cos(),
                -radius * new_pitch.sin(),
                radius * new_yaw.sin() * new_pitch.cos(),
            );

        self.eye = new_eye;
        self.has_change = true; // Set has_change to true after orbiting
    }

    pub fn zoom(&mut self, delta: f32) {
        let direction = (self.center - self.eye).normalize();
        self.eye += direction * delta;
        self.has_change = true;
    }

    pub fn check_change(&mut self) -> bool {
        if self.has_change {
            self.has_change = false; // Reset the change flag
            true // Return true if there was a change
        } else {
            false // Return false if there was no change
        }
    }

    pub fn move_center(&mut self, direction: Vec3) {
        let radius_vector = self.center - self.eye;
        let radius = radius_vector.magnitude();

        let angle_x = direction.x * 0.05; // Adjust this factor to control rotation speed
        let angle_y = direction.y * 0.05;

        let rotated = rotate_vec3(&radius_vector, angle_x, &Vec3::new(0.0, 1.0, 0.0));

        let right = rotated.cross(&self.up).normalize();
        let final_rotated = rotate_vec3(&rotated, angle_y, &right);

        self.center = self.eye + final_rotated.normalize() * radius;
        self.has_change = true;
    }
}
