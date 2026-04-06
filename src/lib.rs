mod app_state;
mod application;
mod configuration;
mod element;
pub mod event;
pub mod iced_wgpu;
mod layout;
mod may_init;
mod primitives;
mod renderer;
mod shell;
mod shell_proxy;
mod theme;
mod traits;
mod variable;
pub mod wgpu;
mod widget;
mod widget_ext;
pub mod widgets;
pub mod winit;

use crate::event::Event;

pub use self::{
    app_state::*, application::*, configuration::*, element::*, layout::*,
    may_init::*, primitives::*, renderer::*, shell::*, shell_proxy::*,
    theme::*, traits::*, variable::*, widget::*, widget_ext::*,
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
