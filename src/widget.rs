use minlin::Rect;

use crate::{LayoutBounds, Shell};

pub trait Widget<Rend, Msg, Evt> {
    fn layout(
        &mut self,
        shell: &mut Shell,
        bounds: &LayoutBounds,
    ) -> Rect<f32>;

    fn event(&mut self, shell: &mut Shell, event: &Evt);

    fn draw(&mut self, shell: &mut Shell, renderer: &mut Rend);
}
