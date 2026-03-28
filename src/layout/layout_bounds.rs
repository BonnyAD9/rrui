use minlin::{CompArithm, Padding, Rect, RectExt, Vec2};

use crate::LayoutSize;

#[derive(Debug, Copy, Clone)]
pub struct LayoutBounds {
    pub pos: Vec2<f32>,
    pub size: LayoutSize,
}

impl LayoutBounds {
    pub fn new(
        pos: impl Into<Vec2<f32>>,
        size: impl Into<LayoutSize>,
    ) -> Self {
        Self {
            pos: pos.into(),
            size: size.into(),
        }
    }

    pub fn at_most(value: impl Into<Rect<f32>>) -> Self {
        let v = value.into();
        Self::new(v.pos(), LayoutSize::at_most(v.size()))
    }

    pub fn exact(bounds: impl Into<Rect<f32>>) -> Self {
        let b = bounds.into();
        Self::new(b.pos(), LayoutSize::exactly(b.size()))
    }

    pub fn pad(&mut self, padding: impl Into<Padding<f32>>) {
        let p = padding.into();
        self.pos.x += p.x.min(self.size.range.right());
        self.pos.y += p.y.min(self.size.range.bottom());
        self.size.shrink_by(p.size());
    }

    pub fn padded(&self, padding: impl Into<Padding<f32>>) -> Self {
        let mut s = *self;
        s.pad(padding);
        s
    }

    pub fn unite(&self, size: &LayoutSize) -> Self {
        Self::new(self.pos, self.size.unite(size))
    }

    pub fn best_min(&self) -> Rect<f32> {
        Rect::from_pos_size(self.pos, self.size.best_min())
    }

    pub fn best_max(&self) -> Rect<f32> {
        Rect::from_pos_size(self.pos, self.size.best_max())
    }

    pub fn best_at_least(&self, v: impl Into<Vec2<f32>>) -> Rect<f32> {
        Rect::from_pos_size(self.pos, self.size.best_at_least(v))
    }

    pub fn shrink(&mut self) {
        self.size.shrink();
    }

    pub fn fill(&mut self) {
        self.size.fill();
    }

    pub fn fillx(&mut self) {
        self.size.fillx();
    }

    pub fn filly(&mut self) {
        self.size.filly()
    }

    pub fn clamp(&self, v: impl Into<Vec2<f32>>) -> Rect<f32> {
        Rect::from_pos_size(self.pos, self.size.clamp(v))
    }

    pub fn filling(bounds: Rect<f32>) -> Self {
        Self::new(bounds.pos(), LayoutSize::filling(bounds.size()))
    }

    pub fn extend_rect_within(
        &self,
        rect: Rect<f32>,
        pad: impl Into<Padding<f32>>,
    ) -> Rect<f32> {
        let p = pad.into();
        let tl = rect.top_left() - self.pos;
        let br = self.pos + self.size.max() - rect.bot_right();
        rect - Padding(
            p.cjoin((tl.x, tl.y, br.x, br.y), |a, b| a.min(b).max(0.)),
        )
    }

    pub fn max_right(&self) -> f32 {
        self.pos.x + self.size.range.z
    }

    pub fn max_bot(&self) -> f32 {
        self.pos.y + self.size.range.w
    }

    pub fn shrink_top(&mut self, amt: f32) {
        self.pos.y += self.size.shrink_tb(amt);
    }

    pub fn shrink_left(&mut self, amt: f32) {
        self.pos.x += self.size.shrink_lr(amt);
    }

    pub fn shrink_right(&mut self, amt: f32) {
        self.size.shrink_lr(amt);
    }

    pub fn shrink_bot(&mut self, amt: f32) {
        self.size.shrink_tb(amt);
    }
}

impl From<Rect<f32>> for LayoutBounds {
    fn from(value: Rect<f32>) -> Self {
        Self::new(value.pos(), value.size())
    }
}
