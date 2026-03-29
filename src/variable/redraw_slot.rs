use std::ops::{Deref, DerefMut};

use crate::{VariableAction, VariableOut, VariableSlot};

#[derive(Debug, Default)]
pub struct RedrawSlot<T>(VariableSlot<T>);

impl<T> RedrawSlot<T> {
    pub fn update(&mut self) -> bool {
        self.0.update()
    }
}

impl<T> From<T> for RedrawSlot<T> {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl<T> From<VariableOut<T>> for RedrawSlot<T> {
    fn from(value: VariableOut<T>) -> Self {
        value.on_change(VariableAction::Redraw);
        Self(value.into())
    }
}

impl<T> From<VariableSlot<T>> for RedrawSlot<T> {
    fn from(value: VariableSlot<T>) -> Self {
        value.on_change(VariableAction::Redraw);
        Self(value)
    }
}

impl<T> Deref for RedrawSlot<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for RedrawSlot<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
