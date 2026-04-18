use minlin::Vec2;

use crate::{EditorEdit, EditorMotion};

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
