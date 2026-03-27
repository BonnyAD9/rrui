#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseRelation {
    None,
    Elswhere,
    Hover,
    Move,
    Leave,
    Enter,
}
