use std::fmt::Debug;

use futures::executor::block_on;
use log::error;
use minlin::Vec2;
use thiserror::Error;
use wgpu::{
    Backends, Device, DeviceDescriptor, ExperimentalFeatures, Features,
    Instance, InstanceDescriptor, Limits, RequestAdapterError,
    RequestAdapterOptions, RequestDeviceError, Surface, SurfaceConfiguration,
    SurfaceError, TextureUsages, TextureViewDescriptor, Trace,
};

use crate::{
    wgpu::{Renderer, Window},
    RenderState,
};

#[derive(Debug, Error)]
pub enum StateCreateError {
    #[error(transparent)]
    RequestAdapter(#[from] RequestAdapterError),
    #[error(transparent)]
    RequestDevice(#[from] RequestDeviceError),
}

#[derive(Debug)]
pub struct State<W: Window, R: Renderer> {
    window: W,

    device: Device,
    surface: Surface<'static>,
    config: SurfaceConfiguration,
    is_surface_configured: bool,

    renderer: R,
}

impl<Window: crate::wgpu::Window, Rend: Renderer> RenderState<Window, Rend>
    for crate::wgpu::State<Window, Rend>
{
    type Config = Rend::Config;
    type Error = crate::wgpu::StateCreateError;

    fn create(
        renderer_config: Self::Config,
        window: Window,
    ) -> Result<Self, Self::Error> {
        let size = window.inner_size();

        let instance = Instance::new(&InstanceDescriptor {
            backends: Backends::PRIMARY,
            ..InstanceDescriptor::default()
        });

        let surface = instance.create_surface(window.get_target()).unwrap();

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
            width: size.x,
            height: size.y,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let renderer = Rend::create(
            renderer_config,
            &adapter,
            &device,
            queue,
            config.format,
        );

        Ok(Self {
            window,
            renderer,
            config,
            surface,
            device,
            is_surface_configured: false,
        })
    }

    fn resize(&mut self, size: Vec2<u32>) {
        if size.x > 0 && size.y > 0 {
            self.config.width = size.x;
            self.config.height = size.y;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
        }
    }

    fn render(&mut self) {
        if !self.is_surface_configured {
            self.window.request_redraw();
            return;
        }

        let output = match self.surface.get_current_texture() {
            Ok(r) => r,
            Err(SurfaceError::Lost | SurfaceError::Outdated) => {
                self.resize(self.window.inner_size());
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

        self.renderer.render(
            (self.config.width, self.config.height).into(),
            &view,
            self.config.format,
        );

        output.present();
    }

    fn renderer(&mut self) -> &mut Rend {
        &mut self.renderer
    }

    fn request_redraw(&mut self) {
        self.window.request_redraw();
    }
}
