use std::ops::{Deref, DerefMut};

use crate::{VariableAction, VariableOut};

#[derive(Debug)]
pub enum VariableSlot<T> {
    Value(T),
    Variable(VariableOut<T>),
}

impl<T> From<T> for VariableSlot<T> {
    fn from(value: T) -> Self {
        Self::Value(value)
    }
}

impl<T> From<VariableOut<T>> for VariableSlot<T> {
    fn from(value: VariableOut<T>) -> Self {
        Self::Variable(value)
    }
}

impl<T> VariableSlot<T> {
    pub fn update(&mut self) -> bool {
        match self {
            VariableSlot::Value(_) => false,
            VariableSlot::Variable(v) => v.update(),
        }
    }

    pub fn on_change(&self, act: VariableAction) {
        match self {
            VariableSlot::Value(_) => {}
            VariableSlot::Variable(v) => v.on_change(act),
        }
    }
}

impl<T> Deref for VariableSlot<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            VariableSlot::Value(v) => v,
            VariableSlot::Variable(v) => v,
        }
    }
}

impl<T> DerefMut for VariableSlot<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            VariableSlot::Value(v) => v,
            VariableSlot::Variable(v) => v,
        }
    }
}

impl<T: Default> Default for VariableSlot<T> {
    fn default() -> Self {
        Self::Value(Default::default())
    }
}
