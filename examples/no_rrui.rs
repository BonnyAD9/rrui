use std::sync::Arc;

use anyhow::Result;
use futures::executor::block_on;
use iced_wgpu::{
    core::{
        alignment,
        renderer::Quad,
        text::{Alignment, LineHeight, Renderer as _, Shaping, Wrapping},
        Background, Border, Color, Font, Point, Rectangle, Renderer as _,
        Shadow, Size, Text,
    },
    graphics::{Antialiasing, Shell, Viewport},
    Engine, Renderer,
};
use log::error;
use wgpu::{
    Backends, Device, DeviceDescriptor, ExperimentalFeatures, Features,
    Instance, InstanceDescriptor, Limits, RequestAdapterOptions, Surface,
    SurfaceConfiguration, SurfaceError, TextureUsages, TextureViewDescriptor,
    Trace,
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
    renderer: Renderer,
    surface: Surface<'static>,
    config: SurfaceConfiguration,
    device: Device,
    viewport: Viewport,
    is_surface_configured: bool,
}

impl State {
    pub fn new(window: Arc<Window>) -> Result<Self> {
        let size = window.inner_size();

        let instance = Instance::new(&InstanceDescriptor {
            backends: Backends::PRIMARY,
            ..InstanceDescriptor::default()
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
            .find(|f| f.is_srgb() && f.components() == 4)
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let engine = Engine::new(
            &adapter,
            device.clone(),
            queue,
            config.format,
            Some(Antialiasing::MSAAx4),
            Shell::headless(),
        );
        let renderer = Renderer::new(engine, Font::DEFAULT, 64.into());

        let viewport = Viewport::with_physical_size(
            Size::new(size.width, size.height),
            1.,
        );

        Ok(Self {
            window,
            renderer,
            config,
            surface,
            device,
            viewport,
            is_surface_configured: false,
        })
    }

    pub fn resize(&mut self, w: u32, h: u32) {
        if w > 0 && h > 0 {
            self.config.width = w;
            self.config.height = h;
            self.viewport = Viewport::with_physical_size(Size::new(w, h), 1.);
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
        }
    }

    pub fn render(&mut self) {
        self.window.request_redraw();

        self.renderer.reset(Rectangle {
            x: 0.,
            y: 0.,
            width: self.config.width as f32,
            height: self.config.height as f32,
        });

        if !self.is_surface_configured {
            return;
        }

        let output = match self.surface.get_current_texture() {
            Ok(r) => r,
            Err(SurfaceError::Lost | SurfaceError::Outdated) => {
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

        self.renderer.fill_quad(
            Quad {
                bounds: Rectangle {
                    x: 10.,
                    y: 10.,
                    width: 50.,
                    height: 50.,
                },
                border: Border::default(),
                shadow: Shadow::default(),
                snap: false,
            },
            Background::Color(Color::from_rgb8(0x12, 0x34, 0x56)),
        );

        self.renderer.fill_text(
            Text {
                content: "Hello rrui!".into(),
                bounds: Size::new(150., 30.),
                size: 16.into(),
                line_height: LineHeight::Absolute(20.into()),
                font: Font::DEFAULT,
                align_x: Alignment::Left,
                align_y: alignment::Vertical::Top,
                shaping: Shaping::Auto,
                wrapping: Wrapping::WordOrGlyph,
            },
            Point::new(20., 20.),
            Color::from_rgba8(255, 255, 255, 1.0),
            Rectangle {
                x: 20.,
                y: 20.,
                width: 100.,
                height: 50.,
            },
        );

        self.renderer.present(
            Some(Color::from_rgb8(0x65, 0x43, 0x21)),
            self.config.format,
            &view,
            &self.viewport,
        );

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
