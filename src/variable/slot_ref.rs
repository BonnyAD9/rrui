use std::{cell, ops::Deref};

pub enum SlotRef<'a, T> {
    Value(&'a T),
    Ref(cell::Ref<'a, T>),
}

impl<'a, T> Deref for SlotRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            SlotRef::Value(v) => v,
            SlotRef::Ref(r) => r,
        }
    }
}
