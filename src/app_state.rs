use std::{marker::PhantomData, sync::Arc};

use winit::window::Window;

use crate::{
    application::Application,
    event::{Event, EventType},
    widgets::Nothing,
    AppCtrl, Element, RenderState, Shell, Widget,
};

pub struct AppState<App, Rend, RendState, Evt, Window>
where
    App: Application<Rend, Evt>,
    RendState: RenderState<Window, Rend>,
    Evt: Event,
{
    app: App,
    config: Option<RendState::Config>,
    render_state: Option<RendState>,
    root: Element<Rend, App::Message, Evt>,
    shell: Shell,
    _phantom: PhantomData<(Rend, Evt, Window)>,
}

impl<App, Rend, RendState, Evt, Window>
    AppState<App, Rend, RendState, Evt, Window>
where
    App: Application<Rend, Evt>,
    RendState: RenderState<Window, Rend>,
    Evt: Event,
{
    pub fn new(app: App, config: RendState::Config) -> Self {
        Self {
            app,
            render_state: None,
            config: Some(config),
            root: Nothing.into(),
            shell: Shell::default(),
            _phantom: PhantomData,
        }
    }

    pub fn init(&mut self, window: Window) {
        let config = self.config.take().unwrap();
        self.render_state = Some(RendState::create(config, window).unwrap());
        self.root = self.app.root();
        self.shell.request_redraw();
    }

    pub fn message(&mut self, msg: App::Message) {
        self.app.message(msg);
    }

    pub fn event(&mut self, event: Evt, ctrl: impl AppCtrl) {
        let Some(state) = &mut self.render_state else {
            return;
        };
        match event.get_type() {
            EventType::CloseRequest => ctrl.exit(),
            EventType::Resize(size) => {
                self.shell.redraw = true;
                state.resize(size);
            }
            EventType::RedrawRequest => {
                if self.shell.redraw {
                    self.shell.redraw = false;
                    self.root.draw(state.renderer(), &mut self.shell);
                }
                state.render();
            }
            _ if !event.is_window() => {
                self.root.event(&event, &mut self.shell);
                if self.shell.redraw {
                    state.request_redraw();
                }
            }
            _ => {}
        }
    }
}

impl<App, Rend, RendState> winit::application::ApplicationHandler<App::Message>
    for AppState<
        App,
        Rend,
        RendState,
        winit::event::WindowEvent,
        Arc<winit::window::Window>,
    >
where
    App: Application<Rend, winit::event::WindowEvent>,
    RendState: RenderState<Arc<winit::window::Window>, Rend> + 'static,
{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes = Window::default_attributes();
        let window =
            event_loop.create_window(window_attributes).unwrap().into();
        self.init(window);
    }

    fn user_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        event: App::Message,
    ) {
        self.message(event);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        self.event(event, event_loop);
    }
}
