mod grid_item;

use minlin::{Infinity, Rect, RectExt, Vec2};

use crate::{
    Element, LayoutBounds, LayoutFlags, RelPosSrc, Size, Widget, WidgetExt,
    event::Event, layout, update_rel_pos,
};

pub use self::grid_item::*;

#[derive(Debug)]
pub struct Grid<W> {
    pub children: Vec<GridItem<W>>,
    pub xdef: Vec<Size>,
    pub ydef: Vec<Size>,
    xbounds: Vec<f32>,
    ybounds: Vec<f32>,
    size: Option<Vec2<Vec2<f32>>>,
    rel_pos: Option<RelPosSrc>,
    bounds: Rect<f32>,
}

impl<W> Grid<W> {
    pub fn new(
        xdef: impl Into<Vec<Size>>,
        ydef: impl Into<Vec<Size>>,
    ) -> Self {
        Self {
            children: vec![],
            xdef: xdef.into(),
            ydef: ydef.into(),
            xbounds: vec![],
            ybounds: vec![],
            size: None,
            rel_pos: None,
            bounds: Rect::default(),
        }
    }

    pub fn add_x(&mut self, width: impl Into<Size>) -> &mut Self {
        self.xdef.push(width.into());
        self
    }

    pub fn add_y(&mut self, height: impl Into<Size>) -> &mut Self {
        self.ydef.push(height.into());
        self
    }

    pub fn add(&mut self, pos: impl Into<Vec2>, child: W) -> &mut Self {
        self.children.push(GridItem::new(pos, child));
        self
    }
}

impl<W> Default for Grid<W> {
    fn default() -> Self {
        Self {
            children: Default::default(),
            xdef: Default::default(),
            ydef: Default::default(),
            xbounds: Default::default(),
            ybounds: Default::default(),
            size: Default::default(),
            rel_pos: Default::default(),
            bounds: Default::default(),
        }
    }
}

impl<W, Rend, Msg, Evt, Theme> Widget<Rend, Msg, Evt, Theme> for Grid<W>
where
    W: Widget<Rend, Msg, Evt, Theme>,
    Evt: Event,
{
    fn layout(
        &mut self,
        lp: &mut crate::LayoutParams<'_, Rend, Msg, Theme>,
        bounds: &crate::LayoutBounds,
        pos_base: crate::RelPos,
        flags: crate::LayoutFlags,
    ) -> minlin::Rect<f32> {
        self.bounds = bounds.best_max();
        let pos_base =
            update_rel_pos(&mut self.rel_pos, pos_base, self.bounds.pos());

        if !flags.contains(LayoutFlags::WIDGET_MODIFIED)
            && let Some(s) = self.size
        {
            Self::update_lay(
                self.bounds.width(),
                &self.xdef,
                &mut self.xbounds,
                s.x,
            );
            Self::update_lay(
                self.bounds.height(),
                &self.ydef,
                &mut self.ybounds,
                s.y,
            );
        } else {
            let sx = Self::relay(
                self.bounds.width(),
                &self.xdef,
                &mut self.xbounds,
            );
            let sy = Self::relay(
                self.bounds.height(),
                &self.ydef,
                &mut self.ybounds,
            );
            self.size = Some(Vec2::new(sx, sy));
        }

        let size = self.grid_size();
        for c in &mut self.children {
            if !size.size_contains(c.pos) {
                continue;
            }
            let tl = Vec2::new(self.xbounds[c.pos.x], self.ybounds[c.pos.y]);
            let br = Vec2::new(
                self.xbounds[c.pos.x + 1],
                self.ybounds[c.pos.y + 1],
            );
            let bounds = LayoutBounds::at_most(tl..br);
            c.widget.layout(lp, &bounds, pos_base.clone(), flags);
        }

        self.bounds
    }

    fn size(&mut self, _: &Theme) -> Vec2<f32> {
        Vec2::INFINITY
    }

    fn reposition(&mut self, _: &Theme, pos: Vec2<f32>) {
        if let Some(rp) = &self.rel_pos {
            rp.move_to(pos);
        }
    }

    fn event(
        &mut self,
        shell: &mut crate::Shell<Msg>,
        theme: &Theme,
        event: &crate::event::EventInfo<Evt>,
    ) -> bool {
        {
            let rp = self.rel_pos.as_mut().unwrap().get();
            let bounds = rp.position_rect(self.bounds);
            if !event.is_for(bounds) {
                return false;
            }
        }

        // TODO: try only childern in touching grid cells
        let size = self.grid_size();
        self.children.iter_mut().any(|c| {
            size.size_contains(c.pos) && c.widget.event(shell, theme, event)
        })
    }

    fn draw(
        &mut self,
        shell: &mut crate::Shell<Msg>,
        theme: &Theme,
        renderer: &mut Rend,
    ) {
        let size = self.grid_size();
        for c in &mut self.children {
            if size.size_contains(c.pos) {
                c.widget.draw(shell, theme, renderer);
            }
        }
    }
}

impl<W> Grid<W> {
    fn grid_size(&self) -> Vec2 {
        Vec2::new(self.xdef.len().max(1), self.ydef.len().max(1))
    }

    fn update_lay(
        best: f32,
        def: &[Size],
        bounds: &mut [f32],
        total: Vec2<f32>,
    ) {
        if def.is_empty() {
            bounds[1] = best;
        } else {
            layout::update_align_sizes(best, def, &mut bounds[1..], total);
        }
    }

    fn relay(best: f32, def: &[Size], bounds: &mut Vec<f32>) -> Vec2<f32> {
        bounds.clear();
        bounds.push(0.);
        if def.is_empty() {
            bounds.push(best);
            Vec2::new(0., 1.)
        } else {
            layout::align_sizes(best, def, bounds)
        }
    }
}

impl<W, Rend, Msg, Evt, Theme> From<Grid<W>> for Element<Rend, Msg, Evt, Theme>
where
    W: Widget<Rend, Msg, Evt, Theme> + 'static,
    Evt: Event,
{
    fn from(value: Grid<W>) -> Self {
        Element::new(value)
    }
}

impl<W> WidgetExt for Grid<W> {}
