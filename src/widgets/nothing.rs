use std::fmt::Debug;

use minlin::{Rect, Vec2};

use crate::{
    Element, LayoutFlags, LayoutParams, RelPos, Shell, Widget,
    event::EventInfo,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct Nothing;

impl<Rend, Msg, Evt: Debug, Theme> Widget<Rend, Msg, Evt, Theme> for Nothing {
    fn layout(
        &mut self,
        _: &mut LayoutParams<'_, Rend, Msg, Evt, Theme>,
        bounds: &crate::LayoutBounds,
        _: RelPos,
        _: LayoutFlags,
    ) -> Rect<f32> {
        bounds.best_min()
    }

    fn event(
        &mut self,
        _: &mut Shell<Rend, Msg, Evt, Theme>,
        _: &Theme,
        _: &EventInfo<Evt>,
    ) -> bool {
        false
    }

    fn draw(
        &mut self,
        _: &mut Shell<Rend, Msg, Evt, Theme>,
        _: &Theme,
        _: &mut Rend,
    ) {
    }

    fn size(&mut self, _: &Theme) -> Vec2<f32> {
        Vec2::ZERO
    }

    fn reposition(&mut self, _: &Theme, _: Vec2<f32>) {}
}

impl<Rend, Msg, Evt: Debug, Theme> From<Nothing>
    for Element<Rend, Msg, Evt, Theme>
{
    fn from(value: Nothing) -> Self {
        Self::new(value)
    }
}
