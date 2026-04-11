mod layout_item;

use minlin::{Infinity, Rect, RectExt, Vec2};

use crate::{
    Element, LayoutBounds, LayoutFlags, LayoutParams, Orientation, RelPos,
    RelPosSrc, Size, Space, Widget, WidgetExt, event::Event, get_pos, layout,
    reposition, update_rel_pos,
};

pub use self::layout_item::*;

#[derive(Debug)]
pub struct Layout<W> {
    pub children: Vec<LayoutItem<W>>,
    pub orientation: Orientation,
    pub spacing: f32,
    obounds: Vec<f32>,
    size: Option<Vec2<f32>>,
    rel_pos: Option<RelPosSrc>,
    bounds: Rect<f32>,
}

impl<W> Layout<W> {
    pub fn new(orientation: impl Into<Orientation>) -> Self {
        Self {
            children: vec![],
            orientation: orientation.into(),
            spacing: 0.,
            obounds: vec![],
            size: None,
            rel_pos: None,
            bounds: Rect::default(),
        }
    }

    pub fn horizontal() -> Self {
        Self::new(Orientation::Horizontal)
    }

    pub fn vertical() -> Self {
        Self::new(Orientation::Vertical)
    }

    pub fn from_top() -> Self {
        Self::vertical()
    }

    pub fn from_left() -> Self {
        Self::vertical()
    }

    pub fn add(&mut self, space: impl Into<Space>, child: W) -> &mut Self {
        self.children.push(LayoutItem::new(space, child));
        self
    }

    pub fn add_auto(&mut self, child: W) -> &mut Self {
        self.add(Space::Auto, child)
    }

    pub fn add_rel(&mut self, size: f32, child: W) -> &mut Self {
        self.add(Size::Relative(size), child)
    }

    pub fn add_abs(&mut self, size: f32, child: W) -> &mut Self {
        self.add(Size::Absolute(size), child)
    }

    pub fn add_unit(&mut self, child: W) -> &mut Self {
        self.add_rel(1., child)
    }
}

impl<W> Default for Layout<W> {
    fn default() -> Self {
        Self {
            children: Default::default(),
            orientation: Orientation::Horizontal,
            spacing: Default::default(),
            obounds: Default::default(),
            size: Default::default(),
            rel_pos: Default::default(),
            bounds: Default::default(),
        }
    }
}

impl<W, Rend, Msg, Evt, Theme> Widget<Rend, Msg, Evt, Theme> for Layout<W>
where
    W: Widget<Rend, Msg, Evt, Theme>,
    Evt: Event,
{
    fn layout(
        &mut self,
        lp: &mut crate::LayoutParams<'_, Rend, Msg, Evt, Theme>,
        bounds: &crate::LayoutBounds,
        pos_base: crate::RelPos,
        flags: crate::LayoutFlags,
    ) -> Rect<f32> {
        if self.children.is_empty() {
            self.bounds = Rect::default();
            return self.bounds;
        }

        self.bounds = bounds.best_max();
        let pos_base =
            update_rel_pos(&mut self.rel_pos, pos_base, self.bounds.pos());
        self.bounds.set_pos(Vec2::ZERO);

        let best = (self.orientation.component(self.bounds.size())
            - self.spacing * (self.children.len() - 1) as f32)
            .max(0.);
        let sizes = self
            .children
            .iter_mut()
            .map(|a| a.size(lp.theme, self.orientation));
        if !flags.contains(LayoutFlags::WIDGET_MODIFIED)
            && let Some(s) = self.size
        {
            layout::update_align_sizes(best, sizes, &mut self.obounds[1..], s);
        } else {
            self.obounds.clear();
            self.obounds.push(0.);
            let s =
                layout::align_sizes_no_clone(best, sizes, &mut self.obounds);
            self.size = Some(s);
        }

        match self.orientation {
            Orientation::Horizontal => {
                self.layout_horizontal(lp, pos_base, flags)
            }
            Orientation::Vertical => self.layout_vertical(lp, pos_base, flags),
        }

        self.bounds
    }

    fn size(&mut self, _: &Theme) -> Vec2<f32> {
        Vec2::INFINITY
    }

    fn reposition(&mut self, _: &Theme, pos: Vec2<f32>) {
        reposition(&self.rel_pos, pos);
    }

    fn event(
        &mut self,
        shell: &mut crate::Shell<Rend, Msg, Evt, Theme>,
        theme: &Theme,
        event: &crate::event::EventInfo<Evt>,
    ) -> bool {
        let rp = get_pos(&mut self.rel_pos);
        let bounds = rp.position_rect(self.bounds);
        if !event.is_for(bounds) {
            return false;
        }

        // TODO: Binary serach for relevant children.
        self.children
            .iter_mut()
            .any(|c| c.widget.event(shell, theme, event))
    }

    fn draw(
        &mut self,
        shell: &mut crate::Shell<Rend, Msg, Evt, Theme>,
        theme: &Theme,
        renderer: &mut Rend,
    ) {
        let _rp = get_pos(&mut self.rel_pos);
        for c in &mut self.children {
            c.widget.draw(shell, theme, renderer);
        }
    }
}

impl<W> Layout<W> {
    fn layout_horizontal<Rend, Msg, Evt, Theme>(
        &mut self,
        lp: &mut LayoutParams<'_, Rend, Msg, Evt, Theme>,
        pos_base: RelPos,
        flags: LayoutFlags,
    ) where
        W: Widget<Rend, Msg, Evt, Theme>,
    {
        let mut max_height: f32 = 0.;
        for (i, c) in self.children.iter_mut().enumerate() {
            let off = i as f32 * self.spacing;
            let l = self.obounds[i] + off;
            let r = self.obounds[i + 1] + off;
            let bounds = LayoutBounds::at_most(Rect::new(
                l,
                self.bounds.y,
                r - l,
                self.bounds.height(),
            ));
            let cb = c.widget.layout(lp, &bounds, pos_base.clone(), flags);
            max_height = max_height.max(cb.height());
        }
        self.bounds.set_height(max_height);
    }

    fn layout_vertical<Rend, Msg, Evt, Theme>(
        &mut self,
        lp: &mut LayoutParams<'_, Rend, Msg, Evt, Theme>,
        pos_base: RelPos,
        flags: LayoutFlags,
    ) where
        W: Widget<Rend, Msg, Evt, Theme>,
    {
        let mut max_width: f32 = 0.;
        for (i, c) in self.children.iter_mut().enumerate() {
            let off = i as f32 * self.spacing;
            let t = self.obounds[i] + off;
            let b = self.obounds[i + 1] + off;
            let bounds = LayoutBounds::at_most(Rect::new(
                self.bounds.x,
                t,
                self.bounds.width(),
                b - t,
            ));
            let cb = c.widget.layout(lp, &bounds, pos_base.clone(), flags);
            max_width = max_width.max(cb.width());
        }
        self.bounds.set_width(max_width);
    }
}

impl<W, Rend, Msg, Evt, Theme> From<Layout<W>>
    for Element<Rend, Msg, Evt, Theme>
where
    W: Widget<Rend, Msg, Evt, Theme> + 'static,
    Evt: Event,
{
    fn from(value: Layout<W>) -> Self {
        Self::new(value)
    }
}

impl<W> WidgetExt for Layout<W> {}
