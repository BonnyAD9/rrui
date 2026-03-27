use minlin::{Rect, Vec2};

use crate::{LayoutBounds, Shell, event::EventInfo};

pub trait Widget<Rend, Msg, Evt, Theme> {
    fn layout(
        &mut self,
        shell: &mut Shell,
        theme: &Theme,
        bounds: &LayoutBounds,
        renderer: &Rend,
    ) -> Rect<f32>;

    fn size(&self, theme: &Theme) -> Vec2<f32>;

    fn reposition(&mut self, theme: &Theme, pos: Vec2<f32>);

    fn event(
        &mut self,
        shell: &mut Shell,
        theme: &Theme,
        event: &EventInfo<Evt>,
    ) -> bool;

    fn draw(&mut self, shell: &mut Shell, theme: &Theme, renderer: &mut Rend);
}
