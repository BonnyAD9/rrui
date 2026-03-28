use minlin::{Infinity, Rect, RectExt, Vec2};

use crate::{
    Direction, Element, LayoutParams, RelPos, RelPosSrc, Widget, WidgetExt,
    event::Event, layout,
};

#[derive(Debug)]
pub struct Stack<W> {
    pub children: Vec<W>,
    pub direction: Direction,
    pub spacing: f32,
    rel_pos: Option<RelPosSrc>,
    rel_off: Vec2<f32>,
    bounds: Rect<f32>,
}

impl<W> Stack<W> {
    pub fn from_direction(
        direction: impl Into<Direction>,
        children: impl Into<Vec<W>>,
    ) -> Self {
        Self {
            children: children.into(),
            direction: direction.into(),
            spacing: 0.,
            bounds: Rect::default(),
            rel_off: Vec2::default(),
            rel_pos: None,
        }
    }

    pub fn new(children: impl Into<Vec<W>>) -> Self {
        Self::from_direction(Direction::Top, children)
    }

    pub fn from_top(children: impl Into<Vec<W>>) -> Self {
        Self::from_direction(Direction::Top, children)
    }

    pub fn from_left(children: impl Into<Vec<W>>) -> Self {
        Self::from_direction(Direction::Left, children)
    }

    pub fn from_right(children: impl Into<Vec<W>>) -> Self {
        Self::from_direction(Direction::Right, children)
    }

    pub fn from_bottom(children: impl Into<Vec<W>>) -> Self {
        Self::from_direction(Direction::Bottom, children)
    }

    pub fn child(&mut self, child: W) -> &mut Self {
        self.children.push(child);
        self
    }

    pub fn children(
        &mut self,
        children: impl IntoIterator<Item = W>,
    ) -> &mut Self {
        self.children.extend(children);
        self
    }
}

impl<W> Default for Stack<W> {
    fn default() -> Self {
        Self {
            children: Default::default(),
            direction: Direction::Top,
            spacing: 0.,
            bounds: Default::default(),
            rel_off: Default::default(),
            rel_pos: None,
        }
    }
}

impl<W, Rend, Msg, Evt, Theme> Widget<Rend, Msg, Evt, Theme> for Stack<W>
where
    W: Widget<Rend, Msg, Evt, Theme>,
    Evt: Event,
{
    fn layout(
        &mut self,
        lp: &mut LayoutParams<'_, Rend, Msg, Theme>,
        bounds: &crate::LayoutBounds,
        rel_pos: RelPos,
    ) -> Rect<f32> {
        let rel_pos = self.update_rel_pos(rel_pos, bounds.pos);
        let mut rbounds = *bounds;
        rbounds.pos = Vec2::ZERO;

        self.bounds = layout::stack(
            &mut self.children,
            self.spacing,
            self.direction,
            lp,
            &rbounds,
            rel_pos,
        );

        if self.direction.is_from_end() {
            let fill_size = bounds.size.best_at_least(self.bounds.size());
            let dsize = fill_size - self.bounds.size();
            self.bounds.set_size(fill_size);
            self.rel_off = -self.bounds.pos() + dsize;

            self.rel_pos.as_mut().unwrap().move_by(self.rel_off);
        } else {
            self.rel_off = Vec2::ZERO;
        }
        Rect::from_pos_size(bounds.pos, self.bounds.size())
    }

    fn size(&self, _: &Theme) -> minlin::Vec2<f32> {
        Vec2::INFINITY
    }

    fn reposition(&mut self, _: &Theme, pos: minlin::Vec2<f32>) {
        if let Some(rp) = &self.rel_pos {
            rp.move_to(pos + self.rel_off);
        }
    }

    fn event(
        &mut self,
        shell: &mut crate::Shell<Msg>,
        theme: &Theme,
        event: &crate::event::EventInfo<Evt>,
    ) -> bool {
        let rp = self.rel_pos.as_mut().unwrap().get();
        let bounds = rp.position_rect(self.bounds);

        if !event.is_for(bounds) {
            return false;
        }

        // TODO: binary search on the correct child
        for c in &mut self.children {
            if c.event(shell, theme, event) {
                return true;
            }
        }

        false
    }

    fn draw(
        &mut self,
        shell: &mut crate::Shell<Msg>,
        theme: &Theme,
        renderer: &mut Rend,
    ) {
        let _rp = self.rel_pos.as_mut().unwrap().get();

        for c in &mut self.children {
            c.draw(shell, theme, renderer);
        }
    }
}

impl<W> Stack<W> {
    pub fn update_rel_pos(
        &mut self,
        rel_pos: RelPos,
        rel: Vec2<f32>,
    ) -> RelPos {
        if let Some(rp) = &mut self.rel_pos {
            rp.update(rel_pos);
            rp.move_to(rel);
            rp.rel_pos()
        } else {
            let rp = rel_pos.relate(rel);
            let res = rp.rel_pos();
            self.rel_pos = Some(rp);
            res
        }
    }
}

impl<W, Rend, Msg, Evt, Theme> From<Stack<W>>
    for Element<Rend, Msg, Evt, Theme>
where
    W: Widget<Rend, Msg, Evt, Theme> + 'static,
    Evt: Event,
{
    fn from(value: Stack<W>) -> Self {
        Element::new(value)
    }
}

impl<W> WidgetExt for Stack<W> {}
