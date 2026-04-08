use crate::event::MouseButton;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum ButtonEvent {
    #[default]
    Nothing,
    Clicked(MouseButton),
}
