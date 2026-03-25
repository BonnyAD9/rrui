use std::fmt::Debug;

use minlin::Rect;

use crate::{Element, Shell, Widget};

pub struct Nothing;

impl<Rend, Msg, Evt: Debug, Theme> Widget<Rend, Msg, Evt, Theme> for Nothing {
    fn layout(
        &mut self,
        _: &mut Shell,
        _: &Theme,
        bounds: &crate::LayoutBounds,
    ) -> Rect<f32> {
        bounds.best_min()
    }

    fn event(&mut self, _: &mut Shell, _: &Theme, _: &Evt) {}

    fn draw(&mut self, _: &mut Shell, _: &Theme, _: &mut Rend) {}
}

impl<Rend, Msg, Evt: Debug, Theme> From<Nothing>
    for Element<Rend, Msg, Evt, Theme>
{
    fn from(value: Nothing) -> Self {
        Self::new(value)
    }
}
