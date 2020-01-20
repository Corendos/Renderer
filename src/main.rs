use std::sync::Arc;

use renderer::device_infos::print_infos;
use renderer::shaders::{basic_vertex_shader, basic_fragment_shader};
use renderer::metrics::FPSCounter;
use renderer::resources::loader::obj::ObjLoader;
use renderer::Vertex;

use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::device::{Device, Queue};
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage, cpu_pool::CpuBufferPool};
use vulkano::format::Format;
use vulkano::framebuffer::{Framebuffer, FramebufferAbstract, RenderPassAbstract, Subpass};
use vulkano::command_buffer::{DynamicState, AutoCommandBufferBuilder, CommandBuffer};
use vulkano::pipeline::{GraphicsPipeline, GraphicsPipelineAbstract, viewport::Viewport};
use vulkano::sync::GpuFuture;
use vulkano::swapchain::{Swapchain, SurfaceTransform, PresentMode, Surface};
use vulkano::image::attachment::AttachmentImage;
use vulkano::image::swapchain::SwapchainImage;

use vulkano_win::VkSurfaceBuild;
use winit::{Window, WindowBuilder, EventsLoop, Event, WindowEvent};
use winit::dpi::LogicalSize;

use cgmath::{Matrix4, Point3, Vector3, Rad};
use cgmath::prelude::*;


const MODEL_PATH: &str = "/home/corendos/dev/rust/renderer/res/models/LEGO_NabooShip.obj";

fn create_device() -> (Arc<Device>, Arc<Queue>) {
    let instance = {
        let extensions = vulkano_win::required_extensions();
        Instance::new(None, &extensions, None).expect("Failed to create instance")
    };

    let physical_device = PhysicalDevice::enumerate(&instance).next().expect("No physical device found");
    println!("Found device:");
    print_infos(&physical_device);

    let queue_family = physical_device.queue_families()
        .find(|&q| { q.supports_graphics() })
        .expect("Couldn't find a graphical queue family");

    let (device, mut queues) = {
        let device_ext = vulkano::device::DeviceExtensions {
            khr_swapchain: true,
            .. vulkano::device::DeviceExtensions::none()
        };

        Device::new(
            physical_device,
            physical_device.supported_features(),
            &device_ext,
            [(queue_family, 0.5)].iter().cloned())
            .expect("Failed to create device")
    };

    let queue = queues.next().unwrap();


    (device, queue)
}

fn create_swapchain(device: Arc<Device>,
                    surface: Arc<Surface<Window>>,
                    queue: Arc<Queue>,
                    old_swapchain: Option<&Arc<Swapchain<Window>>>)
        -> (Arc<Swapchain<Window>>, [u32; 2], Vec<Arc<SwapchainImage<Window>>>) {
    let caps = surface.capabilities(device.physical_device()).expect("Failed to get device capabilities");

    let dimensions = caps.current_extent.unwrap();
    let alpha = caps.supported_composite_alpha.iter().next().unwrap();
    let format = caps.supported_formats[0].0;

    let (swapchain, images) = Swapchain::new(device, surface,
        caps.min_image_count, format, dimensions, 1, caps.supported_usage_flags, &queue,
        SurfaceTransform::Identity, alpha, PresentMode::Fifo, true, old_swapchain).expect("Failed to create swapchain");
    
    (swapchain, dimensions, images)
}

fn create_framebuffers(device: Arc<Device>,
                       images: &Vec<Arc<SwapchainImage<Window>>>,
                       render_pass: Arc<dyn RenderPassAbstract + Send + Sync>) -> Vec<Arc<dyn FramebufferAbstract + Send + Sync>> {
    let dimensions = images[0].dimensions();

    let depth_buffer = AttachmentImage::transient(device.clone(), dimensions, Format::D16Unorm).unwrap();
    
    images.iter().map(|image| {
        Arc::new(
            Framebuffer::start(render_pass.clone())
                .add(image.clone()).unwrap()
                .add(depth_buffer.clone()).unwrap()
                .build().unwrap()) as Arc<dyn FramebufferAbstract + Send + Sync>
    }).collect()
}

fn create_pipeline(vs: &basic_vertex_shader::Shader, fs: &basic_fragment_shader::Shader, dimensions: &[u32; 2],
                   render_pass: Arc<dyn RenderPassAbstract + Send + Sync>, device: Arc<Device>)
                    -> Arc<dyn GraphicsPipelineAbstract + Send + Sync> {
    Arc::new(GraphicsPipeline::start()
        .vertex_input_single_buffer::<Vertex>()
        .vertex_shader(vs.main_entry_point(), ())
        .triangle_list()
        .viewports_dynamic_scissors_irrelevant(1)
        .viewports(std::iter::once(Viewport {
            origin: [0.0, 0.0],
            dimensions: [dimensions[0] as f32, dimensions[1] as f32],
            depth_range: 0.0..1.0,
        }))
        .fragment_shader(fs.main_entry_point(), ())
        .depth_stencil_simple_depth()
        .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
        .cull_mode_back()
        .build(device.clone())
        .unwrap())
}

fn main() {
    let (device, queue) = create_device();

    let mut events_loop = EventsLoop::new();
    let surface = WindowBuilder::new()
        .with_title("Vulkan boilerplate")        
        .build_vk_surface(&events_loop, device.instance().clone()).unwrap();
    
    let (mut swapchain, mut dimensions, mut images) = create_swapchain(device.clone(), surface.clone(), queue.clone(), None);

    let basic_vertex_shader = basic_vertex_shader::Shader::load(device.clone()).expect("Failed to create vertex shader");
    let basic_fragment_shader = basic_fragment_shader::Shader::load(device.clone()).expect("Failed to create fragment shader");
    
    // let model = ObjLoader::load(std::path::Path::new(MODEL_PATH)).unwrap();
    let model = renderer::resources::Model::cube(1.0);

    let vertex_buffer = CpuAccessibleBuffer::from_iter(
        device.clone(),
        BufferUsage::all(),
        model.vertices.into_iter()).unwrap();

    let index_buffer = CpuAccessibleBuffer::from_iter(
        device.clone(),
        BufferUsage::all(),
        model.indices.into_iter()).unwrap();
    
    let uniform_buffer = CpuBufferPool::<basic_vertex_shader::ty::Data>::new(device.clone(), BufferUsage::all());

    
    let render_pass = Arc::new(vulkano::single_pass_renderpass!(device.clone(),
        attachments: {
            color: {
                load: Clear,
                store: Store,
                format: Format::B8G8R8A8Unorm,
                samples: 1,
            },
            depth: {
                load: Clear,
                store: DontCare,
                format: Format::D16Unorm,
                samples: 1,
            }
        },
        pass: {
            color: [color],
            depth_stencil: {depth}
        }
    ).unwrap());

    let mut pipeline = create_pipeline(&basic_vertex_shader, &basic_fragment_shader, &dimensions, render_pass.clone(), device.clone());

    let mut framebuffers = create_framebuffers(device.clone(), &images, render_pass.clone());

    let mut is_running = true;

    let mut fps_counter = FPSCounter::new();

    let mut start = std::time::Instant::now();

    while is_running {
        let elapsed = (std::time::Instant::now() - start).as_secs_f32();

        events_loop.poll_events(|event| {
            match event {
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => is_running = false,
                Event::WindowEvent { event: WindowEvent::Resized(LogicalSize { width, height }), .. } => {

                    dimensions = [width as u32, height as u32];
                    let s = swapchain.recreate_with_dimension(dimensions).expect("Failed to recreate swapchain");
                    
                    swapchain = s.0;
                    images = s.1;

                    pipeline = create_pipeline(&basic_vertex_shader, &basic_fragment_shader, &dimensions, render_pass.clone(), device.clone());

                    framebuffers = create_framebuffers(device.clone(), &images, render_pass.clone());
                },
                _ => {},
            }
        });

        let uniform_subbuffer = {
            let aspect_ratio = dimensions[0] as f32 / dimensions [1] as f32;
            let proj = cgmath::perspective(Rad(std::f32::consts::FRAC_PI_2), aspect_ratio, 0.01, 100.0);
            let view = Matrix4::look_at(Point3::new(3.0, 3.0, 3.0), Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, -1.0, 0.0));
            let rotate = Matrix4::from_angle_x(Rad(3.0 * elapsed)) * Matrix4::from_angle_y(Rad(2.5 * elapsed)) * Matrix4::from_angle_z(Rad(2.0 * elapsed));

            let uniform_data = basic_vertex_shader::ty::Data {
                world: (rotate * Matrix4::from_scale(1.0)).into(),
                view: view.into(),
                proj: proj.into()
            };

            uniform_buffer.next(uniform_data).unwrap()
        };

        let set = Arc::new(PersistentDescriptorSet::start(pipeline.clone(), 0)
            .add_buffer(uniform_subbuffer).unwrap()
            .build().unwrap());

        let (image_num, acquire_future) = match vulkano::swapchain::acquire_next_image(swapchain.clone(), None) {
            Ok(r) => r,
            Err(vulkano::swapchain::AcquireError::OutOfDate) => {
                continue;
            },
            Err(err) => panic!("{:?}", err)
        };

        let command_buffer = AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family()).unwrap()
            .begin_render_pass(
                framebuffers[image_num].clone(), false,
                vec![
                    [0.3 * (elapsed / 7.0 * 3.0).sin() + 0.3, 0.3 * elapsed.sin() + 0.3, 1.0, 1.0].into(),
                    1f32.into()
                ]
            ).unwrap()
            .draw_indexed(
                pipeline.clone(),
                &DynamicState::none(),
                vec![vertex_buffer.clone()],
                index_buffer.clone(), set.clone(), ()).unwrap()
            .end_render_pass().unwrap()
            .build().unwrap();
        
        match acquire_future.then_execute(queue.clone(), command_buffer) {
            Ok(buffer_execute_future) => {
                match buffer_execute_future
                    .then_swapchain_present(queue.clone(), swapchain.clone(), image_num)
                    .then_signal_fence_and_flush() {
                        Ok(_) => {},
                        Err(_) => {}
                    }
            },
            Err(_) => {}
        };

        if let Some(mean_frame_duration) = fps_counter.update() {
            surface.window().set_title(&format!("{} FPS", 1.0 / mean_frame_duration));
        }
    }
}