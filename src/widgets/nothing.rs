use std::fmt::Debug;

use crate::{Element, Shell, Widget};

pub struct Nothing;

impl<Rend, Msg, Evt: Debug> Widget<Rend, Msg, Evt> for Nothing {
    fn event(&mut self, _: &Evt, _: &mut Shell) {}

    fn draw(&mut self, _: &mut Rend, _: &mut Shell) {}
}

impl<Rend, Msg, Evt: Debug> From<Nothing> for Element<Rend, Msg, Evt> {
    fn from(value: Nothing) -> Self {
        Self::new(value)
    }
}
