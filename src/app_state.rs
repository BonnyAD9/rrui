use std::marker::PhantomData;

use crate::{
    application::Application,
    event::{Event, EventType},
    widgets::Nothing,
    AppCtrl, Configuration, Element, EventLoop, MayInit, RenderState, Shell,
    Widget, Window,
};

pub struct AppState<App, Rend, RendState, Evt, Win>
where
    Win: Window,
    App: Application<Rend, Evt>,
    RendState: RenderState<Win, Rend>,
    Evt: Event,
{
    app: App,
    render_state: MayInit<RendState::Config, RendState>,
    window_config: Option<Win::Config>,
    root: Element<Rend, App::Message, Evt>,
    shell: Shell,
    _phantom: PhantomData<(Rend, Evt, Win)>,
}

impl<App, Rend, RendState, Evt, Win> AppState<App, Rend, RendState, Evt, Win>
where
    Win: Window,
    App: Application<Rend, Evt>,
    RendState: RenderState<Win, Rend>,
    Evt: Event,
{
    pub fn new<EvtLoop>(
        app: App,
        config: Configuration<App, Rend, RendState, Evt, Win, EvtLoop>,
    ) -> Self
    where
        EvtLoop: EventLoop<App::Message, Self, Event = Evt, Window = Win>,
    {
        Self {
            app,
            render_state: MayInit::Uninitialized(config.render_config),
            window_config: Some(config.window_config),
            root: Nothing.into(),
            shell: Shell::default(),
            _phantom: PhantomData,
        }
    }

    pub fn init(&mut self, ctrl: impl AppCtrl<Window = Win>) {
        self.render_state.init(|rc| {
            let win_cfg = self.window_config.take().unwrap();
            let win = ctrl.create_window(win_cfg).unwrap();
            RendState::create(rc, win).unwrap()
        });
        self.root = self.app.root();
        self.shell.request_redraw();
    }

    pub fn message(&mut self, msg: App::Message) {
        self.app.message(msg);
    }

    pub fn event(&mut self, event: Evt, ctrl: impl AppCtrl) {
        let MayInit::Initialized(state) = &mut self.render_state else {
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
