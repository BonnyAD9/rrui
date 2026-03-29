mod redraw_slot;
mod relayout_slot;
mod variable_slot;

pub use self::{redraw_slot::*, relayout_slot::*, variable_slot::*};

use std::{
    cell::Cell,
    fmt::Debug,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use crate::ShellProxy;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VariableAction {
    #[default]
    None,
    Redraw,
    Relayout,
}

#[derive(Debug)]
pub struct VariableOut<T> {
    inner: Rc<VariableInner<T>>,
    value: T,
}

#[derive(Debug, Clone)]
pub struct VariableIn<T>(Rc<VariableInner<T>>);

struct VariableInner<T> {
    on_change: Cell<VariableAction>,
    proxy: ShellProxy,
    value: Cell<Option<T>>,
}

pub fn new_variable<T>(
    proxy: ShellProxy,
    value: T,
) -> (VariableIn<T>, VariableOut<T>) {
    let inner = Rc::new(VariableInner {
        on_change: Default::default(),
        proxy,
        value: Default::default(),
    });
    (VariableIn(inner.clone()), VariableOut { inner, value })
}

impl<T> VariableOut<T> {
    pub fn update(&mut self) -> bool {
        if let Some(v) = self.inner.value.take() {
            self.value = v;
            true
        } else {
            false
        }
    }

    pub fn on_change(&self, act: VariableAction) {
        self.inner.on_change.set(act);
    }
}

impl<T> Deref for VariableOut<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for VariableOut<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T> VariableIn<T> {
    pub fn set(&self, value: impl Into<T>) {
        self.0.value.set(Some(value.into()));
        match self.0.on_change.get() {
            VariableAction::None => {}
            VariableAction::Redraw => self.0.proxy.request_redraw(),
            VariableAction::Relayout => self.0.proxy.request_relayout(),
        }
    }
}

impl<T: Debug> Debug for VariableInner<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = self.value.take();
        let res = f
            .debug_struct("VariableInner")
            .field("on_change", &self.on_change)
            .field("proxy", &self.proxy)
            .field("value", &val)
            .finish();
        self.value.set(val);
        res
    }
}
