use cgmath::{Matrix4, Point3, Vector3};
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};

pub struct CameraCenter {
    pub center_position: Point3<f32>,
    pub yaw: f32,
    pub pitch: f32,
    pub radius: f32,
    pub is_active: bool,
}

impl CameraCenter {
    pub fn new() -> CameraCenter {
        CameraCenter {
            center_position: Point3::new(0.0, 0.0, 0.0),
            yaw: 0.0,
            pitch: FRAC_PI_4,
            radius: 3.0,
            is_active: false,
        }
    }

    pub fn update_yaw(&mut self, delta: f32) {
        self.yaw += delta;
        if self.yaw > 2.0 * PI {
            self.yaw -= 2.0 * PI;
        } else if self.yaw < -2.0 * PI {
            self.yaw += 2.0 * PI;
        }
    }

    pub fn update_pitch(&mut self, delta: f32) {
        self.pitch += delta;
        if self.pitch > FRAC_PI_2 - std::f32::EPSILON {
            self.pitch = FRAC_PI_2 - std::f32::EPSILON;
        } else if self.pitch < -FRAC_PI_2 + std::f32::EPSILON {
            self.pitch = -FRAC_PI_2 + std::f32::EPSILON;
        }
    }

    pub fn update_radius(&mut self, delta: f32) {
        self.radius += delta;
        if self.radius < 1.0 {
            self.radius = 1.0;
        }
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        let eye_position = self.center_position
            + Vector3::<f32>::new(
                self.radius * self.pitch.cos() * self.yaw.cos(),
                self.radius * self.pitch.sin(),
                self.radius * self.pitch.cos() * self.yaw.sin(),
            );
        Matrix4::look_at(
            eye_position,
            self.center_position,
            Vector3::<f32>::new(0.0, -1.0, 0.0),
        )
    }

    pub fn position(&self) -> Point3<f32> {
        self.center_position
            + Vector3::<f32>::new(
                self.radius * self.pitch.cos() * self.yaw.cos(),
                self.radius * self.pitch.sin(),
                self.radius * self.pitch.cos() * self.yaw.sin(),
            )
    }
}
