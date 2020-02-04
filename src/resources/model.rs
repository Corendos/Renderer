use std::sync::Arc;

use super::super::color::Color;
use super::super::resources::shaders;
use super::super::transform::Transform;
use super::super::vertex::{Vertex, VertexBuilder};
use super::super::Renderer;

use vulkano::buffer::{BufferAccess, BufferUsage, CpuAccessibleBuffer, CpuBufferPool};
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::descriptor::descriptor_set::{FixedSizeDescriptorSetsPool, PersistentDescriptorSet, DescriptorSet};
use vulkano::descriptor::pipeline_layout::PipelineLayoutAbstract;
use vulkano::descriptor::descriptor_set::DescriptorSetsCollection;
use vulkano::pipeline::GraphicsPipelineAbstract;

use cgmath::prelude::*;

pub struct Model {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub transform: Transform,
    pub vertex_buffer: Option<Arc<CpuAccessibleBuffer<[Vertex]>>>,
    pub index_buffer: Option<Arc<CpuAccessibleBuffer<[u32]>>>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            transform: Transform::new(),
            vertex_buffer: None,
            index_buffer: None,
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
                .build()
                .unwrap(),
            VertexBuilder::start()
                .with_position(scale, -scale, scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(0.0, -1.0, 0.0)
                .with_specular_exponent(1000.0)
                .build()
                .unwrap(),
            VertexBuilder::start()
                .with_position(scale, -scale, -scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(0.0, -1.0, 0.0)
                .with_specular_exponent(1000.0)
                .build()
                .unwrap(),
            VertexBuilder::start()
                .with_position(-scale, -scale, -scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(0.0, -1.0, 0.0)
                .with_specular_exponent(1000.0)
                .build()
                .unwrap(),
            // Top
            VertexBuilder::start()
                .with_position(-scale, scale, scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(0.0, 1.0, 0.0)
                .with_specular_exponent(1000.0)
                .build()
                .unwrap(),
            VertexBuilder::start()
                .with_position(scale, scale, scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(0.0, 1.0, 0.0)
                .with_specular_exponent(1000.0)
                .build()
                .unwrap(),
            VertexBuilder::start()
                .with_position(scale, scale, -scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(0.0, 1.0, 0.0)
                .with_specular_exponent(1000.0)
                .build()
                .unwrap(),
            VertexBuilder::start()
                .with_position(-scale, scale, -scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(0.0, 1.0, 0.0)
                .with_specular_exponent(1000.0)
                .build()
                .unwrap(),
            // Right
            VertexBuilder::start()
                .with_position(scale, -scale, scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(1.0, 0.0, 0.0)
                .with_specular_exponent(1000.0)
                .build()
                .unwrap(),
            VertexBuilder::start()
                .with_position(scale, scale, scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(1.0, 0.0, 0.0)
                .with_specular_exponent(1000.0)
                .build()
                .unwrap(),
            VertexBuilder::start()
                .with_position(scale, scale, -scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(1.0, 0.0, 0.0)
                .with_specular_exponent(1000.0)
                .build()
                .unwrap(),
            VertexBuilder::start()
                .with_position(scale, -scale, -scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(1.0, 0.0, 0.0)
                .with_specular_exponent(1000.0)
                .build()
                .unwrap(),
            // Left
            VertexBuilder::start()
                .with_position(-scale, -scale, scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(-1.0, 0.0, 0.0)
                .with_specular_exponent(1000.0)
                .build()
                .unwrap(),
            VertexBuilder::start()
                .with_position(-scale, scale, scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(-1.0, 0.0, 0.0)
                .with_specular_exponent(1000.0)
                .build()
                .unwrap(),
            VertexBuilder::start()
                .with_position(-scale, scale, -scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(-1.0, 0.0, 0.0)
                .with_specular_exponent(1000.0)
                .build()
                .unwrap(),
            VertexBuilder::start()
                .with_position(-scale, -scale, -scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(-1.0, 0.0, 0.0)
                .with_specular_exponent(1000.0)
                .build()
                .unwrap(),
            // Front
            VertexBuilder::start()
                .with_position(-scale, -scale, scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(0.0, 0.0, 1.0)
                .with_specular_exponent(1000.0)
                .build()
                .unwrap(),
            VertexBuilder::start()
                .with_position(scale, -scale, scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(0.0, 0.0, 1.0)
                .with_specular_exponent(1000.0)
                .build()
                .unwrap(),
            VertexBuilder::start()
                .with_position(scale, scale, scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(0.0, 0.0, 1.0)
                .with_specular_exponent(1000.0)
                .build()
                .unwrap(),
            VertexBuilder::start()
                .with_position(-scale, scale, scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(0.0, 0.0, 1.0)
                .with_specular_exponent(1000.0)
                .build()
                .unwrap(),
            // Back
            VertexBuilder::start()
                .with_position(-scale, -scale, -scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(0.0, 0.0, -1.0)
                .with_specular_exponent(1000.0)
                .build()
                .unwrap(),
            VertexBuilder::start()
                .with_position(scale, -scale, -scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(0.0, 0.0, -1.0)
                .with_specular_exponent(1000.0)
                .build()
                .unwrap(),
            VertexBuilder::start()
                .with_position(scale, scale, -scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(0.0, 0.0, -1.0)
                .with_specular_exponent(1000.0)
                .build()
                .unwrap(),
            VertexBuilder::start()
                .with_position(-scale, scale, -scale)
                .with_diffuse(color)
                .with_ambient(color * 0.2)
                .with_normal(0.0, 0.0, -1.0)
                .with_specular_exponent(1000.0)
                .build()
                .unwrap(),
        ];

        let indices = vec![
            3, 0, 1, 3, 1, 2, 4, 7, 6, 4, 6, 5, 8, 9, 10, 8, 10, 11, 12, 15, 14, 12, 14, 13, 16,
            19, 18, 16, 18, 17, 20, 21, 22, 20, 22, 23,
        ];

        Self {
            vertices,
            indices,
            transform: Transform::new(),
            vertex_buffer: None,
            index_buffer: None,
        }
    }

    pub fn bake(&mut self, renderer: &Renderer) {
        let vertex_buffer = CpuAccessibleBuffer::from_iter(
            renderer.device.clone(),
            BufferUsage::vertex_buffer(),
            self.vertices.clone().into_iter(),
        )
        .unwrap();

        let index_buffer = CpuAccessibleBuffer::from_iter(
            renderer.device.clone(),
            BufferUsage::index_buffer(),
            self.indices.clone().into_iter(),
        )
        .unwrap();

        self.vertex_buffer = Some(vertex_buffer);
        self.index_buffer = Some(index_buffer);
    }
}

pub trait FromBuffers<V, I> {
    fn from_buffers(v: V, i: I) -> Self;
}

impl FromBuffers<Vec<Vertex>, Vec<u32>> for Model {
    fn from_buffers(vertices: Vec<Vertex>, indices: Vec<u32>) -> Model {
        Model {
            vertices,
            indices,
            transform: Transform::new(),
            vertex_buffer: None,
            index_buffer: None,
        }
    }
}

impl FromBuffers<&Vec<Vertex>, &Vec<u32>> for Model {
    fn from_buffers(vertices: &Vec<Vertex>, indices: &Vec<u32>) -> Model {
        Model {
            vertices: vertices.clone(),
            indices: indices.clone(),
            transform: Transform::new(),
            vertex_buffer: None,
            index_buffer: None,
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
                    .build()
                    .unwrap(),
                VertexBuilder::start()
                    .with_position(size, 0.0, 0.0)
                    .with_diffuse(Color::<f32>::RED)
                    .build()
                    .unwrap(),
                VertexBuilder::start()
                    .with_position(0.0, 0.0, 0.0)
                    .with_diffuse(Color::<f32>::GREEN)
                    .build()
                    .unwrap(),
                VertexBuilder::start()
                    .with_position(0.0, size, 0.0)
                    .with_diffuse(Color::<f32>::GREEN)
                    .build()
                    .unwrap(),
                VertexBuilder::start()
                    .with_position(0.0, 0.0, 0.0)
                    .with_diffuse(Color::<f32>::BLUE)
                    .build()
                    .unwrap(),
                VertexBuilder::start()
                    .with_position(0.0, 0.0, size)
                    .with_diffuse(Color::<f32>::BLUE)
                    .build()
                    .unwrap(),
            ],
        }
    }
}

pub trait Renderable {
    fn render<Gp, D, L>(
        &self,
        command_buffer_builder: AutoCommandBufferBuilder,
        pipeline: Gp,
        world_descriptor_set: Arc<D>,
        model_data_uniform_buffer: &CpuBufferPool<shaders::basic::vertex::ty::ModelData>,
        pool: &mut FixedSizeDescriptorSetsPool<L>,
    ) -> AutoCommandBufferBuilder
    where
        Gp: GraphicsPipelineAbstract + Send + Sync + 'static + Clone,
        D: DescriptorSet + Send + Sync + 'static,
        L: PipelineLayoutAbstract + Send + Sync + Clone + 'static;
    
    fn render_with_sets<Gp, D>(
        &self,
        command_buffer_builder: AutoCommandBufferBuilder,
        pipeline: Gp,
        sets: D
    ) -> AutoCommandBufferBuilder
    where
        Gp: GraphicsPipelineAbstract + Send + Sync + 'static + Clone,
        D: DescriptorSetsCollection;
}

impl Renderable for Model {
    fn render<Gp, D, L>(
        &self,
        command_buffer_builder: AutoCommandBufferBuilder,
        pipeline: Gp,
        world_descriptor_set: Arc<D>,
        model_data_uniform_buffer: &CpuBufferPool<shaders::basic::vertex::ty::ModelData>,
        pool: &mut FixedSizeDescriptorSetsPool<L>,
    ) -> AutoCommandBufferBuilder
    where
        Gp: GraphicsPipelineAbstract + Send + Sync + 'static + Clone,
        D: DescriptorSet + Send + Sync + 'static,
        L: PipelineLayoutAbstract + Send + Sync + Clone + 'static,
    {
        assert_eq!(self.vertex_buffer.is_some(), true);
        assert_eq!(self.index_buffer.is_some(), true);

        let model_data_subbuffer = {
            let normal_matrix = self.transform.model_matrix().invert().unwrap().transpose();

            let uniform_data = shaders::basic::vertex::ty::ModelData {
                model_matrix: self.transform.model_matrix().into(),
                normal_matrix: normal_matrix.into(),
            };

            model_data_uniform_buffer.next(uniform_data).unwrap()
        };

        let set = Arc::new(
            pool.next()
                .add_buffer(model_data_subbuffer)
                .unwrap()
                .build()
                .unwrap(),
        );

        self.render_with_sets(command_buffer_builder, pipeline, (world_descriptor_set.clone(), set.clone()))
    }

    fn render_with_sets<Gp, D>(
        &self,
        command_buffer_builder: AutoCommandBufferBuilder,
        pipeline: Gp,
        sets: D
    ) -> AutoCommandBufferBuilder
    where
        Gp: GraphicsPipelineAbstract + Send + Sync + 'static + Clone,
        D: DescriptorSetsCollection,
    {
        assert_eq!(self.vertex_buffer.is_some(), true);
        assert_eq!(self.index_buffer.is_some(), true);
        command_buffer_builder
            .draw_indexed(
                pipeline,
                &DynamicState::none(),
                vec![self.vertex_buffer.as_ref().unwrap().clone()],
                self.index_buffer.as_ref().unwrap().clone(),
                sets,
                (),
            )
            .unwrap()
    }
}
