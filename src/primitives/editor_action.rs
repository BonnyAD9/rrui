use minlin::Vec2;

use crate::{
    EditorEdit, EditorMotion,
    event::{KeyCode, Modifiers},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EditorAction<'a> {
    Move(EditorMotion),
    Select(EditorMotion),
    SelectWord,
    SelectLine,
    SelectAll,
    Edit(EditorEdit<'a>),
    Click(Vec2<f32>),
    Drag(Vec2<f32>),
    ScrollLines(i32),
}

impl<'a> EditorAction<'a> {
    pub fn from_key(key: KeyCode, mods: Modifiers) -> Option<Self> {
        if let Some(motion) = EditorMotion::from_key(key, mods) {
            if mods.shift() {
                Some(Self::Select(motion))
            } else {
                Some(Self::Move(motion))
            }
        } else {
            EditorEdit::from_key(key, mods).map(Self::Edit)
        }
    }
}
