use crate::{RefVariableOut, SlotRef, SlotRefMut, VariableAction};

#[derive(Debug)]
pub enum RefVariableSlot<T> {
    Value(T),
    Variable(RefVariableOut<T>),
}

impl<T> RefVariableSlot<T> {
    pub fn update(&self) -> bool {
        match self {
            RefVariableSlot::Value(_) => false,
            RefVariableSlot::Variable(v) => v.update(),
        }
    }

    pub fn on_change(&self, act: VariableAction) {
        match self {
            RefVariableSlot::Value(_) => {}
            RefVariableSlot::Variable(v) => v.on_change(act),
        }
    }

    pub fn borrow(&self) -> SlotRef<'_, T> {
        match self {
            RefVariableSlot::Value(v) => SlotRef::Value(v),
            RefVariableSlot::Variable(v) => SlotRef::Ref(v.borrow()),
        }
    }

    pub fn borrow_mut(&mut self) -> SlotRefMut<'_, T> {
        match self {
            RefVariableSlot::Value(v) => SlotRefMut::Value(v),
            RefVariableSlot::Variable(v) => SlotRefMut::Ref(v.borrow_mut()),
        }
    }
}

impl<T: Default> Default for RefVariableSlot<T> {
    fn default() -> Self {
        Self::Value(Default::default())
    }
}

impl<T> From<T> for RefVariableSlot<T> {
    fn from(value: T) -> Self {
        Self::Value(value)
    }
}

impl<T> From<RefVariableOut<T>> for RefVariableSlot<T> {
    fn from(value: RefVariableOut<T>) -> Self {
        Self::Variable(value)
    }
}
