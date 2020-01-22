use super::super::vertex::{Vertex, VertexBuilder};
use super::super::color::Color;

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

    pub fn cube(scale: f32, color: Color<f32>) -> Self {
        let vertices = vec![
            // Bottom
            VertexBuilder::start()
                .with_position(-scale, -scale, scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(0.0, -1.0, 0.0)
                .with_specular_exponent(1000.0)
                .build().unwrap(),
            VertexBuilder::start()
                .with_position(scale, -scale, scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(0.0, -1.0, 0.0)
                .with_specular_exponent(1000.0)
                .build().unwrap(),
            VertexBuilder::start()
                .with_position(scale, -scale, -scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(0.0, -1.0, 0.0)
                .with_specular_exponent(1000.0)
                .build().unwrap(),
            VertexBuilder::start()
                .with_position(-scale, -scale, -scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(0.0, -1.0, 0.0)
                .with_specular_exponent(1000.0)
                .build().unwrap(),

            // Top
            VertexBuilder::start()
                .with_position(-scale, scale, scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(0.0, 1.0, 0.0)
                .with_specular_exponent(1000.0)
                .build().unwrap(),
            VertexBuilder::start()
                .with_position(scale, scale, scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(0.0, 1.0, 0.0)
                .with_specular_exponent(1000.0)
                .build().unwrap(),
            VertexBuilder::start()
                .with_position(scale, scale, -scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(0.0, 1.0, 0.0)
                .with_specular_exponent(1000.0)
                .build().unwrap(),
            VertexBuilder::start()
                .with_position(-scale, scale, -scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(0.0, 1.0, 0.0)
                .with_specular_exponent(1000.0)
                .build().unwrap(),

            // Right
            VertexBuilder::start()
                .with_position(scale, -scale, scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(1.0, 0.0, 0.0)
                .with_specular_exponent(1000.0)
                .build().unwrap(),
            VertexBuilder::start()
                .with_position(scale, scale, scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(1.0, 0.0, 0.0)
                .with_specular_exponent(1000.0)
                .build().unwrap(),
            VertexBuilder::start()
                .with_position(scale, scale, -scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(1.0, 0.0, 0.0)
                .with_specular_exponent(1000.0)
                .build().unwrap(),
            VertexBuilder::start()
                .with_position(scale, -scale, -scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(1.0, 0.0, 0.0)
                .with_specular_exponent(1000.0)
                .build().unwrap(),

            // Left
            VertexBuilder::start()
                .with_position(-scale, -scale, scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(-1.0, 0.0, 0.0)
                .with_specular_exponent(1000.0)
                .build().unwrap(),
            VertexBuilder::start()
                .with_position(-scale, scale, scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(-1.0, 0.0, 0.0)
                .with_specular_exponent(1000.0)
                .build().unwrap(),
            VertexBuilder::start()
                .with_position(-scale, scale, -scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(-1.0, 0.0, 0.0)
                .with_specular_exponent(1000.0)
                .build().unwrap(),
            VertexBuilder::start()
                .with_position(-scale, -scale, -scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(-1.0, 0.0, 0.0)
                .with_specular_exponent(1000.0)
                .build().unwrap(),

            // Front
            VertexBuilder::start()
                .with_position(-scale, -scale, scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(0.0, 0.0, 1.0)
                .with_specular_exponent(1000.0)
                .build().unwrap(),
            VertexBuilder::start()
                .with_position(scale, -scale, scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(0.0, 0.0, 1.0)
                .with_specular_exponent(1000.0)
                .build().unwrap(),
            VertexBuilder::start()
                .with_position(scale, scale, scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(0.0, 0.0, 1.0)
                .with_specular_exponent(1000.0)
                .build().unwrap(),
            VertexBuilder::start()
                .with_position(-scale, scale, scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(0.0, 0.0, 1.0)
                .with_specular_exponent(1000.0)
                .build().unwrap(),

            // Back
            VertexBuilder::start()
                .with_position(-scale, -scale, -scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(0.0, 0.0, -1.0)
                .with_specular_exponent(1000.0)
                .build().unwrap(),
            VertexBuilder::start()
                .with_position(scale, -scale, -scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(0.0, 0.0, -1.0)
                .with_specular_exponent(1000.0)
                .build().unwrap(),
            VertexBuilder::start()
                .with_position(scale, scale, -scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(0.0, 0.0, -1.0)
                .with_specular_exponent(1000.0)
                .build().unwrap(),
            VertexBuilder::start()
                .with_position(-scale, scale, -scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(0.0, 0.0, -1.0)
                .with_specular_exponent(1000.0)
                .build().unwrap(),
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

pub struct Gizmo {
    pub vertices: Vec<Vertex>,
}

impl Gizmo {
    pub fn new(size: f32) -> Self {
        Self {
            vertices: vec![
                VertexBuilder::start()
                    .with_position(0.0, 0.0, 0.0)
                    .with_diffuse(Color::<f32>::RED)
                    .build().unwrap(),
                VertexBuilder::start()
                    .with_position(size, 0.0, 0.0)
                    .with_diffuse(Color::<f32>::RED)
                    .build().unwrap(),

                VertexBuilder::start()
                    .with_position(0.0, 0.0, 0.0)
                    .with_diffuse(Color::<f32>::GREEN)
                    .build().unwrap(),
                VertexBuilder::start()
                    .with_position(0.0, size, 0.0)
                    .with_diffuse(Color::<f32>::GREEN)
                    .build().unwrap(),

                VertexBuilder::start()
                    .with_position(0.0, 0.0, 0.0)
                    .with_diffuse(Color::<f32>::BLUE)
                    .build().unwrap(),
                VertexBuilder::start()
                    .with_position(0.0, 0.0, size)
                    .with_diffuse(Color::<f32>::BLUE)
                    .build().unwrap(),
            ]
        }
    }
}