use std::ops::{Deref, DerefMut};

use crate::{VariableAction, VariableOut, VariableSlot};

#[derive(Debug, Default)]
pub struct RelayoutSlot<T>(VariableSlot<T>);

impl<T> RelayoutSlot<T> {
    pub fn update(&mut self) -> bool {
        self.0.update()
    }
}

impl<T> From<T> for RelayoutSlot<T> {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl<T> From<VariableOut<T>> for RelayoutSlot<T> {
    fn from(value: VariableOut<T>) -> Self {
        value.on_change(VariableAction::Relayout);
        Self(value.into())
    }
}

impl<T> From<VariableSlot<T>> for RelayoutSlot<T> {
    fn from(value: VariableSlot<T>) -> Self {
        value.on_change(VariableAction::Relayout);
        Self(value)
    }
}

impl<T> Deref for RelayoutSlot<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for RelayoutSlot<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
