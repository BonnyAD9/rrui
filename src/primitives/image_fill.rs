use minlin::{Rect, RectExt, Vec2};

use crate::Align;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFill {
    Strech,
    Fill(Vec2<Align>),
    Fit(Vec2<Align>),
}

impl ImageFill {
    pub fn fill(a: impl Into<Vec2<Align>>) -> Self {
        Self::Fill(a.into())
    }

    pub fn fit(a: impl Into<Vec2<Align>>) -> Self {
        Self::Fit(a.into())
    }

    pub const fn fill_center() -> Self {
        Self::Fill(Vec2::new(Align::Center, Align::Center))
    }

    pub const fn fit_center() -> Self {
        Self::Fit(Vec2::new(Align::Center, Align::Center))
    }

    pub fn with_align(self, align: impl Into<Vec2<Align>>) -> Self {
        match self {
            ImageFill::Strech => ImageFill::Strech,
            ImageFill::Fill(_) => ImageFill::Fill(align.into()),
            ImageFill::Fit(_) => ImageFill::Fit(align.into()),
        }
    }

    pub fn with_fill(self) -> Self {
        match self {
            Self::Strech => Self::fill_center(),
            Self::Fill(_) => self,
            Self::Fit(a) => Self::Fill(a),
        }
    }

    pub fn with_fit(self) -> Self {
        match self {
            Self::Strech => Self::fill_center(),
            Self::Fill(a) => Self::Fit(a),
            Self::Fit(_) => self,
        }
    }

    pub fn calculate(
        &self,
        bounds: impl Into<Rect<f32>>,
        img_size: impl Into<Vec2<f32>>,
    ) -> Rect<f32> {
        let bounds = bounds.into();

        let (is_fit, align) = match self {
            ImageFill::Strech => return bounds,
            ImageFill::Fill(a) => (false, a),
            ImageFill::Fit(a) => (true, a),
        };
        let img_size = img_size.into();

        let brat = bounds.size().quot();
        let irat = img_size.quot();

        if (brat > irat) ^ is_fit {
            // overlap at top/bottom (fill)
            // empty space at left/right (fit)
            let is = Vec2::new(bounds.width(), bounds.width() / irat);
            let top = align.y.calculate_start(bounds.yrange(), is.y);
            Rect::from_pos_size([bounds.x, top], is)
        } else {
            // overlap at left/right (fill)
            // empty space at top/bottom (fit)
            let is = Vec2::new(bounds.height() * irat, bounds.height());
            let left = align.x.calculate_start(bounds.xrange(), is.x);
            Rect::from_pos_size([left, bounds.y], is)
        }
    }
}

impl Default for ImageFill {
    fn default() -> Self {
        Self::fit_center()
    }
}
