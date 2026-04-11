use std::{cell::RefCell, rc::Rc};

use minlin::{Rect, Vec2};

use crate::{
    LayoutBounds, LayoutFlags, LayoutParams, RelPos, Shell, Widget,
    event::EventInfo,
};

pub type CellWidget<Rend, Msg, Evt, Theme> =
    Rc<RefCell<dyn Widget<Rend, Msg, Evt, Theme>>>;

pub struct Element<Rend, Msg, Evt, Theme>(ElementInner<Rend, Msg, Evt, Theme>);

enum ElementInner<Rend, Msg, Evt, Theme> {
    Box(Box<dyn Widget<Rend, Msg, Evt, Theme>>),
    Cell(CellWidget<Rend, Msg, Evt, Theme>),
}

impl<Rend, Msg, Evt, Theme> ElementInner<Rend, Msg, Evt, Theme> {
    fn mutate<T>(
        &mut self,
        f: impl FnOnce(&mut dyn Widget<Rend, Msg, Evt, Theme>) -> T,
    ) -> T {
        match self {
            ElementInner::Box(w) => f(&mut **w),
            ElementInner::Cell(w) => f(&mut *w.borrow_mut()),
        }
    }

    fn focus_mutate<T>(
        &mut self,
        shell: &mut Shell<Rend, Msg, Evt, Theme>,
        f: impl FnOnce(
            &mut dyn Widget<Rend, Msg, Evt, Theme>,
            &mut Shell<Rend, Msg, Evt, Theme>,
        ) -> T,
    ) -> T {
        match self {
            ElementInner::Box(w) => f(&mut **w, shell),
            ElementInner::Cell(w) => {
                shell.with_focus(w.clone(), |s| f(&mut *w.borrow_mut(), s))
            }
        }
    }
}

impl<Rend, Msg, Evt, Theme> Element<Rend, Msg, Evt, Theme> {
    pub fn new(w: impl Widget<Rend, Msg, Evt, Theme> + 'static) -> Self {
        Self(ElementInner::Box(Box::new(w)))
    }

    pub fn from_box(
        w: Box<impl Widget<Rend, Msg, Evt, Theme> + 'static>,
    ) -> Self {
        Self(ElementInner::Box(w))
    }

    pub fn from_cell(
        w: Rc<RefCell<impl Widget<Rend, Msg, Evt, Theme> + 'static>>,
    ) -> Self {
        Self(ElementInner::Cell(w))
    }
}

impl<Rend, Msg, Evt, Theme> Widget<Rend, Msg, Evt, Theme>
    for Element<Rend, Msg, Evt, Theme>
{
    fn layout(
        &mut self,
        lp: &mut LayoutParams<'_, Rend, Msg, Evt, Theme>,
        bounds: &LayoutBounds,
        pos_base: RelPos,
        flags: LayoutFlags,
    ) -> Rect<f32> {
        self.0.mutate(|w| w.layout(lp, bounds, pos_base, flags))
    }

    fn event(
        &mut self,
        shell: &mut Shell<Rend, Msg, Evt, Theme>,
        theme: &Theme,
        event: &EventInfo<Evt>,
    ) -> bool {
        self.0.focus_mutate(shell, |w, s| w.event(s, theme, event))
    }

    fn draw(
        &mut self,
        shell: &mut Shell<Rend, Msg, Evt, Theme>,
        theme: &Theme,
        renderer: &mut Rend,
    ) {
        self.0.mutate(|w| w.draw(shell, theme, renderer));
    }

    fn size(&mut self, theme: &Theme) -> Vec2<f32> {
        self.0.mutate(|w| w.size(theme))
    }

    fn reposition(&mut self, theme: &Theme, pos: Vec2<f32>) {
        self.0.mutate(|w| w.reposition(theme, pos));
    }
}
