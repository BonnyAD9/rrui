use std::marker::PhantomData;

use crate::{
    application::Application, event::Event, AppState, EventLoop, RenderState,
    Window,
};

#[derive(Debug)]
pub struct Configuration<App, Rend, RendState, Evt, Win, EvtLoop>
where
    Win: Window,
    Evt: Event,
    RendState: RenderState<Win, Rend>,
    EvtLoop: EventLoop<
        App::Message,
        AppState<App, Rend, RendState, Evt, Win>,
        Event = Evt,
        Window = Win,
    >,
    App: Application<Rend, Evt>,
{
    pub render_config: RendState::Config,
    pub window_config: Win::Config,
    _p: PhantomData<(App, Evt, EvtLoop)>,
}

impl<App, Rend, RendState, Evt, Win, EvtLoop> Default
    for Configuration<App, Rend, RendState, Evt, Win, EvtLoop>
where
    Evt: Event,
    Win: Window,
    RendState: RenderState<Win, Rend>,
    EvtLoop: EventLoop<
        App::Message,
        AppState<App, Rend, RendState, Evt, Win>,
        Event = Evt,
        Window = Win,
    >,
    App: Application<Rend, Evt>,
    RendState::Config: Default,
{
    fn default() -> Self {
        Self {
            render_config: Default::default(),
            window_config: Default::default(),
            _p: Default::default(),
        }
    }
}

pub mod config {
    use std::sync::Arc;

    use crate::{application::Application, Configuration};

    #[allow(type_alias_bounds)]
    pub type IcedWgpuWinit<
        App: Application<crate::iced_wgpu::Renderer, winit::event::WindowEvent>,
    > = Configuration<
        App,
        crate::iced_wgpu::Renderer,
        crate::wgpu::State<
            Arc<winit::window::Window>,
            crate::iced_wgpu::Renderer,
        >,
        winit::event::WindowEvent,
        Arc<winit::window::Window>,
        winit::event_loop::EventLoop<App::Message>,
    >;
}
