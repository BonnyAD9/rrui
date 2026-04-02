use std::{
    cell,
    ops::{Deref, DerefMut},
};

pub enum SlotRefMut<'a, T> {
    Value(&'a mut T),
    Ref(cell::RefMut<'a, T>),
}

impl<'a, T> Deref for SlotRefMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Value(v) => v,
            Self::Ref(r) => r,
        }
    }
}

impl<'a, T> DerefMut for SlotRefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Value(v) => v,
            Self::Ref(r) => r,
        }
    }
}
