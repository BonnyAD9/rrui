use minlin::{CompArithm, Rect, RectExt, Vec2};

#[derive(Debug, Clone, Copy)]
pub struct LayoutSize {
    pub range: Rect<f32>,
    pub best: Option<Vec2<f32>>,
}

impl LayoutSize {
    pub const MIN: Vec2<f32> = Vec2::new(0., 0.);
    pub const MAX: Vec2<f32> = Vec2::new(f32::INFINITY, f32::INFINITY);

    pub fn new(range: impl Into<Rect<f32>>, best: Option<Vec2<f32>>) -> Self {
        Self {
            range: range.into(),
            best,
        }
    }

    pub fn no_pref(range: impl Into<Rect<f32>>) -> Self {
        Self::new(range, None)
    }

    pub fn at_most(max: Vec2<f32>) -> Self {
        Self::new(Self::MIN..max, None)
    }

    pub fn at_least(min: Vec2<f32>) -> Self {
        Self::new(min..Self::MAX, None)
    }

    pub fn exactly(size: Vec2<f32>) -> Self {
        Self::new(size..size, Some(size))
    }

    pub fn min(&self) -> Vec2<f32> {
        self.range.top_left()
    }

    pub fn max(&self) -> Vec2<f32> {
        self.range.bot_right()
    }

    pub fn best_min(&self) -> Vec2<f32> {
        self.best.unwrap_or_else(|| self.min())
    }

    pub fn best_max(&self) -> Vec2<f32> {
        self.best.unwrap_or_else(|| self.max())
    }

    pub fn best_at_least(&self, v: impl Into<Vec2<f32>>) -> Vec2<f32> {
        let v = self.range.clamp(v);
        self.best.map(|a| a.cjoin(v, |a, v| a.max(v))).unwrap_or(v)
    }

    pub fn best(&self, fallback: impl Into<Vec2<f32>>) -> Vec2<f32> {
        self.best
            .unwrap_or_else(|| self.range.clamp(fallback.into()))
    }

    pub fn clamp(&self, v: impl Into<Vec2<f32>>) -> Vec2<f32> {
        self.range.clamp(v)
    }

    pub fn unite(&self, other: &Self) -> Self {
        let range = self.range.intersect(other.range);
        let best = self.best.or(other.best).map(|a| range.clamp(a));
        Self::new(range, best)
    }

    pub fn shrink_by(&mut self, v: impl Into<Vec2<f32>>) {
        let v = v.into();
        self.range.x = (self.range.x - v.x).max(0.);
        self.range.y = (self.range.y - v.y).max(0.);
        self.range.z = (self.range.z - v.x).max(0.);
        self.range.w = (self.range.w - v.y).max(0.);
        if let Some(b) = &mut self.best {
            *b = b.cjoin(v, |a, b| (a - b).max(0.));
        }
    }

    pub fn shrink_tb(&mut self, amt: f32) -> f32 {
        self.range.y = (self.range.y - amt).max(0.);
        let neww = (self.range.w - amt).max(0.);
        let res = self.range.w - neww;
        self.range.w = neww;
        res
    }

    pub fn shrink_lr(&mut self, amt: f32) -> f32 {
        self.range.x = (self.range.x - amt).max(0.);
        let newz = (self.range.z - amt).max(0.);
        let res = self.range.z - newz;
        self.range.z = newz;
        res
    }

    pub fn shrink(&mut self) {
        self.best = Some(self.min());
    }

    pub fn fill(&mut self) {
        self.best = Some(self.max());
    }

    pub fn fillx(&mut self) {
        self.best = Some(self.range.top_right());
    }

    pub fn filly(&mut self) {
        self.best = Some(self.range.bot_left());
    }

    pub fn filling(size: Vec2<f32>) -> Self {
        Self::new(Self::MIN..size, Some(size))
    }
}

impl From<Vec2<f32>> for LayoutSize {
    fn from(value: Vec2<f32>) -> Self {
        Self::new(Self::MIN..value, None)
    }
}
