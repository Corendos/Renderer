pub mod device_infos;
pub mod shaders;
pub mod metrics;
pub mod resources;
pub mod vertex;
pub mod color;
pub mod camera;
pub mod input;

use input::Input;

use cgmath::{Matrix4, SquareMatrix, Rad};

pub struct ApplicationState {
    pub is_running: bool,
    pub dimensions: [f32; 2],
    pub aspect_ratio: f32,
    pub projection: Matrix4<f32>,
    pub need_recreation: bool,
    pub input: Input
}

impl ApplicationState {
    pub fn new() -> ApplicationState {
        ApplicationState {
            is_running: true,
            dimensions: [0.0, 0.0],
            aspect_ratio: 0.0,
            projection: SquareMatrix::identity(),
            need_recreation: false,
            input: Input::new()
        }
    }

    pub fn set_dimensions(&mut self, width: f32, height: f32) {
        self.dimensions = [width, height];
        self.aspect_ratio = width / height;
        self.projection = cgmath::perspective(Rad(std::f32::consts::FRAC_PI_2), self.aspect_ratio, 0.01, 100.0);
    }
}