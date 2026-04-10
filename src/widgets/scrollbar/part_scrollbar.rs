use minlin::{Rect, RectExt, Vec2};

use crate::{
    LayoutBounds, Orientation, RedrawSlot, Shell,
    event::{Event, EventInfo, EventKind, MouseRelation, ScrollDelta},
    theme,
    widgets::{
        ButtonEvent, ButtonTheme, PartButton, PartThumb, PartTrack,
        ScrollbarEvent, ScrollbarTheme, ThumbEvent, ThumbState, ThumbTheme,
        TrackEvent, TrackTheme, scrollbar::scrollbar_style::ScrollbarStyle,
    },
};

#[derive(Debug)]
pub struct PartScrollbar<Style, BStyle, TrackStyle, ThumbStyle>
where
    Style: ScrollbarStyle<BStyle, TrackStyle, ThumbStyle>,
{
    pub line_size: Option<f32>,
    style: Style,
    // TODO: relayout with pos length and view.
    // CONSIDER: don't save the bounds for each element but rather calculate on
    // demand.
    pos: f32,
    length: f32,
    view: f32,
    start_button: PartButton<BStyle>,
    end_button: PartButton<BStyle>,
    thumb: PartThumb<ThumbStyle>,
    start_track: PartTrack<TrackStyle>,
    end_track: PartTrack<TrackStyle>,
    bounds: Rect<f32>,
}

impl<BStyle, TrackStyle, ThumbStyle, Style>
    PartScrollbar<Style, BStyle, TrackStyle, ThumbStyle>
where
    Style: ScrollbarStyle<BStyle, TrackStyle, ThumbStyle>,
{
    pub fn styled(style: Style) -> Self {
        Self {
            line_size: None,
            start_button: PartButton::styled(style.button_style().into()),
            end_button: PartButton::styled(style.button_style().into()),
            pos: 0.,
            length: 0.,
            view: 500.,
            thumb: PartThumb::new(style.thumb_style(), Orientation::Vertical),
            start_track: PartTrack::new(
                style.track_style(),
                Orientation::Vertical,
            ),
            end_track: PartTrack::new(
                style.track_style(),
                Orientation::Vertical,
            ),
            bounds: Rect::default(),
            style: style,
        }
    }

    pub fn set_orientation(&mut self, orientation: Orientation) {
        self.thumb.orientation = orientation;
        self.start_track.orientation = orientation;
        self.end_track.orientation = orientation;
    }

    pub fn orientation(&self) -> Orientation {
        self.thumb.orientation
    }

    pub fn osize(&self) -> f32 {
        self.thumb.orientation.other_component(self.bounds.size())
    }

    pub fn bounds(&self) -> Rect<f32> {
        self.bounds
    }

    pub fn off_bounds(&self, off: Vec2<f32>) -> Rect<f32> {
        Rect::from_pos_size(self.bounds.pos() + off, self.bounds.size())
    }

    pub fn layout<Theme>(
        &mut self,
        theme: &Theme,
        bounds: &LayoutBounds,
    ) -> Rect<f32>
    where
        Theme: ScrollbarTheme<Style = Style>,
    {
        match self.thumb.orientation {
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

        let minh = sizes.button * 2. + sizes.min_thumb;
        let (buth, trackh) = if self.bounds.height() < minh {
            let h = minh / 3.;
            (h, h)
        } else {
            (sizes.button, self.bounds.height() * sizes.button * 2.)
        };

        let but_size = Vec2::new(w, buth);
        self.start_button.size = Some(but_size);
        self.end_button.size = Some(but_size);

        let thumbh = (trackh * self.view / self.length)
            .max(sizes.min_thumb)
            .min(trackh);
        let toph = self.pos / self.length * (trackh - thumbh);
        let both = trackh - thumbh - toph;

        let start_but = Rect::from_pos_size(self.bounds.pos(), but_size)
            + sizes.button_padding;
        self.start_button.layout_direct(start_but);
        let mut top = self.bounds.y + but_size.y;

        self.start_track.bounds =
            Rect::new(self.bounds.x, top, self.bounds.width(), toph);
        top += toph;

        let thumb = Rect::new(self.bounds.x, top, self.bounds.width(), thumbh);
        let tmin = self.bounds.y + but_size.y;
        let tmax = tmin + toph + both;
        self.thumb.layout_direct(thumb, tmin..tmax);
        top += thumbh;

        self.end_track.bounds =
            Rect::new(self.bounds.x, top, self.bounds.width(), both);
        top += both;

        let end_but = Rect::from_pos_size([self.bounds.x, top], but_size)
            + sizes.button_padding;
        self.start_button.layout_direct(end_but);

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
        todo!()
    }

    pub fn size<Theme>(&self, theme: &Theme) -> Vec2<f32>
    where
        Theme: ScrollbarTheme<Style = Style>,
    {
        let s = theme.sizes(&self.style).size;
        match self.thumb.orientation {
            Orientation::Horizontal => Vec2::new(f32::INFINITY, s),
            Orientation::Vertical => Vec2::new(s, f32::INFINITY),
        }
    }

    pub fn reposition(&mut self, pos: Vec2<f32>) {
        let change = pos - self.bounds.pos();
        self.bounds.set_pos(pos);

        self.start_button.reposition_direct(pos);
        self.start_track
            .reposition(self.start_track.bounds.pos() + change);
        self.thumb.reposition_by(change);
        self.end_track
            .reposition(self.start_track.bounds.pos() + change);
        self.end_button.reposition_direct(pos);
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
        Theme: ThumbTheme<Style = ThumbStyle>,
        Theme: TrackTheme<Style = TrackStyle>,
        Theme: ButtonTheme<Style = BStyle>,
    {
        let bounds = self.off_bounds(off);

        match event.mouse_relate_to(bounds) {
            MouseRelation::None | MouseRelation::Elswhere => {
                return (false, ScrollbarEvent::Nothing);
            }
            MouseRelation::Hover => {
                if let EventKind::MouseScroll(s) = event.get_kind() {
                    let evt = match s {
                        ScrollDelta::Lines(v) => {
                            self.scroll_by(v.x * self.line_size.unwrap_or(20.))
                        }
                        ScrollDelta::Pixels(v) => self.scroll_by(v.x),
                    };
                    return (true, evt);
                }
            }
            _ => {}
        }

        let (handled, evt) = self.thumb.event(off, shell, theme, event);
        if let ThumbEvent::Move(p) = evt {
            self.relayout_with_pos(p);
            return (true, ScrollbarEvent::ScrollTo(self.pos));
        }
        if handled {
            return (true, ScrollbarEvent::Nothing);
        }

        let (handled, evt) = self.start_track.event(off, shell, theme, event);
        if let TrackEvent::PressAt(p) = evt {
            self.thumb.drag_from(p, shell, theme);
            return (true, self.relayout_with_pos(p));
        }
        if handled {
            return (true, ScrollbarEvent::Nothing);
        }

        let (handled, evt) = self.end_track.event(off, shell, theme, event);
        if let TrackEvent::PressAt(mut p) = evt {
            p -= self.thumb.size();
            self.thumb.drag_from(p, shell, theme);
            return (true, self.relayout_with_pos(p));
        }
        if handled {
            return (true, ScrollbarEvent::Nothing);
        }

        let (handled, evt) =
            self.start_button.event_direct(off, shell, theme, event);
        if matches!(evt, ButtonEvent::Clicked(_)) {
            return (true, self.scroll_by(-self.view));
        }
        if handled {
            return (true, ScrollbarEvent::Nothing);
        }

        let (handled, evt) =
            self.end_button.event_direct(off, shell, theme, event);
        if matches!(evt, ButtonEvent::Clicked(_)) {
            return (true, self.scroll_by(-self.view));
        }

        (handled, ScrollbarEvent::Nothing)
    }

    fn relayout_with_pos(&mut self, p: f32) -> ScrollbarEvent {
        let rel = match self.thumb.orientation {
            Orientation::Horizontal => {
                let pos = p - self.start_track.bounds.x;
                self.start_track.bounds.set_width(pos);
                let es = p + self.thumb.bounds().width();
                self.end_track
                    .bounds
                    .set_width(self.end_track.bounds.right() - es);
                self.end_track.bounds.x = es;
                pos / (self.start_track.bounds.width()
                    + self.end_track.bounds.width())
            }
            Orientation::Vertical => {
                let pos = p - self.start_track.bounds.y;
                self.start_track.bounds.set_height(pos);
                let es = p + self.thumb.bounds().height();
                self.end_track
                    .bounds
                    .set_height(self.end_track.bounds.bottom() - es);
                self.end_track.bounds.y = es;
                pos / (self.start_track.bounds.height()
                    + self.end_track.bounds.height())
            }
        };

        let new_pos = rel * self.length;
        let evt = if new_pos != self.pos {
            ScrollbarEvent::ScrollTo(new_pos)
        } else {
            ScrollbarEvent::Nothing
        };
        self.pos = new_pos;
        evt
    }

    pub fn scroll_by(&mut self, amt: f32) -> ScrollbarEvent {
        let abs_pos = (self.pos - amt).min(self.length).max(0.);
        if abs_pos == self.pos {
            return ScrollbarEvent::Nothing;
        }
        self.pos = abs_pos;

        let rel = self.pos / self.length;

        match self.thumb.orientation {
            Orientation::Horizontal => {
                let p = rel
                    * (self.start_track.bounds.width()
                        + self.end_track.bounds.width());
                self.thumb.move_to(p);
                let pos = p - self.start_track.bounds.x;
                self.start_track.bounds.set_width(pos);
                let es = p + self.thumb.bounds().width();
                self.end_track
                    .bounds
                    .set_width(self.end_track.bounds.right() - es);
                self.end_track.bounds.x = es;
            }
            Orientation::Vertical => {
                let p = rel
                    * (self.start_track.bounds.height()
                        + self.end_track.bounds.height());
                self.thumb.move_to(p);
                let pos = p - self.start_track.bounds.y;
                self.start_track.bounds.set_height(pos);
                let es = p + self.thumb.bounds().height();
                self.end_track
                    .bounds
                    .set_height(self.end_track.bounds.bottom() - es);
                self.end_track.bounds.y = es;
            }
        }

        ScrollbarEvent::ScrollTo(self.pos)
    }
}
