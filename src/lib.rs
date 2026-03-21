mod app_ctrl;
mod app_state;
mod application;
mod configuration;
mod element;
pub mod event;
mod event_loop;
mod event_loop_proxy;
pub mod iced_wgpu;
mod render_state;
mod renderer;
mod shell;
pub mod wgpu;
mod widget;
pub mod widgets;
pub mod winit;

use crate::event::Event;

pub use self::{
    app_ctrl::*, app_state::*, application::*, configuration::*, element::*,
    event_loop::*, event_loop_proxy::*, render_state::*, renderer::*,
    shell::*, widget::*,
};

pub fn run<App, Rend, RendState, Evt, Window, EvtLoop>(
    app: App,
    config: Configuration<App, Rend, RendState, Evt, Window, EvtLoop>,
) -> Result<(), EvtLoop::Error>
where
    Evt: Event,
    RendState: RenderState<Window, Rend> + 'static,
    EvtLoop: EventLoop<
        App::Message,
        AppState<App, Rend, RendState, Evt, Window>,
        Event = Evt,
        Window = Window,
    >,
    App: Application<Rend, Evt>,
{
    let mut state = AppState::new(app, config.render_config);
    EvtLoop::create()?.run(&mut state)
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert!(true);
    }
}
