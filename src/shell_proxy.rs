use std::{cell::Cell, rc::Rc};

#[derive(Debug, Clone, Default)]
pub struct ShellProxy(pub(crate) Rc<ShellProxyInner>);

#[derive(Debug, Default)]
pub(crate) struct ShellProxyInner {
    pub(crate) redraw: Cell<bool>,
    pub(crate) relayout: Cell<bool>,
}

impl ShellProxy {
    pub fn request_redraw(&self) {
        self.0.redraw.set(true);
    }

    pub fn request_relayout(&self) {
        self.0.relayout.set(true);
    }

    pub fn redraw_requested(&self) -> bool {
        self.0.redraw.get()
    }

    pub fn relayout_requested(&self) -> bool {
        self.0.relayout.get()
    }
}
