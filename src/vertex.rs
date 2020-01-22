use super::color::Color;

#[derive(Default, Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub ambient: [f32; 3],
    pub diffuse: [f32; 3],
    pub specular_exponent: f32
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: [x, y, z],
            normal: [0.0, 0.0, 0.0],
            ambient: [0.0, 0.0, 0.0],
            diffuse: [1.0, 1.0, 1.0],
            specular_exponent: 1.0
        }
    }

    pub fn new_with_color(x: f32, y: f32, z: f32, r: f32, g: f32, b: f32) -> Self {
        Self {
            position: [x, y, z],
            normal: [0.0, 0.0, 0.0],
            ambient: [0.0, 0.0, 0.0],
            diffuse: [r, g, b],
            specular_exponent: 1.0,
        }
    }
}

pub struct VertexBuilder {
    pub position: Option<[f32; 3]>,
    pub normal: Option<[f32; 3]>,
    pub ambient: Option<[f32; 3]>,
    pub diffuse: Option<[f32; 3]>,
    pub specular_exponent: Option<f32>
}

impl VertexBuilder {
    pub fn start() -> Self {
        Self {
            position: None,
            normal: None,
            ambient: None,
            diffuse: None,
            specular_exponent: None,
        }
    }

    pub fn with_position(mut self, x: f32, y: f32, z: f32) -> Self {
        self.position = Some([x, y, z]);
        self
    }

    pub fn with_normal(mut self, x: f32, y: f32, z: f32) -> Self {
        self.normal = Some([x, y, z]);
        self

    }

    pub fn with_ambient<T: Into<f32>>(mut self, color: Color<T>) -> Self {
        self.ambient = Some([
            color.r.into(),
            color.g.into(),
            color.b.into()
        ]);
        self

    }

    pub fn with_diffuse<T: Into<f32>>(mut self, color: Color<T>) -> Self {
        self.diffuse = Some([
            color.r.into(),
            color.g.into(),
            color.b.into()
        ]);
        self

    }

    pub fn with_specular_exponent(mut self, e: f32) -> Self {
        self.specular_exponent = Some(e);
        self

    }

    pub fn build(self) -> Result<Vertex, VertexBuildError> {
        if !self.is_valid() {
            Err(VertexBuildError)
        } else {
            Ok(Vertex {
                position: self.position.unwrap(),
                normal: self.normal.unwrap_or([0.0, 0.0, 0.0]),
                ambient: self.ambient.unwrap_or([0.0, 0.0, 0.0]),
                diffuse: self.diffuse.unwrap(),
                specular_exponent: self.specular_exponent.unwrap_or(1.0),
            })
        }
    }

    pub fn is_valid(&self) -> bool {
        self.position.is_some() && self.diffuse.is_some()
    }
}

#[derive(Debug)]
pub struct VertexBuildError;

vulkano::impl_vertex!(Vertex, position, normal, ambient, diffuse, specular_exponent);

#[derive(Default, Copy, Clone)]
pub struct SimpleVertex {
    pub position: [f32; 3],
    pub color: [f32; 3]
}

impl SimpleVertex {
    pub fn new<T: Into<f32>>(x: f32, y: f32, z: f32, color: Color<T>) -> SimpleVertex {
        SimpleVertex {
            position: [x, y, z],
            color: [
                color.r.into(),
                color.g.into(),
                color.b.into(),
            ]
        }
    }
}

vulkano::impl_vertex!(SimpleVertex, position, color);