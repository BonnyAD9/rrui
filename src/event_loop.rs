use std::sync::Arc;

use crate::{
    application::Application, event::Event, AppState, EventLoopProxy,
    RenderState,
};

pub trait EventLoop<Message, AppState>: Sized {
    type Event: Event;
    type Proxy: EventLoopProxy<Message>;
    type Window;
    type Error;

    fn create() -> Result<Self, Self::Error>;

    fn run(self, app: &mut AppState) -> Result<(), Self::Error>;
}

impl<App, Rend, RendState>
    EventLoop<
        App::Message,
        AppState<
            App,
            Rend,
            RendState,
            winit::event::WindowEvent,
            Arc<winit::window::Window>,
        >,
    > for winit::event_loop::EventLoop<App::Message>
where
    App: Application<Rend, winit::event::WindowEvent>,
    RendState: RenderState<Arc<winit::window::Window>, Rend> + 'static,
{
    type Event = winit::event::WindowEvent;
    type Proxy = winit::event_loop::EventLoopProxy<App::Message>;
    type Window = Arc<winit::window::Window>;
    type Error = winit::error::EventLoopError;

    fn create() -> Result<Self, Self::Error> {
        Self::with_user_event().build()
    }

    fn run(
        self,
        app: &mut AppState<
            App,
            Rend,
            RendState,
            winit::event::WindowEvent,
            Arc<winit::window::Window>,
        >,
    ) -> Result<(), Self::Error> {
        self.run_app(app)
    }
}
