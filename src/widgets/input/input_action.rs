use crate::EditorAction;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum InputAction<'a> {
    #[default]
    Default,
    Confirm,
    Ignore,
    Action(EditorAction<'a>),
}
