mod scrollable_style;
mod scrollable_theme;
mod scrollbar_behaviour;

use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use minlin::{Infinity, MapExt, Padding, Rect, RectExt, Vec2};

use crate::{
    Element, LayerRenderer, LayoutBounds, Orientation, QuadRenderer, RelPos,
    SvgRenderer, Widget, WidgetExt,
    event::{Event, EventKind, Modifiers, MouseRelation},
    widgets::{
        ButtonTheme, PartScrollbar, ScrollbarEvent, ScrollbarStyle,
        ScrollbarTheme, ThumbTheme, TrackTheme,
    },
};

pub use self::{
    scrollable_style::*, scrollable_theme::*, scrollbar_behaviour::*,
};

pub struct Scrollable<W, Msg, Style: ScrollableStyle>(
    Rc<RefCell<ScrollableInner<W, Msg, Style>>>,
);

pub struct ScrollableInner<W, Msg, Style: ScrollableStyle> {
    pub child: W,
    pub style: Style,
    pub behaviour: Vec2<ScrollbarBehaviour>,
    pub padding: Padding<f32>,
    pub scroll: Vec2<PartScrollbar<Style::ScrollbarStyle>>,
    pub on_scroll: Box<dyn FnMut(Vec2<f32>) -> Option<Msg>>,
    vbounds: Rect<f32>,
    rel_pos: RelPos,
    bounds: Rect<f32>,
}

impl<W, Msg, Style: ScrollableStyle> ScrollableInner<W, Msg, Style> {
    fn with_scrollbars_styled(
        style: Style,
        behaviour: impl Into<Vec2<ScrollbarBehaviour>>,
        child: W,
    ) -> Self {
        Self {
            child,
            behaviour: behaviour.into(),
            padding: Padding::default(),
            scroll: Vec2::new(
                PartScrollbar::styled(
                    style.scrollbar_style(),
                    Orientation::Horizontal,
                ),
                PartScrollbar::styled(
                    style.scrollbar_style(),
                    Orientation::Vertical,
                ),
            ),
            on_scroll: Box::new(|_| None),
            vbounds: Rect::default(),
            rel_pos: RelPos::default(),
            bounds: Rect::default(),
            style,
        }
    }
}

impl<W, Msg, Style: ScrollableStyle> Scrollable<W, Msg, Style> {
    pub fn borrow(&self) -> Ref<'_, ScrollableInner<W, Msg, Style>> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, ScrollableInner<W, Msg, Style>> {
        self.0.borrow_mut()
    }

    pub fn with_scrollbars_styled(
        style: Style,
        behaviour: impl Into<Vec2<ScrollbarBehaviour>>,
        child: W,
    ) -> Self {
        Self(Rc::new(
            ScrollableInner::with_scrollbars_styled(style, behaviour, child)
                .into(),
        ))
    }

    pub fn vertical_styled(style: Style, child: W) -> Self {
        Self::with_scrollbars_styled(
            style,
            [ScrollbarBehaviour::Disabled, ScrollbarBehaviour::Visible],
            child,
        )
    }

    pub fn horizontal_styled(style: Style, child: W) -> Self {
        Self::with_scrollbars_styled(
            style,
            [ScrollbarBehaviour::Visible, ScrollbarBehaviour::Disabled],
            child,
        )
    }

    pub fn both_styled(style: Style, child: W) -> Self {
        Self::with_scrollbars_styled(
            style,
            [ScrollbarBehaviour::Visible, ScrollbarBehaviour::Visible],
            child,
        )
    }

    pub fn styled(style: Style, child: W) -> Self {
        Self::vertical_styled(style, child)
    }

    pub fn behaviour(
        &mut self,
        scrollbar: impl Into<Vec2<ScrollbarBehaviour>>,
    ) -> &mut Self {
        self.0.borrow_mut().behaviour = scrollbar.into();
        self
    }

    pub fn on_scroll(
        &mut self,
        on_scroll: impl FnMut(Vec2<f32>) -> Option<Msg> + 'static,
    ) -> &mut Self {
        self.0.borrow_mut().on_scroll = Box::new(on_scroll);
        self
    }

    pub fn padding(&mut self, padding: impl Into<Padding<f32>>) -> &mut Self {
        self.0.borrow_mut().padding = padding.into();
        self
    }
}

impl<W, Msg, Style: Default + ScrollableStyle> Scrollable<W, Msg, Style> {
    pub fn with_scrollbars(
        scrollbar: impl Into<Vec2<ScrollbarBehaviour>>,
        child: W,
    ) -> Self {
        Self::with_scrollbars_styled(Style::default(), scrollbar, child)
    }

    pub fn vertical(child: W) -> Self {
        Self::vertical_styled(Style::default(), child)
    }

    pub fn horizontal(child: W) -> Self {
        Self::horizontal_styled(Style::default(), child)
    }

    pub fn both(child: W) -> Self {
        Self::both_styled(Style::default(), child)
    }

    pub fn new(child: W) -> Self {
        Self::styled(Style::default(), child)
    }
}

#[rustfmt::skip] // rustfmt is confused from the long clauses and fails.
impl<W, Rend, Msg, Evt, Theme> Widget<Rend, Msg, Evt, Theme>
    for ScrollableInner<W, Msg, <Theme as ScrollableTheme>::Style>
where
    W: Widget<Rend, Msg, Evt, Theme>,
    Theme: ScrollableTheme,
    <Theme as ScrollableTheme>::Style: ScrollableStyle,
    Theme: ScrollbarTheme<Style = <<Theme as ScrollableTheme>::Style
        as ScrollableStyle>::ScrollbarStyle>,
    Theme: ThumbTheme<Style = <<<Theme as ScrollableTheme>::Style
        as ScrollableStyle>::ScrollbarStyle as ScrollbarStyle>::ThumbStyle>,
    Theme: TrackTheme<Style = <<<Theme as ScrollableTheme>::Style
        as ScrollableStyle>::ScrollbarStyle as ScrollbarStyle>::TrackStyle>,
    Theme: ButtonTheme<Style = <<<Theme as ScrollableTheme>::Style
        as ScrollableStyle>::ScrollbarStyle as ScrollbarStyle>::ButtonStyle>,
    Evt: Event,
    Rend: QuadRenderer + SvgRenderer + LayerRenderer,
{
    fn layout(
        &mut self,
        lp: &mut crate::LayoutParams<'_, Rend, Msg, Evt, Theme>,
        bounds: &crate::LayoutBounds,
        pos_base: RelPos,
        flags: crate::LayoutFlags,
    ) -> Rect<f32> {
        self.rel_pos.update(pos_base.clone());
        self.bounds = bounds.best_max();

        let enabled = self.enabled();
        let scroll_sizes = self.scroll_sizes(lp.theme);
        let size = (self.bounds.size() - scroll_sizes.swapped())
            .combine(Vec2::INFINITY, enabled);
        let best = size.combine(Vec2::ZERO, enabled);

        self.vbounds = Rect::from_pos_size(
            self.bounds.pos(),
            self.bounds.size() - scroll_sizes
        )
        .pad_rect(self.padding);

        let mut cspace = LayoutBounds::at_most(Rect::from_pos_size(
            self.vbounds.pos(),
            size,
        ));
        cspace.size.best = Some(best);
        let cbounds = self.child.layout(lp, &cspace, pos_base, flags);

        let cor = if enabled.both() { scroll_sizes } else { Vec2::ZERO };

        if enabled.x {
            let bounds = Rect::new(
                self.bounds.x,
                self.bounds.bottom() - scroll_sizes.x,
                self.bounds.width() - cor.x,
                scroll_sizes.y
            );
            self.scroll.x
                .layout(lp.theme, lp.shell, &LayoutBounds::exact(bounds));
            self.scroll.x.state.view = self.vbounds.width();
            self.scroll.x.state.len = cbounds.width();
        }
        if enabled.y {
            let bounds = Rect::new(
                self.bounds.right() - scroll_sizes.x,
                self.bounds.y,
                scroll_sizes.x,
                self.bounds.height() - cor.y
            );
            self.scroll.y
                .layout(lp.theme, lp.shell, &LayoutBounds::exact(bounds));
            self.scroll.y.state.view = self.vbounds.height();
            self.scroll.y.state.len = cbounds.height();
        }

        let abs_pos = self.abs_pos();

        if abs_pos != Vec2::ZERO {
            self.child
                .reposition(lp.theme, self.vbounds.pos() - abs_pos);
        }

        self.bounds
    }

    fn size(&mut self, theme: &Theme) -> Vec2<f32> {
        let scroll_sizes = self.scroll_sizes(theme);
        self.child.size(theme)
            + scroll_sizes
            + self.padding.size()
    }

    fn reposition(&mut self, theme: &Theme, pos: Vec2<f32>) {
        let change = pos - self.bounds.pos();
        self.bounds.set_pos(pos);
        self.vbounds.set_pos(pos + self.padding.offset());
        self.child
            .reposition(theme, self.vbounds.pos() - self.abs_pos());
        self.scroll.x.reposition_by(change);
        self.scroll.y.reposition_by(change);
    }

    fn event(
        &mut self,
        shell: &mut crate::Shell<Rend, Msg, Evt, Theme>,
        theme: &Theme,
        event: &crate::event::EventInfo<Evt>,
    ) -> bool {
        let bounds = self.rel_pos.position_rect(self.bounds);
        let vbounds = self.rel_pos.position_rect(self.vbounds);

        let off = self.rel_pos.get();
        
        let drag_capture = event.is_drag_capture();

        if self.behaviour.x.enabled() 
            && (!drag_capture || self.scroll.x.is_dragging())
        {
            let (handled, evt) = self.scroll.x.event(off, shell, theme, event);
            if matches!(evt, ScrollbarEvent::ScrollTo(_)) {
                let pos = self.abs_pos();
                self.child.reposition(theme, self.vbounds.pos() - pos);
                (self.on_scroll)(pos);
                return true;
            }
            if handled {
                return true;
            }
        }

        if self.behaviour.y.enabled()
            && (!drag_capture || self.scroll.y.is_dragging())
        {
            let (handled, evt) = self.scroll.y.event(off, shell, theme, event);
            if matches!(evt, ScrollbarEvent::ScrollTo(_)) {
                let pos = self.abs_pos();
                self.child.reposition(theme, self.vbounds.pos() - pos);
                (self.on_scroll)(pos);
                return true;
            }
            if handled {
                return true;
            }
        }
        
        let handled = match event.mouse_relate_to(vbounds) {
            MouseRelation::Elswhere => false,
            _ => self.child.event(shell, theme, event),
        };

        if handled {
            return true;
        }

        match event.mouse_relate_to(bounds) {
            MouseRelation::None | MouseRelation::Elswhere => false,
            MouseRelation::Hover => {
                if let EventKind::MouseScroll(mut s) = event.get_kind() {
                    if shell.modifiers().intersects(Modifiers::SHIFT) {
                        s.swap();
                    }
                    let mut change = false;
                    if self.behaviour.x.enabled() {
                        change |= self.scroll.x.scroll_event(s, shell);
                    }
                    if self.behaviour.y.enabled() {
                        change |= self.scroll.y.scroll_event(s, shell);
                    }
                    if change {
                        let pos = self.abs_pos();
                        self.child.reposition(theme, self.vbounds.pos() - pos);
                        (self.on_scroll)(pos);
                    }
                    true
                } else {
                    false
                }
            }
            _ => false
        }
    }

    fn draw(
        &mut self,
        shell: &mut crate::Shell<Rend, Msg, Evt, Theme>,
        theme: &Theme,
        renderer: &mut Rend,
    ) {
        if let Some(a) =
            <Theme as ScrollableTheme>::appereance(theme, &self.style)
        {
            let bounds = self.rel_pos.position_rect(self.bounds);
            renderer.draw_border(bounds, a.border, a.background);
        }

        let vbounds = self.rel_pos.position_rect(self.vbounds);
        renderer.with_clip(vbounds, |r| {
            self.child.draw(shell, theme, r);
        });

        let off = self.rel_pos.get();

        if self.behaviour.x.visible() {
            self.scroll.x.draw(off, theme, renderer);
        }

        if self.behaviour.y.visible() {
            self.scroll.y.draw(off, theme, renderer);
        }
    }
}

#[rustfmt::skip] // rustfmt is confused from the long clauses and fails.
impl<W, Rend, Msg, Evt, Theme> Widget<Rend, Msg, Evt, Theme>
    for Scrollable<W, Msg, <Theme as ScrollableTheme>::Style>
where
    W: Widget<Rend, Msg, Evt, Theme> + 'static,
    Theme: ScrollableTheme,
    <Theme as ScrollableTheme>::Style: ScrollableStyle + 'static,
    Theme: ScrollbarTheme<Style = <<Theme as ScrollableTheme>::Style
        as ScrollableStyle>::ScrollbarStyle>,
    Theme: ThumbTheme<Style = <<<Theme as ScrollableTheme>::Style
        as ScrollableStyle>::ScrollbarStyle as ScrollbarStyle>::ThumbStyle>,
    Theme: TrackTheme<Style = <<<Theme as ScrollableTheme>::Style
        as ScrollableStyle>::ScrollbarStyle as ScrollbarStyle>::TrackStyle>,
    Theme: ButtonTheme<Style = <<<Theme as ScrollableTheme>::Style
        as ScrollableStyle>::ScrollbarStyle as ScrollbarStyle>::ButtonStyle>,
    Evt: Event,
    Rend: QuadRenderer + SvgRenderer + LayerRenderer,
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

    fn size(&mut self, theme: &Theme) -> Vec2<f32> {
        self.0.borrow_mut().size(theme)
    }

    fn reposition(&mut self, theme: &Theme, pos: Vec2<f32>) {
        self.0.borrow_mut().reposition(theme, pos);
    }

    fn event(
        &mut self,
        shell: &mut crate::Shell<Rend, Msg, Evt, Theme>,
        theme: &Theme,
        event: &crate::event::EventInfo<Evt>,
    ) -> bool {
        shell.with_focus(self.0.clone(), |s| {
            self.0.borrow_mut().event(s, theme, event)
        })
    }

    fn draw(
        &mut self,
        shell: &mut crate::Shell<Rend, Msg, Evt, Theme>,
        theme: &Theme,
        renderer: &mut Rend,
    ) {
        self.0.borrow_mut().draw(shell, theme, renderer);
    }
}

impl<W, Msg, Style: ScrollableStyle> ScrollableInner<W, Msg, Style> {
    fn abs_pos(&self) -> Vec2<f32> {
        self.scroll.as_ref().map(|a| a.state.pos)
    }

    fn enabled(&self) -> Vec2<bool> {
        self.behaviour.map(|a| a.enabled())
    }

    fn visible(&self) -> Vec2<bool> {
        self.behaviour.map(|a| a.visible())
    }

    fn scroll_sizes<Theme>(&self, theme: &Theme) -> Vec2<f32>
    where
        Theme: ScrollbarTheme<Style = Style::ScrollbarStyle>,
    {
        let siz = self
            .scroll
            .as_ref()
            .map(|a| theme.sizes(a.style(), a.orientation()).size);
        Vec2::ZERO.combine(siz, self.visible()).swapped()
    }
}

#[rustfmt::skip] // rustfmt is confused from the long clauses and fails
impl<W, Rend, Msg, Evt, Theme>
    From<Scrollable<W, Msg, <Theme as ScrollableTheme>::Style>>
    for Element<Rend, Msg, Evt, Theme>
where
    W: Widget<Rend, Msg, Evt, Theme> + 'static,
    Theme: ScrollableTheme,
    <Theme as ScrollableTheme>::Style: ScrollableStyle + 'static,
    Theme: ScrollbarTheme<Style = <<Theme as ScrollableTheme>::Style
        as ScrollableStyle>::ScrollbarStyle>,
    Theme: ThumbTheme<Style = <<<Theme as ScrollableTheme>::Style
        as ScrollableStyle>::ScrollbarStyle as ScrollbarStyle>::ThumbStyle>,
    Theme: TrackTheme<Style = <<<Theme as ScrollableTheme>::Style
        as ScrollableStyle>::ScrollbarStyle as ScrollbarStyle>::TrackStyle>,
    Theme: ButtonTheme<Style = <<<Theme as ScrollableTheme>::Style
        as ScrollableStyle>::ScrollbarStyle as ScrollbarStyle>::ButtonStyle>,
    Evt: Event,
    Rend: QuadRenderer + SvgRenderer + LayerRenderer,
    Msg: 'static,
{
    fn from(value: Scrollable<W, Msg, <Theme as ScrollableTheme>::Style>)
    -> Self {
        Self::from_cell(value.0)
    }
}

impl<W, Msg, Style> WidgetExt for Scrollable<W, Msg, Style> where
    Style: ScrollableStyle
{
}
