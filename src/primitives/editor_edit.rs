#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EditorEdit<'a> {
    Insert(char),
    Paste(&'a str),
    Enter,
    Indent,
    Unindent,
    Backspace,
    Delete,
}
