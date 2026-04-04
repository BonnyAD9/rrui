use crate::{Element, RefVariableOut, VariableAction, Widget, WidgetExt};

pub struct Variable<W>(RefVariableOut<W>);

impl<W> Variable<W> {
    pub fn new(child: RefVariableOut<W>) -> Self {
        child.on_change(VariableAction::Relayout);
        Self(child)
    }
}

impl<W, Rend, Msg, Evt, Theme> Widget<Rend, Msg, Evt, Theme> for Variable<W>
where
    W: Widget<Rend, Msg, Evt, Theme>,
{
    fn layout(
        &mut self,
        lp: &mut crate::LayoutParams<'_, Rend, Msg, Theme>,
        bounds: &crate::LayoutBounds,
        pos_base: crate::RelPos,
    ) -> minlin::Rect<f32> {
        self.0.borrow_mut().layout(lp, bounds, pos_base)
    }

    fn size(&mut self, theme: &Theme) -> minlin::Vec2<f32> {
        self.0.borrow_mut().size(theme)
    }

    fn reposition(&mut self, theme: &Theme, pos: minlin::Vec2<f32>) {
        self.0.borrow_mut().reposition(theme, pos);
    }

    fn event(
        &mut self,
        shell: &mut crate::Shell<Msg>,
        theme: &Theme,
        event: &crate::event::EventInfo<Evt>,
    ) -> bool {
        self.0.borrow_mut().event(shell, theme, event)
    }

    fn draw(
        &mut self,
        shell: &mut crate::Shell<Msg>,
        theme: &Theme,
        renderer: &mut Rend,
    ) {
        self.0.borrow_mut().draw(shell, theme, renderer);
    }
}

impl<W, Rend, Msg, Evt, Theme> From<Variable<W>>
    for Element<Rend, Msg, Evt, Theme>
where
    W: Widget<Rend, Msg, Evt, Theme> + 'static,
{
    fn from(value: Variable<W>) -> Self {
        Self::new(value)
    }
}

impl<W> WidgetExt for Variable<W> {}
