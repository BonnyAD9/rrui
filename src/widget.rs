use minlin::{Rect, Vec2};

use crate::{LayoutBounds, LayoutParams, RelPos, Shell, event::EventInfo};

pub trait Widget<Rend, Msg, Evt, Theme> {
    fn layout(
        &mut self,
        lp: &mut LayoutParams<'_, Rend, Msg, Theme>,
        bounds: &LayoutBounds,
        pos_base: RelPos,
    ) -> Rect<f32>;

    fn size(&mut self, theme: &Theme) -> Vec2<f32>;

    fn reposition(&mut self, theme: &Theme, pos: Vec2<f32>);

    fn event(
        &mut self,
        shell: &mut Shell<Msg>,
        theme: &Theme,
        event: &EventInfo<Evt>,
    ) -> bool;

    fn draw(
        &mut self,
        shell: &mut Shell<Msg>,
        theme: &Theme,
        renderer: &mut Rend,
    );
}
