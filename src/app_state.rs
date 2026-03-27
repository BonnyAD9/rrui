use std::marker::PhantomData;

use minlin::{MapExt, RectExt};

use crate::{
    AppCtrl, Configuration, Element, EventLoop, LayoutBounds, MayInit,
    RenderState, Renderer, Shell, Widget, Window,
    application::Application,
    event::{Event, EventInfo, EventKind},
    widgets::Nothing,
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
    root: Element<Rend, App::Message, Evt, App::Theme>,
    shell: Shell,
    pending_redraw: bool,
    _phantom: PhantomData<(Rend, Evt, Win)>,
}

impl<App, Rend, RendState, Evt, Win> AppState<App, Rend, RendState, Evt, Win>
where
    Win: Window,
    App: Application<Rend, Evt>,
    RendState: RenderState<Win, Rend>,
    Rend: Renderer,
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
            pending_redraw: true,
            _phantom: PhantomData,
        }
    }

    pub fn init(&mut self, ctrl: impl AppCtrl<Window = Win>) {
        let s = self.render_state.init(|rc| {
            let win_cfg = self.window_config.take().unwrap();
            let win = ctrl.create_window(win_cfg).unwrap();
            self.shell.window_bounds.set_size(win.size().cast());
            RendState::create(rc, win).unwrap()
        });
        s.request_redraw();
        self.root = self.app.root();
        self.shell.request_redraw();
        self.shell.request_relayout();
    }

    pub fn message(&mut self, msg: App::Message) {
        self.app.message(msg);
    }

    pub fn event(&mut self, event: Evt, ctrl: impl AppCtrl) {
        let MayInit::Initialized(state) = &mut self.render_state else {
            return;
        };

        let event_info = EventInfo::new(event, self.shell.mouse_pos);

        match event_info.get_type() {
            EventKind::CloseRequest => ctrl.exit(),
            EventKind::Resize(size) => {
                self.shell.relayout = true;
                self.shell.window_bounds.set_size(size.cast());
                state.resize(size);
            }
            EventKind::RedrawRequest => {
                if self.shell.redraw {
                    self.shell.redraw = false;
                    self.pending_redraw = false;
                    let rend = state.renderer();
                    rend.reset(self.shell.window_bounds.size().cast());
                    self.root.draw(
                        &mut self.shell,
                        self.app.theme(),
                        state.renderer(),
                    );
                }
                state.render();
            }
            EventKind::MouseMove(pos) => {
                self.shell.mouse_pos = Some(pos);
            }
            EventKind::MouseLeaveWindow => {
                self.shell.mouse_pos = None;
            }
            EventKind::ModifiersChange(modifiers) => {
                self.shell.modifiers = modifiers;
            }
            _ => {}
        }

        if event_info.is_for_widgets() {
            self.root
                .event(&mut self.shell, self.app.theme(), &event_info);
        }

        if self.shell.relayout {
            self.shell.redraw = true;
            self.shell.relayout = false;
            let bounds = LayoutBounds::filling(self.shell.window_bounds);
            self.root.layout(
                &mut self.shell,
                self.app.theme(),
                &bounds,
                state.renderer(),
            );
        }

        if self.shell.redraw && !self.pending_redraw {
            self.pending_redraw = true;
            state.request_redraw();
        }
    }
}
