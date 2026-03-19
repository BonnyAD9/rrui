use std::{iter, sync::Arc};

use anyhow::Result;
use futures::executor::block_on;
use log::error;
use wgpu::{
    Backends, Color, CommandEncoderDescriptor, CurrentSurfaceTexture, Device,
    DeviceDescriptor, ExperimentalFeatures, Features, Instance,
    InstanceDescriptor, Limits, LoadOp, Operations, Queue,
    RenderPassDescriptor, RequestAdapterOptions, StoreOp, Surface,
    SurfaceConfiguration, TextureUsages, TextureViewDescriptor, Trace,
};
use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

fn main() -> anyhow::Result<()> {
    let _logr = flexi_logger::Logger::try_with_env_or_str("")?.start()?;

    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = App::new();
    event_loop.run_app(&mut app)?;

    Ok(())
}

pub struct State {
    window: Arc<Window>,
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    is_surface_configured: bool,
}

impl State {
    pub fn new(window: Arc<Window>) -> Result<Self> {
        let size = window.inner_size();

        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::PRIMARY,
            ..InstanceDescriptor::new_without_display_handle()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter =
            block_on(instance.request_adapter(&RequestAdapterOptions {
                power_preference: Default::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }))?;

        let (device, queue) =
            block_on(adapter.request_device(&DeviceDescriptor {
                label: None,
                required_features: Features::empty(),
                experimental_features: ExperimentalFeatures::disabled(),
                required_limits: Limits::defaults(),
                memory_hints: Default::default(),
                trace: Trace::Off,
            }))?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        Ok(Self {
            window,
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
        })
    }

    pub fn resize(&mut self, w: u32, h: u32) {
        if w > 0 && h > 0 {
            self.config.width = w;
            self.config.height = h;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
        }
    }

    pub fn render(&mut self) {
        self.window.request_redraw();

        if !self.is_surface_configured {
            return;
        }

        let output = match self.surface.get_current_texture() {
            CurrentSurfaceTexture::Suboptimal(t) => t,
            CurrentSurfaceTexture::Success(t) => t,
            CurrentSurfaceTexture::Lost | CurrentSurfaceTexture::Outdated => {
                let size = self.window.inner_size();
                self.resize(size.width, size.height);
                return;
            }
            v => {
                error!("Failed to get surface: {v:?}");
                return;
            }
        };

        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut encoder =
            self.device
                .create_command_encoder(&CommandEncoderDescriptor {
                    label: Some("Render encoder"),
                });

        encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                depth_slice: None,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
            multiview_mask: None,
        });

        self.queue.submit(iter::once(encoder.finish()));
        output.present();
    }

    fn handle_key(
        &self,
        event_loop: &ActiveEventLoop,
        code: KeyCode,
        is_pressed: bool,
    ) {
        match (code, is_pressed) {
            (KeyCode::Escape, true) => event_loop.exit(),
            _ => {}
        }
    }

    fn update(&mut self) {}
}

pub struct App {
    state: Option<State>,
}

impl App {
    pub fn new() -> Self {
        Self { state: None }
    }
}

impl ApplicationHandler<State> for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes = Window::default_attributes();
        let window =
            Arc::new(event_loop.create_window(window_attributes).unwrap());
        self.state = Some(State::new(window).unwrap());
    }

    fn user_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        event: State,
    ) {
        self.state = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let Some(state) = &mut self.state else {
            return;
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                state.resize(size.width, size.height)
            }
            WindowEvent::RedrawRequested => {
                state.update();
                state.render();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: ks,
                        ..
                    },
                ..
            } => state.handle_key(event_loop, code, ks.is_pressed()),
            _ => {}
        }
    }
}
