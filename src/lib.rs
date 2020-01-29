pub mod device_infos;
pub mod metrics;
pub mod resources;
pub mod vertex;
pub mod color;
pub mod camera;
pub mod input;

use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;

use serde::Deserialize;
use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::device::{Device, Queue};
use vulkano::format::Format;
use cgmath::{Matrix4, SquareMatrix, Rad};

use input::Input;
use device_infos::print_infos;


pub struct ApplicationState {
    pub is_running: bool,
    pub dimensions: [f32; 2],
    pub aspect_ratio: f32,
    pub projection: Matrix4<f32>,
    pub need_recreation: bool
}

impl ApplicationState {
    pub fn new() -> ApplicationState {
        ApplicationState {
            is_running: true,
            dimensions: [0.0, 0.0],
            aspect_ratio: 0.0,
            projection: SquareMatrix::identity(),
            need_recreation: false
        }
    }

    pub fn set_dimensions(&mut self, width: f32, height: f32) {
        self.dimensions = [width, height];
        self.aspect_ratio = width / height;
        self.projection = cgmath::perspective(Rad(std::f32::consts::FRAC_PI_2), self.aspect_ratio, 0.01, 100.0);
    }
}

#[derive(Deserialize)]
#[serde(remote = "Format")]
pub enum FormatDef{
    B8G8R8A8Srgb,
    B8G8R8A8Unorm
}

impl From<FormatDef> for Format {
    fn from(def: FormatDef) -> Format {
        match def {
            FormatDef::B8G8R8A8Srgb => Format::B8G8R8A8Srgb,
            FormatDef::B8G8R8A8Unorm => Format::B8G8R8A8Unorm,
        }
    }
}


#[derive(Debug, Deserialize)]
pub struct RendererConfig {
    pub fps: Option<f32>,
    pub width: f32,
    pub height: f32,
    pub line_width: f32,
    pub clear_color: [f32; 3],
    #[serde(with = "FormatDef")]
    pub format: vulkano::format::Format,
}

impl RendererConfig {
    pub fn load_from_file<P: AsRef<Path>>(p: P) -> Self {
        // TODO: Fix this
        let mut config_file = match File::open(p) {
            Ok(f) => f,
            Err(e) => panic!(format!("{:?}", e.kind()))
        };

        let mut config_string = String::new();
        config_file.read_to_string(&mut config_string).unwrap();
        let config: Self = toml::from_str(config_string.as_str()).unwrap();
        config
    }
}

pub struct Renderer {
    pub config: RendererConfig,
    pub device: Arc<Device>,
    pub graphics_queue: Arc<Queue>,
    pub transfer_queue: Arc<Queue>,
    pub state: ApplicationState,
    pub input: Input,
}

impl Renderer {
    pub fn create(config: RendererConfig) -> Self {
        let instance = {
            let extensions = vulkano_win::required_extensions();
            Instance::new(None, &extensions, None).expect("Failed to create instance")
        };

        let physical_device = PhysicalDevice::enumerate(&instance).next().expect("No physical device found");
        println!("Found device:");
        print_infos(&physical_device);

        let (device, graphics_queue, transfer_queue) = Self::create_device_and_queue(physical_device);

        Self {
            config,
            device,
            graphics_queue,
            transfer_queue,
            state: ApplicationState::new(),
            input: Input::new()
        }
    }

    fn create_device_and_queue(physical_device: PhysicalDevice) -> (Arc<Device>, Arc<Queue>, Arc<Queue>) {
        let graphics_queue_family = physical_device.queue_families()
            .find(|&q| { q.supports_graphics() })
            .expect("Couldn't find a graphical queue family");

        let transfer_queue_family = physical_device.queue_families()
            .find(|&q| { q.explicitly_supports_transfers() })
            .unwrap_or(graphics_queue_family);

        let (device, mut queues) = {
            let device_ext = vulkano::device::DeviceExtensions {
                khr_swapchain: true,
                .. vulkano::device::DeviceExtensions::none()
            };

            Device::new(
                physical_device,
                physical_device.supported_features(),
                &device_ext,
                [(graphics_queue_family, 0.5), (transfer_queue_family, 0.5)].iter().cloned())
                .expect("Failed to create device")
        };

        let graphics_queue = queues.next().unwrap();
        let transfer_queue = queues.next().unwrap();

        (device, graphics_queue, transfer_queue)
    }
}

// TODO: Go back to this later

/*
use vulkano::command_buffer::{synced, sys, CommandBuffer};
use vulkano::framebuffer::FramebufferAbstract;
use vulkano::pipeline::GraphicsPipelineAbstract;
use vulkano::buffer::device_local::DeviceLocalBuffer;
use vulkano::descriptor::DescriptorSet;
use vulkano::buffer::TypedBufferAccess;
use vulkano::command_buffer::pool::standard::StandardCommandPoolAlloc;

pub struct CustomCommandBuffer {
    pub inner: vulkano::command_buffer::synced::SyncCommandBuffer<StandardCommandPoolAlloc>
}

impl CustomCommandBuffer {
    fn new<F, Gp, S>(renderer: &Renderer, framebuffers: Vec<Arc<F>>, image_num: usize,
                     gizmo_pipeline: Arc<Gp>, gizmo_vertex_buffer: Arc<DeviceLocalBuffer<[vertex::Vertex]>>,
                     set: Arc<S>) -> CustomCommandBuffer
    where F: FramebufferAbstract + Send + Sync + 'static, Gp: GraphicsPipelineAbstract + Send + Sync + 'static,
    S: DescriptorSet + Send + Sync + 'static {
        let standard_pool = Device::standard_command_pool(&renderer.device, renderer.graphics_queue.family());
        
        let temp = unsafe {
            let mut command = synced::SyncCommandBufferBuilder::new(&standard_pool, sys::Kind::primary(), sys::Flags::OneTimeSubmit).unwrap();
            command.begin_render_pass(
                    framebuffers[image_num].clone(),
                    vulkano::framebuffer::SubpassContents::Inline,
                    vec![
                        renderer.config.clear_color.into(),
                        1f32.into()
                    ].into_iter()).unwrap();
            command.bind_pipeline_graphics(gizmo_pipeline.clone());

            let mut vertex_buffer_bind_builder = command.bind_vertex_buffers();
            vertex_buffer_bind_builder.add(gizmo_vertex_buffer.clone());
            vertex_buffer_bind_builder.submit(0).unwrap();

            let mut set_bind_builder = command.bind_descriptor_sets();
            set_bind_builder.add(set.clone());
            set_bind_builder.submit(true, gizmo_pipeline.clone(), 0, Vec::<u32>::new().into_iter()).unwrap();

            command.draw(gizmo_vertex_buffer.len() as u32, 1, 0, 0);

            command.end_render_pass();
            command.build()
        }.unwrap();

        CustomCommandBuffer {
            inner: temp
        }
    }
}

unsafe impl CommandBuffer for CustomCommandBuffer {
    type PoolAlloc = StandardCommandPoolAlloc;
}*/