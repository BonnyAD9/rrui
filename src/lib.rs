mod app_ctrl;
mod app_state;
mod application;
mod configuration;
mod element;
pub mod event;
mod event_loop;
mod event_loop_proxy;
pub mod iced_wgpu;
mod layout;
mod may_init;
mod primitives;
mod render_state;
mod renderer;
mod shell;
mod theme;
pub mod wgpu;
mod widget;
pub mod widgets;
mod window;
pub mod winit;

use crate::event::Event;

pub use self::{
    app_ctrl::*, app_state::*, application::*, configuration::*, element::*,
    event_loop::*, event_loop_proxy::*, layout::*, may_init::*, primitives::*,
    render_state::*, renderer::*, shell::*, theme::*, widget::*, window::*,
};

pub type Color = minlin::Rgba<f32>;

pub fn run<App, Rend, RendState, Evt, Win, EvtLoop>(
    app: App,
    config: Configuration<App, Rend, RendState, Evt, Win, EvtLoop>,
) -> Result<(), EvtLoop::Error>
where
    Evt: Event,
    Win: Window,
    Rend: crate::Renderer,
    RendState: RenderState<Win, Rend> + 'static,
    EvtLoop: EventLoop<
            App::Message,
            AppState<App, Rend, RendState, Evt, Win>,
            Event = Evt,
            Window = Win,
        >,
    App: Application<Rend, Evt>,
{
    let mut state = AppState::new(app, config);
    EvtLoop::create()?.run(&mut state)
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert!(true);
    }
}
