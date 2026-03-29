use minlin::{Rect, Vec2};

use crate::{
    LayoutBounds, LayoutParams, RelPos, Shell, Widget, event::EventInfo,
};

pub struct Element<Rend, Msg, Evt, Theme>(
    Box<dyn Widget<Rend, Msg, Evt, Theme>>,
);

impl<Rend, Msg, Evt, Theme> Element<Rend, Msg, Evt, Theme> {
    pub fn new(w: impl Widget<Rend, Msg, Evt, Theme> + 'static) -> Self {
        Self(Box::new(w))
    }
}

impl<Rend, Msg, Evt, Theme> Widget<Rend, Msg, Evt, Theme>
    for Element<Rend, Msg, Evt, Theme>
{
    fn init(&mut self) {
        self.0.init();
    }

    fn layout(
        &mut self,
        lp: &mut LayoutParams<'_, Rend, Msg, Theme>,
        bounds: &LayoutBounds,
        pos_base: RelPos,
    ) -> Rect<f32> {
        self.0.layout(lp, bounds, pos_base)
    }

    fn event(
        &mut self,
        shell: &mut Shell<Msg>,
        theme: &Theme,
        event: &EventInfo<Evt>,
    ) -> bool {
        self.0.event(shell, theme, event)
    }

    fn draw(
        &mut self,
        shell: &mut Shell<Msg>,
        theme: &Theme,
        renderer: &mut Rend,
    ) {
        self.0.draw(shell, theme, renderer);
    }

    fn size(&mut self, theme: &Theme) -> Vec2<f32> {
        self.0.size(theme)
    }

    fn reposition(&mut self, theme: &Theme, pos: Vec2<f32>) {
        self.0.reposition(theme, pos);
    }
}
