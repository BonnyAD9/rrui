use minlin::{Rect, Vec2};

use crate::{LayoutBounds, Shell};

pub trait Widget<Rend, Msg, Evt, Theme> {
    fn layout(
        &mut self,
        shell: &mut Shell,
        theme: &Theme,
        bounds: &LayoutBounds,
    ) -> Rect<f32>;

    fn size(&self) -> Vec2<f32>;

    fn reposition(&mut self, pos: Vec2<f32>);

    fn event(&mut self, shell: &mut Shell, theme: &Theme, event: &Evt);

    fn draw(&mut self, shell: &mut Shell, theme: &Theme, renderer: &mut Rend);
}
