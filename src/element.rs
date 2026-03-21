use std::{cell::RefCell, rc::Rc};

use minlin::Rect;

use crate::{LayoutBounds, Shell, Widget};

pub struct Element<Rend, Msg, Evt>(
    pub Rc<RefCell<dyn Widget<Rend, Msg, Evt>>>,
);

impl<Rend, Msg, Evt> Element<Rend, Msg, Evt> {
    pub fn new(w: impl Widget<Rend, Msg, Evt> + 'static) -> Self {
        Self(Rc::new(RefCell::new(w)))
    }
}

impl<Rend, Msg, Evt> Widget<Rend, Msg, Evt> for Element<Rend, Msg, Evt> {
    fn layout(
        &mut self,
        shell: &mut Shell,
        bounds: &LayoutBounds,
    ) -> Rect<f32> {
        self.0.borrow_mut().layout(shell, bounds)
    }

    fn event(&mut self, shell: &mut Shell, event: &Evt) {
        self.0.borrow_mut().event(shell, event);
    }

    fn draw(&mut self, shell: &mut Shell, renderer: &mut Rend) {
        self.0.borrow_mut().draw(shell, renderer);
    }
}
