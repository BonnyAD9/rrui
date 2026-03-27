#[derive(Debug, Default)]
pub struct EventCtrl {
    pub ignore: bool,
    pub for_widgets: Option<bool>,
}

impl EventCtrl {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ignore_event(&mut self) {
        self.ignore = true;
    }

    pub fn capture(&mut self) {
        self.for_widgets = Some(false);
    }
}
