mod input_action;
mod input_state;
mod input_theme;

use std::{
    cell::{LazyCell, RefCell},
    rc::Rc,
};

pub use self::{input_action::*, input_state::*, input_theme::*};

use minlin::{Padding, RangeExt, Rect, RectExt};

use crate::{
    Background, ControlRenderer, Editor, EditorAction, EditorEdit,
    EditorParams, Element, Quad, QuadRenderer, RedrawSlot, RelPos, Shell,
    Size, Text, TextRenderer, TextWrap, Widget, WidgetExt, WidgetState,
    event::{Event, EventKind, KeyCode, MouseRelation, MouseState},
};

pub type DynLazy<'a, T> = LazyCell<T, Box<dyn Fn() -> T + 'a>>;

pub struct Input<Style, Editr: Editor, Msg>(
    Rc<RefCell<InputInner<Style, Editr, Msg>>>,
);

pub type DynOnInputCallback<Msg> =
    dyn for<'a> FnMut(
        &DynLazy<String>,
        KeyCode,
        Option<&'a str>,
    ) -> (Option<Msg>, InputAction<'a>);

pub type DynOnChangeCallback<Msg> =
    dyn FnMut(&DynLazy<String>) -> (Option<Msg>, Option<bool>);

pub struct InputInner<Style, Editr: Editor, Msg> {
    pub style: RedrawSlot<Style>,
    pub font: Option<Editr::Font>,
    pub font_size: Option<f32>,
    pub line_height: Size,
    pub wrapping: TextWrap,
    pub padding: Padding<f32>,
    pub react: MouseState,
    pub on_input: Box<DynOnInputCallback<Msg>>,
    pub on_change: Box<DynOnChangeCallback<Msg>>,
    pub on_confirm: Box<DynOnChangeCallback<Msg>>,
    pub confirm_on_focus_lost: bool,
    pub width: Option<f32>,
    update_editor: bool,
    editor: Editr,
    state: InputState,
    bounds: Rect<f32>,
    rel_pos: RelPos,
}

impl<Style, Editr, Msg> Input<Style, Editr, Msg>
where
    Editr: Editor,
    Msg: 'static,
{
    pub fn styled(style: impl Into<RedrawSlot<Style>>) -> Self {
        Self(Rc::new(
            InputInner {
                style: style.into(),
                font: None,
                font_size: None,
                line_height: Text::<Editr::Font>::DEFAULT_LINE_HEIGHT,
                wrapping: TextWrap::None,
                padding: Padding::uniform(4.),
                react: MouseState::LEFT,
                on_input: Box::new(input_default_on_input),
                on_change: Box::new(input_default_on_change),
                on_confirm: Box::new(input_default_on_confirm),
                confirm_on_focus_lost: true,
                width: None,
                editor: Default::default(),
                state: InputState::VALID,
                bounds: Rect::default(),
                rel_pos: RelPos::default(),
                update_editor: true,
            }
            .into(),
        ))
    }

    pub fn new() -> Self
    where
        Style: Default,
    {
        Self::styled(Style::default())
    }

    pub fn style(&mut self, style: impl Into<RedrawSlot<Style>>) -> &mut Self {
        self.0.borrow_mut().style = style.into();
        self
    }

    pub fn font(&mut self, font: impl Into<Option<Editr::Font>>) -> &mut Self {
        self.0.borrow_mut().font = font.into();
        self
    }

    pub fn font_size(
        &mut self,
        font_size: impl Into<Option<f32>>,
    ) -> &mut Self {
        self.0.borrow_mut().font_size = font_size.into();
        self
    }

    pub fn line_heignt(&mut self, line_height: impl Into<Size>) -> &mut Self {
        self.0.borrow_mut().line_height = line_height.into();
        self
    }

    pub fn wrapping(&mut self, wrapping: TextWrap) -> &mut Self {
        self.0.borrow_mut().wrapping = wrapping;
        self
    }

    pub fn padding(&mut self, padding: impl Into<Padding<f32>>) -> &mut Self {
        self.0.borrow_mut().padding = padding.into();
        self
    }

    pub fn react(&mut self, react: MouseState) -> &mut Self {
        self.0.borrow_mut().react = react;
        self
    }

    pub fn on_input(
        &mut self,
        on_input: impl for<'a> FnMut(
            &DynLazy<String>,
            KeyCode,
            Option<&'a str>,
        ) -> (Option<Msg>, InputAction<'a>)
        + 'static,
    ) -> &mut Self {
        self.0.borrow_mut().on_input = Box::new(on_input);
        self
    }

    pub fn on_change(
        &mut self,
        on_change: impl FnMut(&DynLazy<String>) -> (Option<Msg>, Option<bool>)
        + 'static,
    ) -> &mut Self {
        self.0.borrow_mut().on_change = Box::new(on_change);
        self
    }

    pub fn on_confirm(
        &mut self,
        on_confirm: impl FnMut(&DynLazy<String>) -> (Option<Msg>, Option<bool>)
        + 'static,
    ) -> &mut Self {
        self.0.borrow_mut().on_confirm = Box::new(on_confirm);
        self
    }

    pub fn confirm_on_focus_lost(
        &mut self,
        confirm_on_focus_lost: bool,
    ) -> &mut Self {
        self.0.borrow_mut().confirm_on_focus_lost = confirm_on_focus_lost;
        self
    }

    pub fn width(&mut self, width: impl Into<Option<f32>>) -> &mut Self {
        self.0.borrow_mut().width = width.into();
        self
    }
}

impl<Style, Editr, Msg> Default for Input<Style, Editr, Msg>
where
    Editr: Editor,
    Msg: 'static,
    Style: Default,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Rend, Msg, Evt, Theme> Widget<Rend, Msg, Evt, Theme>
    for InputInner<Theme::Style, Rend::Editor, Msg>
where
    Theme: InputTheme,
    Rend: TextRenderer + QuadRenderer + ControlRenderer,
    Evt: Event,
{
    fn layout(
        &mut self,
        lp: &mut crate::LayoutParams<'_, Rend, Msg, Evt, Theme>,
        bounds: &crate::LayoutBounds,
        pos_base: RelPos,
        _: crate::LayoutFlags,
    ) -> Rect<f32> {
        self.rel_pos.update(pos_base);
        let fs = self
            .font_size
            .unwrap_or_else(|| lp.renderer.default_font_size());
        let lh = self.line_height.absolute_round(fs);
        self.bounds = bounds.best_at_least(self.padding.size() + [0., lh]);
        if let Some(w) = self.width {
            self.bounds.set_width(bounds.size.range.xrange().clamp(w));
        }
        let params = EditorParams {
            font: self
                .font
                .clone()
                .unwrap_or_else(|| lp.renderer.default_font()),
            font_size: fs,
            line_height: self.line_height,
            wrapping: self.wrapping,
        };
        self.editor
            .update([self.bounds.x - self.padding.size().x, lh], &params);
        self.update_editor = false;
        self.bounds
    }

    fn size(&mut self, _: &Theme) -> minlin::Vec2<f32> {
        self.bounds.size()
    }

    fn reposition(&mut self, _: &Theme, pos: minlin::Vec2<f32>) {
        self.bounds.set_pos(pos);
    }

    fn event(
        &mut self,
        shell: &mut crate::Shell<Rend, Msg, Evt, Theme>,
        theme: &Theme,
        event: &crate::event::EventInfo<Evt>,
    ) -> bool {
        let bounds = self.rel_pos.position_rect(self.bounds);

        let mut new_state = self.state;
        let handled = match event.mouse_relate_to(bounds) {
            MouseRelation::None => {
                if !self.state.contains(InputState::FOCUS) {
                    return false;
                }
                let Some(k) = event.key_press() else {
                    return false;
                };
                let chr = event.key_char();
                let (msg, act) = {
                    let contents =
                        DynLazy::new(Box::new(|| self.editor.get_text()));
                    (self.on_input)(&contents, k, chr.as_deref())
                };
                shell.msgs(msg);
                let (changed, confirmed) = match act {
                    InputAction::Default => {
                        let mut act =
                            EditorAction::from_key(k, shell.modifiers());
                        if act.is_none()
                            && let Some(s) = chr.as_deref()
                            && s.chars().all(|c| !c.is_ascii_control())
                        {
                            act =
                                Some(EditorAction::Edit(EditorEdit::Paste(s)));
                        }
                        let changed =
                            matches!(act, Some(EditorAction::Edit(_)));
                        if let Some(act) = act {
                            self.editor.do_action(act);
                            self.update_editor = true;
                        }
                        (changed, false)
                    }
                    InputAction::Confirm => (false, true),
                    InputAction::Ignore => (false, false),
                    InputAction::Action(editor_action) => {
                        let changed =
                            matches!(editor_action, EditorAction::Edit(_));
                        self.editor.do_action(editor_action);
                        self.update_editor = true;
                        (changed, false)
                    }
                };

                let contents =
                    DynLazy::new(Box::new(|| self.editor.get_text()));
                if changed {
                    let (msg, valid) = (self.on_change)(&contents);
                    shell.msgs(msg);
                    shell.request_redraw();
                    if let Some(valid) = valid {
                        new_state.set(InputState::VALID, valid);
                    }
                }

                if confirmed {
                    shell.lose_focus();
                    if !self.confirm_on_focus_lost {
                        let (msg, valid) = (self.on_confirm)(&contents);
                        shell.msgs(msg);
                        if let Some(valid) = valid {
                            new_state.set(InputState::VALID, valid);
                        }
                    }
                    true
                } else {
                    true
                }
            }
            MouseRelation::Elswhere => {
                if event.mouse_press().is_some() {
                    new_state = InputState::empty();
                    false
                } else if self.state.contains(InputState::DRAG)
                    && event.is_drag_capture()
                {
                    match event.get_kind() {
                        EventKind::MouseMove(pos) => {
                            self.editor.do_action(EditorAction::Drag(
                                pos - bounds.pos() - self.padding.offset(),
                            ));
                            true
                        }
                        EventKind::MouseRelease(b)
                            if self.react.contains(b.into()) =>
                        {
                            new_state &= !InputState::DRAG;
                            true
                        }
                        _ => true,
                    }
                } else {
                    return false;
                }
            }
            MouseRelation::Hover => {
                if event.mouse_press_of(self.react) {
                    shell.focus();
                    if let Some(pos) = shell.mouse_pos() {
                        self.editor.do_action(EditorAction::Click(
                            pos - bounds.pos() - self.padding.offset(),
                        ));
                    }
                    shell.capture_drag();
                    new_state |= InputState::DRAG;
                    true
                } else if event.mouse_release_of(self.react) {
                    new_state &= !InputState::DRAG;
                    true
                } else {
                    true
                }
            }
            MouseRelation::Enter | MouseRelation::Move => {
                new_state |= InputState::HOVER;
                if self.state.contains(InputState::DRAG) {
                    if let Some(pos) = shell.mouse_pos() {
                        self.editor.do_action(EditorAction::Drag(
                            pos - bounds.pos() - self.padding.offset(),
                        ));
                    }
                    true
                } else {
                    false
                }
            }
            MouseRelation::Leave => {
                new_state &= !InputState::HOVER;
                if self.state.contains(InputState::DRAG) {
                    if let Some(pos) = shell.mouse_pos() {
                        self.editor.do_action(EditorAction::Drag(
                            pos - bounds.pos() - self.padding.offset(),
                        ));
                    }
                    true
                } else {
                    false
                }
            }
        };

        if self.update_editor
            || new_state != self.state
                && theme.is_different(&self.style, self.state, new_state)
        {
            shell.request_redraw();
        }
        self.state = new_state;
        handled
    }

    fn draw(
        &mut self,
        _: &mut crate::Shell<Rend, Msg, Evt, Theme>,
        theme: &Theme,
        renderer: &mut Rend,
    ) {
        let bounds = self.rel_pos.position_rect(self.bounds);
        if let Some(a) = theme.appereance(&self.style, self.state) {
            renderer.draw_quad(&Quad::border(bounds, a.border), a.background);
        }

        let lh = self.line_height.absolute_round(
            self.font_size
                .unwrap_or_else(|| renderer.default_font_size()),
        );
        let ts = (bounds.height() - lh - self.padding.size().y).max(0.) / 2.;
        let tbounds = bounds.pad_rect(self.padding + [0., ts, 0., ts]);

        if !self.editor.is_empty() {
            // This is just hack because the editor is buggy and wouldn't draw
            // anyting
            if self.update_editor {
                let params = EditorParams {
                    font: self
                        .font
                        .clone()
                        .unwrap_or_else(|| renderer.default_font()),
                    font_size: self
                        .font_size
                        .unwrap_or_else(|| renderer.default_font_size()),
                    line_height: self.line_height,
                    wrapping: self.wrapping,
                };
                self.editor
                    .update(self.bounds.size() - self.padding.size(), &params);
            }

            match self.editor.selection() {
                crate::Selection::Caret(mut pos) => {
                    pos += tbounds.pos();
                    let color = theme.cursor(&self.style, self.state);
                    let cb = tbounds.intersect([pos.x, pos.y, 2., lh]);
                    renderer.draw_rect(cb, Background::Solid(color));
                }
                crate::Selection::Range(rects) => {
                    let color = theme.selection(&self.style, self.state);
                    for mut r in rects {
                        r.set_pos(r.pos() + tbounds.pos());
                        renderer.draw_rect(r, Background::Solid(color));
                    }
                }
            }

            let color = theme.foreground(
                &self.style,
                self.state,
                renderer.foreground(),
            );
            renderer.draw_editor(&self.editor, tbounds.pos(), color, tbounds);
        } else {
            let color = theme.cursor(&self.style, self.state);
            let cb = tbounds.intersect([tbounds.x, tbounds.y, 2., lh]);
            renderer.draw_rect(cb, Background::Solid(color));
        }
    }

    fn state_change(
        &mut self,
        shell: &mut Shell<Rend, Msg, Evt, Theme>,
        theme: &Theme,
        state: WidgetState,
    ) -> bool {
        match state {
            WidgetState::FocusLost => {
                let mut new_state = self.state & !InputState::FOCUS;

                if self.confirm_on_focus_lost {
                    let contents =
                        DynLazy::new(Box::new(|| self.editor.get_text()));
                    let (msg, valid) = (self.on_confirm)(&contents);
                    shell.msgs(msg);
                    if let Some(valid) = valid {
                        new_state.set(InputState::VALID, valid);
                    }
                    shell.lose_focus();
                }

                if theme.is_different(&self.style, self.state, new_state) {
                    shell.request_redraw();
                }
                self.state = new_state;
                true
            }
            WidgetState::FocusGain => {
                let new_state = self.state | InputState::FOCUS;
                if theme.is_different(&self.style, self.state, new_state) {
                    shell.request_redraw();
                }
                self.state = new_state;
                true
            }
        }
    }
}

impl<Rend, Msg, Evt, Theme> Widget<Rend, Msg, Evt, Theme>
    for Input<Theme::Style, Rend::Editor, Msg>
where
    Theme: InputTheme,
    Rend: TextRenderer + QuadRenderer + ControlRenderer,
    Evt: Event,
    Theme::Style: 'static,
    Msg: 'static,
{
    fn layout(
        &mut self,
        lp: &mut crate::LayoutParams<'_, Rend, Msg, Evt, Theme>,
        bounds: &crate::LayoutBounds,
        pos_base: RelPos,
        flags: crate::LayoutFlags,
    ) -> Rect<f32> {
        self.0.borrow_mut().layout(lp, bounds, pos_base, flags)
    }

    fn size(&mut self, theme: &Theme) -> minlin::Vec2<f32> {
        <InputInner<Theme::Style, Rend::Editor, Msg> as Widget<
            Rend,
            Msg,
            Evt,
            Theme,
        >>::size(&mut *self.0.borrow_mut(), theme)
    }

    fn reposition(&mut self, theme: &Theme, pos: minlin::Vec2<f32>) {
        <InputInner<Theme::Style, Rend::Editor, Msg> as Widget<
            Rend,
            Msg,
            Evt,
            Theme,
        >>::reposition(&mut *self.0.borrow_mut(), theme, pos)
    }

    fn event(
        &mut self,
        shell: &mut Shell<Rend, Msg, Evt, Theme>,
        theme: &Theme,
        event: &crate::event::EventInfo<Evt>,
    ) -> bool {
        shell.with_focus(self.0.clone(), |s| {
            self.0.borrow_mut().event(s, theme, event)
        })
    }

    fn state_change(
        &mut self,
        _: &mut Shell<Rend, Msg, Evt, Theme>,
        _: &Theme,
        _: WidgetState,
    ) -> bool {
        false
    }

    fn draw(
        &mut self,
        shell: &mut Shell<Rend, Msg, Evt, Theme>,
        theme: &Theme,
        renderer: &mut Rend,
    ) {
        self.0.borrow_mut().draw(shell, theme, renderer);
    }
}

pub fn input_default_on_input<'a, Msg>(
    _: &DynLazy<String>,
    key: KeyCode,
    _: Option<&'a str>,
) -> (Option<Msg>, InputAction<'a>) {
    if matches!(key, KeyCode::Enter | KeyCode::Tab) {
        (None, InputAction::Confirm)
    } else {
        (None, InputAction::Default)
    }
}

pub fn input_default_on_change<Msg>(
    _: &DynLazy<String>,
) -> (Option<Msg>, Option<bool>) {
    (None, None)
}

pub fn input_default_on_confirm<Msg>(
    _: &DynLazy<String>,
) -> (Option<Msg>, Option<bool>) {
    (None, None)
}

impl<Style, Editr: Editor, Msg> WidgetExt for InputInner<Style, Editr, Msg> {}
impl<Style, Editr: Editor, Msg> WidgetExt for Input<Style, Editr, Msg> {}

impl<Rend, Msg, Evt, Theme> From<Input<Theme::Style, Rend::Editor, Msg>>
    for Element<Rend, Msg, Evt, Theme>
where
    Theme: InputTheme,
    Rend: TextRenderer + QuadRenderer + ControlRenderer,
    Evt: Event,
    Theme::Style: 'static,
    Msg: 'static,
{
    fn from(value: Input<Theme::Style, Rend::Editor, Msg>) -> Self {
        Self::from_cell(value.0)
    }
}

impl<Rend, Msg, Evt, Theme> From<InputInner<Theme::Style, Rend::Editor, Msg>>
    for Element<Rend, Msg, Evt, Theme>
where
    Theme: InputTheme,
    Rend: TextRenderer + QuadRenderer + ControlRenderer,
    Evt: Event,
    Theme::Style: 'static,
    Msg: 'static,
{
    fn from(value: InputInner<Theme::Style, Rend::Editor, Msg>) -> Self {
        Self::from_cell(Rc::new(value.into()))
    }
}
