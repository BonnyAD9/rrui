use minlin::{Rect, Vec2};

use crate::{LayoutBounds, Shell, Widget};

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
    fn layout(
        &mut self,
        shell: &mut Shell,
        theme: &Theme,
        bounds: &LayoutBounds,
        renderer: &Rend,
    ) -> Rect<f32> {
        self.0.layout(shell, theme, bounds, renderer)
    }

    fn event(&mut self, shell: &mut Shell, theme: &Theme, event: &Evt) {
        self.0.event(shell, theme, event);
    }

    fn draw(&mut self, shell: &mut Shell, theme: &Theme, renderer: &mut Rend) {
        self.0.draw(shell, theme, renderer);
    }

    fn size(&self, theme: &Theme) -> Vec2<f32> {
        self.0.size(theme)
    }

    fn reposition(&mut self, theme: &Theme, pos: Vec2<f32>) {
        self.0.reposition(theme, pos);
    }
}
