use std::fmt::Debug;

use minlin::Rect;

use crate::{Element, Shell, Widget};

pub struct Nothing;

impl<Rend, Msg, Evt: Debug> Widget<Rend, Msg, Evt> for Nothing {
    fn layout(
        &mut self,
        _: &mut Shell,
        bounds: &crate::LayoutBounds,
    ) -> Rect<f32> {
        bounds.best_min()
    }

    fn event(&mut self, _: &mut Shell, _: &Evt) {}

    fn draw(&mut self, _: &mut Shell, _: &mut Rend) {}
}

impl<Rend, Msg, Evt: Debug> From<Nothing> for Element<Rend, Msg, Evt> {
    fn from(value: Nothing) -> Self {
        Self::new(value)
    }
}
