use std::{marker::PhantomData, mem};

use minlin::{MapExt, RectExt};

use crate::{
    AppCtrl, Configuration, Element, EventLoop, LayoutBounds, LayoutParams,
    MayInit, RelPos, RenderState, Renderer, Shell, Widget, Window,
    application::Application,
    event::{Event, EventCtrl, EventInfo, EventKind},
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
    shell: Shell<App::Message>,
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
            shell: Shell::<App::Message>::default(),
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
        self.shell.request_redraw();
        self.shell.request_relayout();
        self.root = self.app.root(&mut self.shell);
        self.root.init();
    }

    pub fn message(&mut self, msg: App::Message) {
        self.shell.messages.push(msg);
        self.flush_messages();
    }

    fn flush_messages(&mut self) {
        let mut msgs = mem::take(&mut self.shell.messages);
        self.app.messages(&mut self.shell, &mut msgs);
        if self.shell.messages.is_empty() {
            self.shell.messages = msgs;
        }
    }

    pub fn event(&mut self, event: Evt, ctrl: impl AppCtrl) {
        let MayInit::Initialized(state) = &mut self.render_state else {
            return;
        };

        let event_info = EventInfo::new(event, self.shell.mouse_pos);

        let mut evt_ctrl = EventCtrl::new();
        self.app
            .pre_event(&mut self.shell, &event_info, &mut evt_ctrl);

        if !evt_ctrl.ignore {
            match event_info.get_kind() {
                EventKind::CloseRequest => ctrl.exit(),
                EventKind::Resize(size) => {
                    self.shell.request_relayout();
                    self.shell.window_bounds.set_size(size.cast());
                    state.resize(size);
                }
                EventKind::RedrawRequest => {
                    if self.shell.redraw_requested() {
                        self.shell.reset_redraw();
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
                EventKind::MousePress(m) => {
                    self.shell.mouse_state.press(m);
                }
                EventKind::MouseRelease(m) => {
                    self.shell.mouse_state.release(m);
                }
                _ => {}
            }

            if evt_ctrl
                .for_widgets
                .unwrap_or_else(|| event_info.is_for_widgets())
            {
                let handled = self.root.event(
                    &mut self.shell,
                    self.app.theme(),
                    &event_info,
                );

                if !handled {
                    self.app.post_event(&mut self.shell, &event_info);
                }
            }
        }

        self.flush_messages();

        let MayInit::Initialized(state) = &mut self.render_state else {
            return;
        };

        if self.shell.relayout_requested() {
            self.shell.request_redraw();
            self.shell.reset_relayout();
            let bounds = LayoutBounds::filling(self.shell.window_bounds);
            self.root.layout(
                &mut LayoutParams::new(
                    &mut self.shell,
                    self.app.theme(),
                    state.renderer(),
                ),
                &bounds,
                RelPos::new(),
            );
        }

        if self.shell.redraw_requested() && !self.pending_redraw {
            self.pending_redraw = true;
            state.request_redraw();
        }
    }
}
