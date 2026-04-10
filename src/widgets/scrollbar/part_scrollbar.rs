use minlin::{Rect, RectExt, Vec2};

use crate::{
    LayoutBounds, Orientation, QuadRenderer, RedrawSlot, Shell, SvgRenderer,
    event::{Event, EventInfo, EventKind, MouseRelation, ScrollDelta},
    widgets::{
        ButtonEvent, ButtonTheme, PartButton, PartThumb, PartTrack,
        ScrollbarEvent, ScrollbarState, ScrollbarTheme, ThumbEvent,
        ThumbLayout, ThumbTheme, TrackEvent, TrackTheme,
        scrollbar::scrollbar_style::ScrollbarStyle,
    },
};

#[derive(Debug)]
pub struct PartScrollbar<Style: ScrollbarStyle> {
    pub line_size: Option<f32>,
    pub state: RedrawSlot<ScrollbarState>,
    style: Style,
    orientation: Orientation,
    start_button: PartButton<Style::ButtonStyle>,
    end_button: PartButton<Style::ButtonStyle>,
    thumb: PartThumb<Style::ThumbStyle>,
    start_track: PartTrack<Style::TrackStyle>,
    end_track: PartTrack<Style::TrackStyle>,
    track_bounds: Rect<f32>,
    bounds: Rect<f32>,
}

impl<Style> PartScrollbar<Style>
where
    Style: ScrollbarStyle,
{
    pub const SCROLL_LINE_SIZE: f32 = 80.;

    pub fn styled(style: Style, orientation: Orientation) -> Self {
        Self {
            line_size: None,
            state: Default::default(),
            orientation,
            start_button: PartButton::styled(style.button_style().into()),
            end_button: PartButton::styled(style.button_style().into()),
            thumb: PartThumb::new(style.thumb_style()),
            start_track: PartTrack::new(style.track_style()),
            end_track: PartTrack::new(style.track_style()),
            track_bounds: Rect::default(),
            bounds: Rect::default(),
            style,
        }
    }

    pub fn style(&self) -> &Style {
        &self.style
    }

    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    pub fn osize(&self) -> f32 {
        self.orientation.other_component(self.bounds.size())
    }

    pub fn bounds(&self) -> Rect<f32> {
        self.bounds
    }

    pub fn off_bounds(&self, off: Vec2<f32>) -> Rect<f32> {
        Rect::from_pos_size(self.bounds.pos() + off, self.bounds.size())
    }

    pub fn scroll_event<Msg>(
        &mut self,
        delta: ScrollDelta,
        shell: &mut Shell<Msg>,
    ) -> bool {
        match delta {
            ScrollDelta::Lines(v) => {
                let v = self.orientation.component(v);
                let evt = self.scroll_by(
                    v * self.line_size.unwrap_or(Self::SCROLL_LINE_SIZE),
                    shell,
                );
                matches!(evt, ScrollbarEvent::ScrollTo(_))
            }
            ScrollDelta::Pixels(v) => {
                let evt =
                    self.scroll_by(self.orientation().component(v), shell);
                matches!(evt, ScrollbarEvent::ScrollTo(_))
            }
        }
    }

    fn off_track_bounds(&self, off: Vec2<f32>) -> Rect<f32> {
        Rect::from_pos_size(
            self.track_bounds.pos() + off,
            self.track_bounds.size(),
        )
    }

    pub fn layout<Theme>(
        &mut self,
        theme: &Theme,
        bounds: &LayoutBounds,
    ) -> Rect<f32>
    where
        Theme: ScrollbarTheme<Style = Style>,
    {
        match self.orientation {
            Orientation::Horizontal => self.layout_horizontal(theme, bounds),
            Orientation::Vertical => self.layout_vertical(theme, bounds),
        }
    }

    fn layout_vertical<Theme>(
        &mut self,
        theme: &Theme,
        bounds: &LayoutBounds,
    ) -> Rect<f32>
    where
        Theme: ScrollbarTheme<Style = Style>,
    {
        let sizes = theme.sizes(&self.style);
        let w = sizes.size;
        let h = bounds.size.best_max().y;
        self.bounds = bounds.clamp([w, h]);

        let minh = sizes.button * 2. + theme.min_thumb(&self.style);
        let (buth, trackh) = if self.bounds.height() < minh {
            let h = minh / 3.;
            (h, h)
        } else {
            (sizes.button, self.bounds.height() - sizes.button * 2.)
        };

        self.track_bounds = Rect::new(
            self.bounds.x,
            self.bounds.y + buth,
            self.bounds.width(),
            trackh,
        );

        let but_size = Vec2::new(w, buth);
        self.start_button.size = Some(but_size);
        self.end_button.size = Some(but_size);

        let start_but = Rect::from_pos_size(self.bounds.pos(), but_size)
            + sizes.button_padding;
        self.start_button.layout_direct(start_but);

        let end_but = Rect::from_pos_size(
            [self.bounds.x, self.track_bounds.bottom()],
            but_size,
        ) + sizes.button_padding;
        self.end_button.layout_direct(end_but);

        self.bounds
    }

    fn layout_horizontal<Theme>(
        &mut self,
        theme: &Theme,
        bounds: &LayoutBounds,
    ) -> Rect<f32>
    where
        Theme: ScrollbarTheme<Style = Style>,
    {
        let sizes = theme.sizes(&self.style);
        let h = sizes.size;
        let w = bounds.size.best_max().x;
        self.bounds = bounds.clamp([w, h]);

        let minw = sizes.button * 2. + theme.min_thumb(&self.style);
        let (butw, trackw) = if self.bounds.width() < minw {
            let w = minw / 3.;
            (w, w)
        } else {
            (sizes.button, self.bounds.width() - sizes.button * 2.)
        };

        self.track_bounds = Rect::new(
            self.bounds.x + butw,
            self.bounds.y,
            trackw,
            self.bounds.height(),
        );

        let but_size = Vec2::new(h, butw);
        self.start_button.size = Some(but_size);
        self.end_button.size = Some(but_size);

        let start_but = Rect::from_pos_size(self.bounds.pos(), but_size)
            + sizes.button_padding;
        self.start_button.layout_direct(start_but);

        let end_but = Rect::from_pos_size(
            [self.track_bounds.right(), self.bounds.y],
            but_size,
        ) + sizes.button_padding;
        self.end_button.layout_direct(end_but);

        self.bounds
    }

    pub fn size<Theme>(&self, theme: &Theme) -> Vec2<f32>
    where
        Theme: ScrollbarTheme<Style = Style>,
    {
        let s = theme.sizes(&self.style).size;
        match self.orientation {
            Orientation::Horizontal => Vec2::new(f32::INFINITY, s),
            Orientation::Vertical => Vec2::new(s, f32::INFINITY),
        }
    }

    pub fn reposition(&mut self, pos: Vec2<f32>) {
        let change = pos - self.bounds.pos();
        self.reposition_by(change);
    }

    pub fn reposition_by(&mut self, change: Vec2<f32>) {
        self.bounds.set_pos(self.bounds.pos() + change);
        self.track_bounds.set_pos(self.track_bounds.pos() + change);

        self.start_button
            .reposition_direct(self.start_button.bounds().pos() + change);
        self.end_button
            .reposition_direct(self.end_button.bounds().pos() + change);
    }

    pub fn event<Msg, Evt, Theme>(
        &mut self,
        off: Vec2<f32>,
        shell: &mut Shell<Msg>,
        theme: &Theme,
        event: &EventInfo<Evt>,
    ) -> (bool, ScrollbarEvent)
    where
        Evt: Event,
        Theme: ScrollbarTheme<Style = Style>,
        Theme: ThumbTheme<Style = Style::ThumbStyle>,
        Theme: TrackTheme<Style = Style::TrackStyle>,
        Theme: ButtonTheme<Style = Style::ButtonStyle>,
    {
        self.state.update();
        let bounds = self.off_bounds(off);
        let tbounds = self.off_track_bounds(off);

        match event.mouse_relate_to(bounds) {
            MouseRelation::None | MouseRelation::Elswhere => {
                return (false, ScrollbarEvent::Nothing);
            }
            MouseRelation::Hover => {
                if let EventKind::MouseScroll(s) = event.get_kind() {
                    let evt = match s {
                        ScrollDelta::Lines(v) => self.scroll_by(
                            v.y * self
                                .line_size
                                .unwrap_or(Self::SCROLL_LINE_SIZE),
                            shell,
                        ),
                        ScrollDelta::Pixels(v) => self.scroll_by(v.y, shell),
                    };
                    return (true, evt);
                }
            }
            _ => {}
        }

        let tlay = self.thumb_layout(tbounds, theme.min_thumb(&self.style));
        let (handled, evt) = self.thumb.event(&tlay, shell, theme, event);
        if let ThumbEvent::Move(p) = evt {
            self.move_to_sceen_space_pos(&tlay, p, shell);
            return (true, ScrollbarEvent::ScrollTo(self.state.pos));
        }
        if handled {
            return (true, ScrollbarEvent::Nothing);
        }

        let sbounds = tlay.start_track_bounds();
        let (handled, evt) = self.start_track.event(
            sbounds,
            shell,
            theme,
            event,
            self.orientation,
        );
        if let TrackEvent::PressAt(p) = evt {
            let dpos = tlay.size() / 2.;
            self.thumb.start_drag(dpos, shell);
            return (
                true,
                self.move_to_sceen_space_pos(&tlay, p - dpos, shell),
            );
        }
        if handled {
            return (true, ScrollbarEvent::Nothing);
        }

        let ebounds = tlay.end_track_bounds();
        let (handled, evt) = self.end_track.event(
            ebounds,
            shell,
            theme,
            event,
            self.orientation,
        );
        if let TrackEvent::PressAt(p) = evt {
            let dpos = tlay.size() / 2.;
            self.thumb.start_drag(dpos, shell);
            return (
                true,
                self.move_to_sceen_space_pos(&tlay, p - dpos, shell),
            );
        }
        if handled {
            return (true, ScrollbarEvent::Nothing);
        }

        let (handled, evt) =
            self.start_button.event_direct(off, shell, theme, event);
        if matches!(evt, ButtonEvent::Clicked(_)) {
            return (true, self.scroll_by(self.state.view, shell));
        }
        if handled {
            return (true, ScrollbarEvent::Nothing);
        }

        let (handled, evt) =
            self.end_button.event_direct(off, shell, theme, event);
        if matches!(evt, ButtonEvent::Clicked(_)) {
            return (true, self.scroll_by(-self.state.view, shell));
        }

        (handled, ScrollbarEvent::Nothing)
    }

    pub fn draw<Rend, Theme>(
        &mut self,
        off: Vec2<f32>,
        theme: &Theme,
        renderer: &mut Rend,
    ) where
        Rend: QuadRenderer + SvgRenderer,
        Theme: ScrollbarTheme<Style = Style>,
        Theme: ThumbTheme<Style = Style::ThumbStyle>,
        Theme: TrackTheme<Style = Style::TrackStyle>,
        Theme: ButtonTheme<Style = Style::ButtonStyle>,
    {
        self.state.update();

        if let Some(a) = <Theme as ScrollbarTheme>::appereance(
            theme,
            &self.style,
            self.orientation,
        ) {
            let bounds = self.off_bounds(off)
                + <Theme as ScrollbarTheme>::padding(
                    theme,
                    &self.style,
                    self.orientation,
                );
            renderer.draw_border(bounds, a.border, a.background);
        }

        let track_bounds = self.off_track_bounds(off);
        let tlay =
            self.thumb_layout(track_bounds, theme.min_thumb(&self.style));

        self.start_track.draw(
            || tlay.start_track_bounds(),
            theme,
            renderer,
            self.orientation,
        );
        self.end_track.draw(
            || tlay.end_track_bounds(),
            theme,
            renderer,
            self.orientation,
        );
        self.thumb
            .draw(|| tlay.bounds, theme, renderer, self.orientation);

        let (start, end) = match self.orientation {
            Orientation::Horizontal => (
                theme.left_button(&self.style),
                theme.right_button(&self.style),
            ),
            Orientation::Vertical => (
                theme.top_button(&self.style),
                theme.bottom_button(&self.style),
            ),
        };

        self.start_button.draw_direct(off, theme, renderer);
        renderer.draw_svg(
            self.start_button.off_bounds(off),
            &start.0,
            &start.1,
        );

        self.end_button.draw_direct(off, theme, renderer);
        renderer.draw_svg(self.end_button.off_bounds(off), &end.0, &end.1);
    }

    fn thumb_layout(&self, tb: Rect<f32>, min_size: f32) -> ThumbLayout {
        match self.orientation {
            Orientation::Horizontal => {
                let w = min_size
                    .max(self.state.view / self.state.len * tb.width());
                let tw = tb.width() - w;
                let x = self.state.rel_pos() * tw;
                ThumbLayout {
                    bounds: Rect::new(tb.x + x, tb.y, w, tb.height()),
                    range: tb.x..tb.x + tw,
                    orientation: self.orientation,
                }
            }
            Orientation::Vertical => {
                let h = min_size
                    .max(self.state.view / self.state.len * tb.height());
                let th = tb.height() - h;
                let y = self.state.rel_pos() * th;
                ThumbLayout {
                    bounds: Rect::new(tb.x, tb.y + y, tb.width(), h),
                    range: tb.y..tb.y + th,
                    orientation: self.orientation,
                }
            }
        }
    }

    fn move_to_sceen_space_pos<Msg>(
        &mut self,
        tlay: &ThumbLayout,
        mut p: f32,
        shell: &mut Shell<Msg>,
    ) -> ScrollbarEvent {
        p = p.clamp(tlay.range.start, tlay.range.end);
        let rel = (p - tlay.range.start) / (tlay.range.end - tlay.range.start);

        let new_pos = self.state.from_rel(rel);
        let evt = if new_pos != self.state.pos {
            shell.request_redraw();
            ScrollbarEvent::ScrollTo(new_pos)
        } else {
            ScrollbarEvent::Nothing
        };
        self.state.pos = new_pos;
        evt
    }

    pub fn scroll_by<Msg>(
        &mut self,
        amt: f32,
        shell: &mut Shell<Msg>,
    ) -> ScrollbarEvent {
        let abs_pos = (self.state.pos - amt)
            .min(self.state.len - self.state.view)
            .max(0.);
        if abs_pos == self.state.pos {
            return ScrollbarEvent::Nothing;
        }
        shell.request_redraw();
        self.state.pos = abs_pos;
        ScrollbarEvent::ScrollTo(self.state.pos)
    }
}
