use std::{cell::RefCell, rc::Rc};

use crate::{Shell, Widget};

pub struct Element<Rend, Msg, Evt>(
    pub Rc<RefCell<dyn Widget<Rend, Msg, Evt>>>,
);

impl<Rend, Msg, Evt> Element<Rend, Msg, Evt> {
    pub fn new(w: impl Widget<Rend, Msg, Evt> + 'static) -> Self {
        Self(Rc::new(RefCell::new(w)))
    }
}

impl<Rend, Msg, Evt> Widget<Rend, Msg, Evt> for Element<Rend, Msg, Evt> {
    fn event(&mut self, event: &Evt, shell: &mut Shell) {
        self.0.borrow_mut().event(event, shell);
    }

    fn draw(&mut self, renderer: &mut Rend, shell: &mut Shell) {
        self.0.borrow_mut().draw(renderer, shell);
    }
}
