use cgmath::prelude::*;
use cgmath::{Matrix4, Quaternion, Rad, Rotation3, Vector3};

pub struct Transform {
    pub position: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub rotation: Quaternion<f32>,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            position: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
            rotation: Quaternion::new(1.0, 0.0, 0.0, 0.0),
        }
    }

    pub fn translate(&mut self, delta: Vector3<f32>) {
        self.position += delta;
    }

    pub fn scale(&mut self, delta: Vector3<f32>) {
        self.scale.x *= delta.x;
        self.scale.y *= delta.y;
        self.scale.z *= delta.z;
    }

    pub fn rotate(&mut self, axis: Vector3<f32>, angle: Rad<f32>) {
        let half_angle = angle / 2.0;
        let rotation = Quaternion::from_sv(half_angle.cos(), half_angle.sin() * axis.normalize());
        self.rotation = rotation * self.rotation;
    }

    pub fn model_matrix(&self) -> Matrix4<f32> {
        let rotation_matrix: Matrix4<f32> = self.rotation.into();
        let scale_matrix = Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);
        let translation_matrix = Matrix4::from_translation(self.position);

        translation_matrix * scale_matrix * rotation_matrix
    }
}
