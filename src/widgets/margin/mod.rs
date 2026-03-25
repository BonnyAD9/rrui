mod margin_ext;
pub use self::margin_ext::*;

use minlin::{Padding, Rect, Vec2};

use crate::{LayoutBounds, Shell, Widget};

#[derive(Debug, Clone, Copy, Default)]
pub struct Margin<W> {
    pub margin: Padding<f32>,
    pub child: W,
}

impl<W> Margin<W> {
    pub fn new(margin: impl Into<Padding<f32>>, child: W) -> Self {
        Self {
            margin: margin.into(),
            child,
        }
    }

    pub fn marge(&mut self, additional: impl Into<Padding<f32>>) -> &mut Self {
        self.margin += additional.into();
        self
    }

    pub fn margin(&mut self, total: impl Into<Padding<f32>>) -> &mut Self {
        self.margin = total.into();
        self
    }
}

impl<W, Rend, Msg, Evt, Theme> Widget<Rend, Msg, Evt, Theme> for Margin<W>
where
    W: Widget<Rend, Msg, Evt, Theme>,
{
    fn layout(
        &mut self,
        shell: &mut Shell,
        theme: &Theme,
        bounds: &LayoutBounds,
    ) -> Rect<f32> {
        self.child.layout(shell, theme, &bounds.padded(self.margin))
            - self.margin
    }

    fn size(&self, theme: &Theme) -> Vec2<f32> {
        self.child.size(theme) + self.margin.size()
    }

    fn reposition(&mut self, theme: &Theme, pos: Vec2<f32>) {
        self.child.reposition(theme, pos + self.margin.offset());
    }

    fn event(&mut self, shell: &mut crate::Shell, theme: &Theme, event: &Evt) {
        self.child.event(shell, theme, event)
    }

    fn draw(
        &mut self,
        shell: &mut crate::Shell,
        theme: &Theme,
        renderer: &mut Rend,
    ) {
        self.child.draw(shell, theme, renderer)
    }
}
