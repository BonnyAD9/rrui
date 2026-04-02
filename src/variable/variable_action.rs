use crate::ShellProxy;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VariableAction {
    #[default]
    None,
    Redraw,
    Relayout,
}

impl VariableAction {
    pub fn apply(&self, proxy: &ShellProxy) {
        match self {
            VariableAction::None => {}
            VariableAction::Redraw => proxy.request_redraw(),
            VariableAction::Relayout => proxy.request_relayout(),
        }
    }
}
