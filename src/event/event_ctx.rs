use crate::event::EventTarget;

#[derive(Debug, Default)]
pub struct EventCtrl {
    pub ignore: bool,
    pub target: Option<EventTarget>,
}

impl EventCtrl {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ignore_event(&mut self) {
        self.ignore = true;
    }

    pub fn capture(&mut self) {
        self.target = Some(EventTarget::Nothing);
    }
}
