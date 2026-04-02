use std::{
    cell::{self, Cell, RefCell},
    rc::Rc,
};

use crate::{ShellProxy, VariableAction};

#[derive(Debug)]
pub struct RefVariableOut<T>(Rc<RefVariableInner<T>>);

#[derive(Debug, Clone)]
pub struct RefVariableIn<T>(Rc<RefVariableInner<T>>);

#[derive(Debug)]
struct RefVariableInner<T> {
    on_change: Cell<VariableAction>,
    changed: Cell<bool>,
    proxy: ShellProxy,
    value: RefCell<T>,
}

pub fn new_ref_variable<T>(
    proxy: ShellProxy,
    value: T,
) -> (RefVariableIn<T>, RefVariableOut<T>) {
    let inner = Rc::new(RefVariableInner {
        on_change: Default::default(),
        changed: Default::default(),
        proxy,
        value: value.into(),
    });
    (RefVariableIn(inner.clone()), RefVariableOut(inner))
}

impl<T> RefVariableOut<T> {
    pub fn update(&self) -> bool {
        self.0.changed.replace(false)
    }

    pub fn on_change(&self, act: VariableAction) {
        self.0.on_change.set(act);
    }

    pub fn borrow(&self) -> cell::Ref<'_, T> {
        self.0.value.borrow()
    }

    pub fn borrow_mut(&self) -> cell::RefMut<'_, T> {
        self.0.value.borrow_mut()
    }
}

impl<T> RefVariableIn<T> {
    pub fn borrow(&self) -> cell::Ref<'_, T> {
        self.0.value.borrow()
    }

    pub fn borrow_mut(&self) -> cell::RefMut<'_, T> {
        self.0.changed.set(true);
        self.0.on_change.get().apply(&self.0.proxy);
        self.0.value.borrow_mut()
    }
}
