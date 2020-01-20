use super::super::Vertex;

pub struct Model {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>
}

impl Model {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn cube(scale: f32) -> Self {
        let vertices = vec![
            // Bottom
            Vertex { position: [-scale, -scale, scale], color: [1.0, 0.0, 0.0]},
            Vertex { position: [scale, -scale, scale], color: [1.0, 0.0, 0.0]},
            Vertex { position: [scale, -scale, -scale], color: [1.0, 0.0, 0.0]},
            Vertex { position: [-scale, -scale, -scale], color: [1.0, 0.0, 0.0]},

            // Top
            Vertex { position: [-scale, scale, scale], color: [0.0, 1.0, 0.0]},
            Vertex { position: [scale, scale, scale], color: [0.0, 1.0, 0.0]},
            Vertex { position: [scale, scale, -scale], color: [0.0, 1.0, 0.0]},
            Vertex { position: [-scale, scale, -scale], color: [0.0, 1.0, 0.0]},

            // Right
            Vertex { position: [scale, -scale, scale], color: [0.0, 0.0, 1.0]},
            Vertex { position: [scale, scale, scale], color: [0.0, 0.0, 1.0]},
            Vertex { position: [scale, scale, -scale], color: [0.0, 0.0, 1.0]},
            Vertex { position: [scale, -scale, -scale], color: [0.0, 0.0, 1.0]},

            // Left
            Vertex { position: [-scale, -scale, scale], color: [0.0, 1.0, 1.0]},
            Vertex { position: [-scale, scale, scale], color: [0.0, 1.0, 1.0]},
            Vertex { position: [-scale, scale, -scale], color: [0.0, 1.0, 1.0]},
            Vertex { position: [-scale, -scale, -scale], color: [0.0, 1.0, 1.0]},

            // Front
            Vertex { position: [-scale, -scale, scale], color: [1.0, 0.0, 1.0]},
            Vertex { position: [scale, -scale, scale], color: [1.0, 0.0, 1.0]},
            Vertex { position: [scale, scale, scale], color: [1.0, 0.0, 1.0]},
            Vertex { position: [-scale, scale, scale], color: [1.0, 0.0, 1.0]},

            // Back
            Vertex { position: [-scale, -scale, -scale], color: [1.0, 1.0, 0.0]},
            Vertex { position: [scale, -scale, -scale], color: [1.0, 1.0, 0.0]},
            Vertex { position: [scale, scale, -scale], color: [1.0, 1.0, 0.0]},
            Vertex { position: [-scale, scale, -scale], color: [1.0, 1.0, 0.0]},
        ];

        let indices = vec![
            3, 0, 1, 3, 1, 2,
            4, 7, 6, 4, 6, 5,
            8, 9, 10, 8, 10, 11,
            12, 15, 14, 12, 14, 13,
            16, 19, 18, 16, 18, 17,
            20, 21, 22, 20, 22, 23
        ];

        Self {
            vertices,
            indices
        }
    }
}

pub trait FromBuffers<V, I> {
    fn from_buffers(v: V, i: I) -> Self;
}

impl FromBuffers<Vec<Vertex>, Vec<u32>> for Model {
    fn from_buffers(vertices: Vec<Vertex>, indices: Vec<u32>) -> Model {
        Model {
            vertices,
            indices
        }
    }
}

impl FromBuffers<&Vec<Vertex>, &Vec<u32>> for Model {
    fn from_buffers(vertices: &Vec<Vertex>, indices: &Vec<u32>) -> Model {
        Model {
            vertices: vertices.clone(),
            indices: indices.clone()
        }
    }
}