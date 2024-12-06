use std::sync::Arc;
use winit::event::WindowEvent;
use winit::window::Window;
use wgpu::{Surface, Device, Queue, SurfaceConfiguration, include_wgsl, RenderPipeline};
use wgpu::util::DeviceExt;

pub struct State<'a> {
    pub surface: Surface<'a>,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub window: Arc<Window>,
    pub render_pipelines: Vec<RenderPipeline>,
    pub number_of_vertices: u32,
    pub curr_pipeline_index: u32,
    pub vertex_buffer: wgpu::Buffer,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ]
        }
    }
}

const VERTICES: &[Vertex] = &[
    Vertex { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
];

impl<'a> State<'a> {
    pub async fn new(window: Arc<Window>) -> State<'a> {
        let number_of_vertices = VERTICES.len() as u32;
        let window_size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: None,
                memory_hints: Default::default(),
            },
            None,
        ).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window_size.width,
            height: window_size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let basic_pipeline = Self::get_default_pipeline(&device, &config, "vs_main", "fs_main");
        let coloring_pipeline = Self::get_default_pipeline(&device, &config, "vs_main_two", "fs_main_two");
        let render_pipelines = vec![basic_pipeline, coloring_pipeline];
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        State {
            size: window_size,
            number_of_vertices,
            curr_pipeline_index: 0,
            render_pipelines,
            vertex_buffer,
            surface,
            device,
            queue,
            window,
            config
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn toggle_pipeline(&mut self) {
        let max = self.render_pipelines.len() as u32;
        let pipeline_index = (self.curr_pipeline_index + 1) % max;

        self.curr_pipeline_index = pipeline_index;
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {

    }

    pub fn render(&mut self, color: (u8, u8, u8)) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: (color.0 as f64) / 255.0,
                        g: (color.1 as f64) / 255.0,
                        b: (color.2 as f64) / 255.0,
                        a: 0.5,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        let pipeline = &self.render_pipelines[self.curr_pipeline_index as usize];

        render_pass.set_pipeline(pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..self.number_of_vertices, 0..1);

        drop(render_pass);

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    fn get_default_pipeline(
        device: &Device,
        config: &SurfaceConfiguration,
        vertex_entry_point: &str,
        fragment_entry_point: &str
    ) -> RenderPipeline {
        let shader = device.create_shader_module(include_wgsl!("shaders/shader.wgsl"));
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Coloring Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some(vertex_entry_point),
                buffers: &[Vertex::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some(fragment_entry_point),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default()
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        })
    }
}
