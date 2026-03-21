use minlin::Rect;

use crate::{Background, Border, Quad};

pub trait QuadRenderer {
    fn draw_quad(&mut self, quad: &Quad, bg: impl Into<Background>);

    fn draw_rect(
        &mut self,
        rect: impl Into<Rect<f32>>,
        bg: impl Into<Background>,
    ) {
        self.draw_quad(&rect.into().into(), bg);
    }

    fn draw_border(
        &mut self,
        bounds: impl Into<Rect<f32>>,
        border: impl Into<Border>,
        bg: impl Into<Background>,
    ) {
        self.draw_quad(&Quad::border(bounds, border), bg);
    }
}
