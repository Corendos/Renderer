pub mod device_infos;
pub mod shaders;
pub mod metrics;
pub mod resources;


#[derive(Default, Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3]
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { position: [x, y, z], color: [1.0, 1.0, 1.0] }
    }

    pub fn new_with_color(x: f32, y: f32, z: f32, r: f32, g: f32, b: f32) -> Self {
        Self { position: [x, y, z], color: [r, g, b] }
    }
}

vulkano::impl_vertex!(Vertex, position, color);