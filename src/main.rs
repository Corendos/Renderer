use std::sync::Arc;
use std::time::Duration;

use renderer::shaders;
use renderer::metrics::FPSCounter;
use renderer::vertex::Vertex;
use renderer::color::Color;
use renderer::resources::model::Gizmo;
use renderer::camera::CameraCenter;
use renderer::{ApplicationState, RendererConfig, Renderer};
use renderer::input::Input;

use vulkano::device::{Device, Queue};
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::buffer::{CpuAccessibleBuffer, TypedBufferAccess, DeviceLocalBuffer, BufferUsage, cpu_pool::CpuBufferPool};
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
use winit::dpi::{LogicalSize, LogicalPosition};

use cgmath::{Matrix4, Vector3};
use cgmath::prelude::*;

fn create_swapchain(device: Arc<Device>,
                    surface: Arc<Surface<Window>>,
                    queue: Arc<Queue>,
                    state: &mut ApplicationState,
                    config: &RendererConfig,
                    old_swapchain: Option<&Arc<Swapchain<Window>>>)
        -> (Arc<Swapchain<Window>>, Vec<Arc<SwapchainImage<Window>>>) {
    let caps = surface.capabilities(device.physical_device()).expect("Failed to get device capabilities");

    let [width, height] = caps.current_extent.unwrap();
    let alpha = caps.supported_composite_alpha.iter().next().unwrap();

    let format = caps.supported_formats.into_iter().find(|format| {
        format.0 == config.format
    }).expect("Failed to find the wanted format").0;

    state.set_dimensions(width as f32, height as f32);

    let (swapchain, images) = Swapchain::new(device, surface,
        caps.min_image_count, format, [width, height], 1, caps.supported_usage_flags, &queue,
        SurfaceTransform::Identity, alpha, PresentMode::Fifo, true, old_swapchain).expect("Failed to create swapchain");
    
    (swapchain, images)
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

fn create_pipeline(vs: &shaders::basic::vertex::Shader, fs: &shaders::basic::fragment::Shader, state: &ApplicationState,
                   render_pass: Arc<dyn RenderPassAbstract + Send + Sync>, device: Arc<Device>)
                    -> Arc<dyn GraphicsPipelineAbstract + Send + Sync> {
    Arc::new(GraphicsPipeline::start()
        .vertex_input_single_buffer::<Vertex>()
        .vertex_shader(vs.main_entry_point(), ())
        .triangle_list()
        .viewports_dynamic_scissors_irrelevant(1)
        .viewports(std::iter::once(Viewport {
            origin: [0.0, 0.0],
            dimensions: state.dimensions,
            depth_range: 0.0..1.0,
        }))
        .fragment_shader(fs.main_entry_point(), ())
        .depth_stencil_simple_depth()
        .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
        .cull_mode_back()
        .build(device.clone())
        .unwrap())
}

fn create_renderpass() {

}

fn main() {
    let config = RendererConfig::load_from_file("/home/corendos/dev/rust/renderer/renderer.toml");
    let mut renderer = Renderer::create(config);

    let mut events_loop = EventsLoop::new();
    let surface = WindowBuilder::new()
        .with_dimensions(LogicalSize::from((renderer.config.width as f64, renderer.config.height as f64)))
        .with_title("Vulkan boilerplate")        
        .build_vk_surface(&events_loop, renderer.device.instance().clone()).unwrap();
    
    let target_frame_duration = match renderer.config.fps {
        Some(target) => Some(1.0 / target),
        None => None
    };
    
    let (mut swapchain, mut images) = create_swapchain(
        renderer.device.clone(), surface.clone(), renderer.queue.clone(),
        &mut renderer.state, &renderer.config, None);

    let basic_vertex_shader = shaders::basic::vertex::Shader::load(renderer.device.clone()).expect("Failed to create vertex shader");
    let basic_fragment_shader = shaders::basic::fragment::Shader::load(renderer.device.clone()).expect("Failed to create fragment shader");
    
    let gizmo_vertex_shader = shaders::gizmo::vertex::Shader::load(renderer.device.clone()).expect("Failed to create vertex shader");
    let gizmo_fragment_shader = shaders::gizmo::fragment::Shader::load(renderer.device.clone()).expect("Failed to create fragment shader");

    let model = renderer::resources::Model::cube(1.0, Color::<f32>::new(1.0, 0.6, 0.0));
    let gizmo = Gizmo::new(2.0);

    let vertex_buffer = CpuAccessibleBuffer::from_iter(
        renderer.device.clone(),
        BufferUsage::vertex_buffer(),
        model.vertices.into_iter()).unwrap();

    let index_buffer = CpuAccessibleBuffer::from_iter(
        renderer.device.clone(),
        BufferUsage::index_buffer(),
        model.indices.into_iter()).unwrap();
    
    let uniform_buffer = CpuBufferPool::<shaders::basic::vertex::ty::Data>::new(renderer.device.clone(), BufferUsage::uniform_buffer());
    let gizmo_uniform_buffer = CpuBufferPool::<shaders::gizmo::vertex::ty::Data>::new(renderer.device.clone(), BufferUsage::uniform_buffer());
    
    let render_pass = Arc::new(vulkano::single_pass_renderpass!(renderer.device.clone(),
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

    let mut pipeline = create_pipeline(&basic_vertex_shader, &basic_fragment_shader, &renderer.state, render_pass.clone(), renderer.device.clone());

    let mut gizmo_pipeline = Arc::new(
        GraphicsPipeline::start()
            .line_list()
            .line_width(renderer.config.line_width)
            .vertex_input_single_buffer::<Vertex>()
            .vertex_shader(gizmo_vertex_shader.main_entry_point(), ())
            .fragment_shader(gizmo_fragment_shader.main_entry_point(), ())
            .viewports_dynamic_scissors_irrelevant(1)
            .viewports(std::iter::once(Viewport {
                origin: [0.0, 0.0],
                dimensions: renderer.state.dimensions,
                depth_range: 0.0..1.0,
            }))
            .depth_stencil_simple_depth()
            .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
            .build(renderer.device.clone())
            .unwrap()
    );

    let mut framebuffers = create_framebuffers(renderer.device.clone(), &images, render_pass.clone());

    let mut fps_counter = FPSCounter::new();

    let start = std::time::Instant::now();

    let mut camera = CameraCenter::new();
    camera.set_active(true);

    let light_position = Vector3::new(0.0, 10.0, 0.0);
    
    let gizmo_vertex_buffer = {
        let gizmo_transfer_buffer = CpuAccessibleBuffer::from_iter(
            renderer.device.clone(),
            BufferUsage::transfer_source(),
            gizmo.vertices.into_iter()).unwrap();

        let buffer: Arc<DeviceLocalBuffer<[Vertex]>> = DeviceLocalBuffer::array(
            renderer.device.clone(),
            gizmo_transfer_buffer.len(),
            BufferUsage::vertex_buffer_transfer_destination(),
            vec![renderer.queue.family()]
        ).unwrap();

        let transfer_command = AutoCommandBufferBuilder::primary_one_time_submit(
            renderer.device.clone(),
            renderer.queue.family()).unwrap()
            .copy_buffer(gizmo_transfer_buffer.clone(), buffer.clone()).unwrap()
            .build().unwrap();
    
        transfer_command.execute(renderer.queue.clone()).unwrap()
            .then_signal_fence_and_flush().unwrap()
            .wait(None).unwrap();
        
            buffer
    };

    while renderer.state.is_running {
        let frame_start = std::time::Instant::now();
        let _elapsed = (std::time::Instant::now() - start).as_secs_f32();

        events_loop.poll_events(|event| {
            handle_input(event, &mut renderer.state, &mut renderer.input, &mut camera);
        });

        if renderer.state.need_recreation {
            let u_dimensions = [renderer.state.dimensions[0] as u32, renderer.state.dimensions[1] as u32];
            let s = swapchain.recreate_with_dimension(u_dimensions).expect("Failed to recreate swapchain");
            
            swapchain = s.0;
            images = s.1;

            pipeline = create_pipeline(&basic_vertex_shader, &basic_fragment_shader, &renderer.state, render_pass.clone(), renderer.device.clone());

            gizmo_pipeline = Arc::new(
                GraphicsPipeline::start()
                    .line_list()
                    .line_width(renderer.config.line_width)
                    .vertex_input_single_buffer::<Vertex>()
                    .vertex_shader(gizmo_vertex_shader.main_entry_point(), ())
                    .fragment_shader(gizmo_fragment_shader.main_entry_point(), ())
                    .viewports_dynamic_scissors_irrelevant(1)
                    .viewports(std::iter::once(Viewport {
                        origin: [0.0, 0.0],
                        dimensions: renderer.state.dimensions,
                        depth_range: 0.0..1.0,
                    }))
                    .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
                    .depth_stencil_simple_depth()
                    .build(renderer.device.clone())
                    .unwrap()
            );

            framebuffers = create_framebuffers(renderer.device.clone(), &images, render_pass.clone());
            renderer.state.need_recreation = false;
        }

        renderer.input.update();
        
        if renderer.input.mouse_left_button_state == winit::ElementState::Pressed {
            camera.update_pitch(renderer.input.mouse_movement.y as f32 * 0.01);
            camera.update_yaw(-renderer.input.mouse_movement.x as f32 * 0.01);
        }

        let uniform_subbuffer = {
            //let model = Matrix4::from_angle_x(Rad(3.0 * elapsed)) *
            //             Matrix4::from_angle_y(Rad(2.5 * elapsed)) *
            //             Matrix4::from_angle_z(Rad(2.0 * elapsed));
            let model = Matrix4::from_scale(1.0);
            let normal_matrix = model.invert().unwrap().transpose();


            let uniform_data = shaders::basic::vertex::ty::Data {
                model: model.into(),
                normal: normal_matrix.into(),
                view: camera.view_matrix().into(),
                proj: renderer.state.projection.into(),
                light_position: light_position.into(),
                _dummy0: [0; 4],
                view_position: camera.position().into()
            };

            uniform_buffer.next(uniform_data).unwrap()
        };

        let uniform_gizmo_subbuffer = {
            let uniform_data = shaders::gizmo::vertex::ty::Data {
                model: Matrix4::from_scale(1.0).into(),
                normal: Matrix4::from_scale(1.0).into(),
                view: camera.view_matrix().into(),
                proj: renderer.state.projection.into(),
                light_position: light_position.into(),
                _dummy0: [0; 4],
                view_position: camera.position().into()
            };

            gizmo_uniform_buffer.next(uniform_data).unwrap()
        };

        let set = Arc::new(PersistentDescriptorSet::start(pipeline.clone(), 0)
            .add_buffer(uniform_subbuffer).unwrap()
            .build().unwrap());
        
        let gizmo_set = Arc::new(PersistentDescriptorSet::start(gizmo_pipeline.clone(), 0)
            .add_buffer(uniform_gizmo_subbuffer).unwrap()
            .build().unwrap());

        let (image_num, acquire_future) = match vulkano::swapchain::acquire_next_image(swapchain.clone(), None) {
            Ok(r) => r,
            Err(vulkano::swapchain::AcquireError::OutOfDate) => {
                continue;
            },
            Err(err) => panic!("{:?}", err)
        };

        let command_buffer = AutoCommandBufferBuilder::primary_one_time_submit(renderer.device.clone(), renderer.queue.family()).unwrap()
            .begin_render_pass(
                framebuffers[image_num].clone(), false,
                vec![
                    renderer.config.clear_color.into(),
                    1f32.into()
                ]
            ).unwrap()
            .draw(gizmo_pipeline.clone(),
                &DynamicState::none(),
                gizmo_vertex_buffer.clone(),
                gizmo_set.clone(), ()).unwrap()
            .draw_indexed(
                pipeline.clone(),
                &DynamicState::none(),
                vec![vertex_buffer.clone()],
                index_buffer.clone(), set.clone(), ()).unwrap()
            .end_render_pass().unwrap()
            .build().unwrap();
        
        match acquire_future.then_execute(renderer.queue.clone(), command_buffer) {
            Ok(buffer_execute_future) => {
                match buffer_execute_future
                    .then_swapchain_present(renderer.queue.clone(), swapchain.clone(), image_num)
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

        let frame_end = std::time::Instant::now();
        if let Some(target) = target_frame_duration {
            if let Some(sleep_duration) = Duration::from_secs_f32(target).checked_sub(frame_end - frame_start) {
                std::thread::sleep(sleep_duration);
            }
        }
    }
}

fn handle_input(event: Event, state: &mut ApplicationState, input: &mut Input, camera: &mut CameraCenter) {
    match event {
        Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => state.is_running = false,
        Event::WindowEvent { event: WindowEvent::CursorMoved {
            position,
            ..
        }, .. } => {
            input.new_mouse_position = Some(position);
        },
        Event::WindowEvent { event: WindowEvent::MouseInput { state, button: winit::MouseButton::Left, .. }, .. } => {
            input.mouse_left_button_state = state;
        },
        Event::WindowEvent { event: WindowEvent::MouseWheel { delta, ..}, ..} => {
            let (_x, y): (f64, f64) = match delta {
                winit::MouseScrollDelta::LineDelta(x, y) => {(x as f64, y as f64)},
                winit::MouseScrollDelta::PixelDelta(LogicalPosition { x, y }) => {(x, y)},
            };
            camera.update_radius(-y as f32 * 0.1);
        }
        Event::WindowEvent { event: WindowEvent::Resized(LogicalSize { width, height }), .. } => {
            state.set_dimensions(width as f32, height as f32);
            state.need_recreation = true;
        },
        _ => {},
    }
}