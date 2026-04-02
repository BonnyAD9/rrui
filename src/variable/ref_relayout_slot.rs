use crate::{
    RefVariableOut, RefVariableSlot, SlotRef, SlotRefMut, VariableAction,
};

#[derive(Debug, Default)]
pub struct RefRelayoutSlot<T>(RefVariableSlot<T>);

impl<T> RefRelayoutSlot<T> {
    pub fn update(&self) -> bool {
        self.0.update()
    }

    pub fn borrow(&self) -> SlotRef<'_, T> {
        self.0.borrow()
    }

    pub fn borrow_mut(&mut self) -> SlotRefMut<'_, T> {
        self.0.borrow_mut()
    }
}

impl<T> From<T> for RefRelayoutSlot<T> {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl<T> From<RefVariableOut<T>> for RefRelayoutSlot<T> {
    fn from(value: RefVariableOut<T>) -> Self {
        value.on_change(VariableAction::Relayout);
        Self(value.into())
    }
}

impl<T> From<RefVariableSlot<T>> for RefRelayoutSlot<T> {
    fn from(value: RefVariableSlot<T>) -> Self {
        value.on_change(VariableAction::Relayout);
        Self(value)
    }
}
