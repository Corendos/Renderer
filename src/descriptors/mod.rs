/*use std::sync::Arc;
use std::marker::PhantomData;

use vulkano::device::Device;
use vulkano::descriptor::descriptor_set::UnsafeDescriptorSetLayout;
use vulkano::descriptor::descriptor::DescriptorDesc;
use vulkano::descriptor::descriptor_set::DescriptorWrite;
use vulkano::descriptor::PipelineLayoutAbstract;
use vulkano::descriptor::descriptor_set::DescriptorPool;
use vulkano::buffer::BufferAccess;

pub struct FrameDescriptorSet<L> {
    pipeline_layout: L,
    set_id: usize,
    layout: Arc<UnsafeDescriptorSetLayout>,
}

impl<L> FrameDescriptorSet<L> {
    pub fn start(layout: L, set_id: usize) -> FrameDescriptorSetBuilder<L>
    where L: PipelineLayoutAbstract {
        assert!(layout.num_sets() > set_id);

        let cap = layout.num_bindings_in_set(set_id).unwrap_or(0);

        FrameDescriptorSetBuilder {
            layout: layout,
            set_id: set_id,
            binding_id: 0,
            writes: Vec::with_capacity(cap),
        }
    }
}

pub struct FrameDescriptorSetBuilder<L> {
    layout: L,
    set_id: usize,
    binding_id: usize,
    writes: Vec<DescriptorWrite>,
}

impl<L> FrameDescriptorSetBuilder<L> where L: PipelineLayoutAbstract {
    pub fn build(self) -> Result<FrameDescriptorSet<L>, (/* TODO Change */)> {
        let mut pool = Device::standard_descriptor_pool(self.layout.device());

        self.build_with_pool(&mut pool)
    }

    pub fn build_with_pool<P>(self, pool: &mut P) -> Result<FrameDescriptorSet<L>, (/*TODO Changes */)>
        where P: ?Sized + DescriptorPool {
            
    }

    pub fn enter_array(
        self)
        -> Result<FrameDescriptorSetBuilderArray<L>, ()> {
        let desc = match self.layout.descriptor(self.set_id, self.binding_id) {
            Some(d) => d,
            None => return Err(PersistentDescriptorSetError::EmptyExpected),
        };

        Ok(FrameDescriptorSetBuilderArray {
               builder: self,
               desc,
               array_element: 0,
           })
    }
}

pub struct FrameDescriptorSetBuilderArray<L> {
    builder: FrameDescriptorSetBuilder<L>,
    array_element: usize,
    desc: DescriptorDesc
}

impl<L> FrameDescriptorSetBuilderArray<L> where L: PipelineLayoutAbstract {
    pub fn leave_array(self) -> Result<FrameDescriptorSetBuilder<L>, FrameDescriptorSetError> {
        if self.desc.array_count > self.array_element as u32 {
            return Err(FrameDescriptorSetError::MissingArrayElements {
                expected: self.desc.array_count,
                obtained: self.array_element as u32,
            });
        }
        self.builder.binding_id += 1;

        Ok(self.builder)
    }

    pub fn add_buffer<T>(mut self, buffer: T) -> Result<FrameDescriptorSetBuilderArray<L>, FrameDescriptorSetError>
        where T: BufferAccess {
        if self.array_element as u32 >= self.desc.array_count {
            return Err(FrameDescriptorSetError::ArrayOutOfBounds);
        }
    }
}*/