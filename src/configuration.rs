use std::marker::PhantomData;

use crate::{
    application::Application, event::Event, AppState, EventLoop, RenderState,
};

#[derive(Debug)]
pub struct Configuration<App, Rend, RendState, Evt, Window, EvtLoop>
where
    Evt: Event,
    RendState: RenderState<Window, Rend>,
    EvtLoop: EventLoop<
        App::Message,
        AppState<App, Rend, RendState, Evt, Window>,
        Event = Evt,
        Window = Window,
    >,
    App: Application<Rend, Evt>,
{
    pub render_config: RendState::Config,
    pub _p: PhantomData<(App, Evt, EvtLoop)>,
}

impl<App, Rend, RendState, Evt, Window, EvtLoop> Default
    for Configuration<App, Rend, RendState, Evt, Window, EvtLoop>
where
    Evt: Event,
    RendState: RenderState<Window, Rend>,
    EvtLoop: EventLoop<
        App::Message,
        AppState<App, Rend, RendState, Evt, Window>,
        Event = Evt,
        Window = Window,
    >,
    App: Application<Rend, Evt>,
    RendState::Config: Default,
{
    fn default() -> Self {
        Self {
            render_config: Default::default(),
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
